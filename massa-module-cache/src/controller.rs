use massa_hash::Hash;
use massa_models::prehash::BuildHashMapper;
use massa_sc_runtime::{Compiler, RuntimeModule};
use schnellru::{ByLength, LruMap};
use tracing::debug;

use crate::{
    config::ModuleCacheConfig, error::CacheError, hd_cache::HDCache, lru_cache::LRUCache,
    types::ModuleInfo,
};

/// `LruMap` specialization for `PreHashed` keys
pub type PreHashLruMap<K, V> = LruMap<K, V, ByLength, BuildHashMapper<K>>;

/// Cache controller of compiled runtime modules
pub struct ModuleCache {
    /// Cache config.
    /// See `CacheConfig` documentation for more information.
    cfg: ModuleCacheConfig,
    /// RAM stored LRU cache.
    /// See `LRUCache` documentation for more information.
    lru_cache: LRUCache,
    /// Disk stored cache.
    /// See the `HDCache` documentation for more information.
    hd_cache: HDCache,
}

impl ModuleCache {
    /// Creates a new `ModuleCache`
    pub fn new(cfg: ModuleCacheConfig) -> Self {
        Self {
            lru_cache: LRUCache::new(cfg.lru_cache_size),
            hd_cache: HDCache::new(
                cfg.hd_cache_path.clone(),
                cfg.hd_cache_size,
                cfg.snip_amount,
            ),
            cfg,
        }
    }

    /// Internal function to compile and build `ModuleInfo`
    fn compile_cached(&mut self, bytecode: &[u8], hash: Hash) -> ModuleInfo {
        match RuntimeModule::new(bytecode, self.cfg.gas_costs.clone(), Compiler::CL) {
            Ok(module) => {
                debug!("compilation of module {} succeeded", hash);
                ModuleInfo::Module(module)
            }
            Err(e) => {
                let err_msg = format!("compilation of module {} failed: {}", hash, e);
                debug!(err_msg);
                ModuleInfo::Invalid(err_msg)
            }
        }
    }

    /// Save a new or an already existing module in the cache
    pub fn save_module(&mut self, bytecode: &[u8]) {
        let hash = Hash::compute_from(bytecode);
        if let Some(hd_module_info) = self.hd_cache.get(hash, self.cfg.gas_costs.clone()) {
            debug!("save_module: {} present in hd", hash);
            self.lru_cache.insert(hash, hd_module_info);
        } else if let Some(lru_module_info) = self.lru_cache.get(hash) {
            debug!("save_module: {} missing in hd but present in lru", hash);
            self.hd_cache.insert(hash, lru_module_info);
        } else {
            debug!("save_module: {} missing", hash);
            let module_info = self.compile_cached(bytecode, hash);
            self.hd_cache.insert(hash, module_info.clone());
            self.lru_cache.insert(hash, module_info);
        }
    }

    /// Set the initialization cost of a cached module
    pub fn set_init_cost(&mut self, bytecode: &[u8], init_cost: u64) {
        let hash = Hash::compute_from(bytecode);
        self.lru_cache.set_init_cost(hash, init_cost);
        self.hd_cache.set_init_cost(hash, init_cost);
    }

    /// Set a cached module as invalid
    pub fn set_invalid(&mut self, bytecode: &[u8], err_msg: String) {
        let hash = Hash::compute_from(bytecode);
        self.lru_cache.set_invalid(hash, err_msg.clone());
        self.hd_cache.set_invalid(hash, err_msg);
    }

    /// Load a cached module for execution
    ///
    /// Returns the module information, it can be:
    /// * `ModuleInfo::Invalid` if the module is invalid
    /// * `ModuleInfo::Module` if the module is valid and has no delta
    /// * `ModuleInfo::ModuleAndDelta` if the module is valid and has a delta
    fn load_module_info(&mut self, bytecode: &[u8]) -> ModuleInfo {
        if bytecode.is_empty() {
            let error_msg = "load_module: bytecode is absent".to_string();
            debug!(error_msg);
            return ModuleInfo::Invalid(error_msg);
        }
        if bytecode.len() > self.cfg.max_module_length as usize {
            let error_msg = format!(
                "load_module: bytecode length {} exceeds max module length {}",
                bytecode.len(),
                self.cfg.max_module_length
            );
            debug!(error_msg);
            return ModuleInfo::Invalid(error_msg);
        }
        let hash = Hash::compute_from(bytecode);
        if let Some(lru_module_info) = self.lru_cache.get(hash) {
            debug!("load_module: {} present in lru", hash);
            lru_module_info
        } else if let Some(hd_module_info) = self.hd_cache.get(hash, self.cfg.gas_costs.clone()) {
            debug!("load_module: {} missing in lru but present in hd", hash);
            self.lru_cache.insert(hash, hd_module_info.clone());
            hd_module_info
        } else {
            debug!("load_module: {} missing", hash);
            let module_info = self.compile_cached(bytecode, hash);
            self.hd_cache.insert(hash, module_info.clone());
            self.lru_cache.insert(hash, module_info.clone());
            module_info
        }
    }

    /// Load a cached module for execution and check its validity for execution.
    /// Also checks that the provided execution gas is enough to pay for the instance creation cost.
    ///
    /// Returns the module and the remaining gas after loading.
    pub fn load_module(
        &mut self,
        bytecode: &[u8],
        execution_gas: u64,
    ) -> Result<(RuntimeModule, u64), CacheError> {
        // Do not actually debit the instance creation cost from the provided gas
        // This is only supposed to be a check
        execution_gas
            .checked_sub(self.cfg.gas_costs.max_instance_cost)
            .ok_or(CacheError::LoadError(
                "Provided max gas is below the default instance creation cost".to_string(),
            ))?;
        // TODO: interesting but unimportant optim
        // remove max_instance_cost hard check if module is cached and has a delta
        let module_info = self.load_module_info(bytecode);
        let module = match module_info {
            ModuleInfo::Invalid(err) => {
                let err_msg = format!("invalid module: {}", err);
                return Err(CacheError::LoadError(err_msg));
            }
            ModuleInfo::Module(module) => module,
            ModuleInfo::ModuleAndDelta((module, delta)) => {
                if delta > execution_gas {
                    return Err(CacheError::LoadError(format!(
                        "Provided gas {} is below the gas cost of instance creation ({})",
                        execution_gas, delta
                    )));
                } else {
                    module
                }
            }
        };
        Ok((module, execution_gas))
    }

    /// Load a temporary module from arbitrary bytecode.
    /// Also checks that the provided execution gas is enough to pay for the instance creation cost.
    ///
    /// Returns the module and the remaining gas after compilation.
    pub fn load_tmp_module(
        &self,
        bytecode: &[u8],
        limit: u64,
    ) -> Result<(RuntimeModule, u64), CacheError> {
        debug!("load_tmp_module");
        // Do not actually debit the instance creation cost from the provided gas
        // This is only supposed to be a check
        limit
            .checked_sub(self.cfg.gas_costs.max_instance_cost)
            .ok_or(CacheError::LoadError(
                "Provided max gas is below the default instance creation cost".to_string(),
            ))?;
        let module = RuntimeModule::new(bytecode, self.cfg.gas_costs.clone(), Compiler::SP)?;
        Ok((module, limit))
    }
}
