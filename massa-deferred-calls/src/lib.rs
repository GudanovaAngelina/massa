use call::{DeferredCallDeserializer, DeferredCallSerializer};
use massa_db_exports::{
    DBBatch, ShareableMassaDBController, DEFERRED_CALLS_PREFIX, KEY_SER_ERROR, MESSAGE_DESER_ERROR,
    MESSAGE_SER_ERROR, STATE_CF,
};
use massa_serialization::{DeserializeError, Deserializer, Serializer};
use registry_changes::DeferredRegistryChanges;

/// This module implements a new version of the Autonomous Smart Contracts. (ASC)
/// This new version allow asynchronous calls to be registered for a specific slot and ensure his execution.
mod call;
pub mod registry_changes;
pub mod slot_changes;

#[macro_use]
mod macros;

pub use call::DeferredCall;
use massa_ledger_exports::{SetOrDelete, SetOrKeep};
use massa_models::{
    amount::Amount,
    config::THREAD_COUNT,
    deferred_call_id::{DeferredCallId, DeferredCallIdDeserializer, DeferredCallIdSerializer},
    slot::Slot,
};
use std::collections::BTreeMap;

// #[derive(Debug)]
pub struct DeferredCallRegistry {
    db: ShareableMassaDBController,
    deferred_call_serializer: DeferredCallSerializer,
    deferred_call_id_serializer: DeferredCallIdSerializer,
    deferred_call_deserializer: DeferredCallDeserializer,
}

impl DeferredCallRegistry {
    /*
     DB layout:
        [ASYNC_CALL_TOTAL_GAS_PREFIX] -> u64 // total currently booked gas
        [ASYNC_CAL_SLOT_PREFIX][slot][TOTAL_GAS_TAG] -> u64 // total gas booked for a slot (optional, default 0, deleted when set to 0)
        [ASYNC_CAL_SLOT_PREFIX][slot][CALLS_TAG][id][ASYNC_CALL_FIELD_X_TAG] -> AsyncCall.x // call data
    */

    pub fn new(db: ShareableMassaDBController) -> Self {
        Self {
            db,
            deferred_call_serializer: DeferredCallSerializer::new(),
            deferred_call_id_serializer: DeferredCallIdSerializer::new(),
            deferred_call_deserializer: DeferredCallDeserializer::new(THREAD_COUNT),
        }
    }

    pub fn get_slot_calls(&self, slot: Slot) -> DeferredSlotCalls {
        todo!()
    }

    /// Returns the DeferredCall for a given slot and id
    pub fn get_call(&self, slot: &Slot, id: &DeferredCallId) -> Option<DeferredCall> {
        let mut buf_id = Vec::new();
        self.deferred_call_id_serializer
            .serialize(id, &mut buf_id)
            .expect(MESSAGE_SER_ERROR);
        let key = deferred_call_prefix_key!(buf_id, slot.to_bytes_key());

        let mut serialized_call: Vec<u8> = Vec::new();
        for (serialized_key, serialized_value) in self.db.read().prefix_iterator_cf(STATE_CF, &key)
        {
            if !serialized_key.starts_with(&key) {
                break;
            }

            serialized_call.extend(serialized_value.iter());
        }

        match self
            .deferred_call_deserializer
            .deserialize::<DeserializeError>(&serialized_call)
        {
            Ok((_rest, call)) => Some(call),
            Err(_) => None,
        }
    }

    /// Returns the total amount of gas booked for a slot
    pub fn get_slot_gas(&self, slot: Slot) -> u64 {
        // By default, if it is absent, it is 0
        let key = deferred_call_slot_total_gas_key!(slot.to_bytes_key());
        match self.db.read().get_cf(STATE_CF, key) {
            Ok(Some(v)) => {
                let result = self
                    .deferred_call_deserializer
                    .u64_var_int_deserializer
                    .deserialize::<DeserializeError>(&v)
                    .expect(MESSAGE_DESER_ERROR)
                    .1;
                result
            }
            _ => 0,
        }
    }

    /// Returns the base fee for a slot
    pub fn get_slot_base_fee(&self, slot: &Slot) -> Amount {
        unimplemented!("get_slot_base_fee");
        // self.db.read().get_cf(handle_cf, key)
        // // By default, if it is absent, it is 0
        // self.db.read().get
        //     .get(ASYNC_CAL_SLOT_PREFIX, slot, BASE_FEE_TAG)
        //     .map(|v| Amount::from_bytes(v).unwrap())
        //     .unwrap_or(Amount::zero())
    }

    /// Returns the total amount of gas booked
    pub fn get_total_gas() -> u128 {
        todo!()
    }

    pub fn put_entry(
        &self,
        slot: &Slot,
        call_id: &DeferredCallId,
        call: &DeferredCall,
        batch: &mut DBBatch,
    ) {
        let mut buffer_id = Vec::new();
        self.deferred_call_id_serializer
            .serialize(call_id, &mut buffer_id)
            .expect(MESSAGE_SER_ERROR);

        let slot_bytes = slot.to_bytes_key();

        let db = self.db.read();

        // sender address
        let mut temp_buffer = Vec::new();
        self.deferred_call_serializer
            .address_serializer
            .serialize(&call.sender_address, &mut temp_buffer)
            .expect(MESSAGE_SER_ERROR);
        db.put_or_update_entry_value(
            batch,
            sender_address_key!(buffer_id, slot_bytes),
            &temp_buffer,
        );
        temp_buffer.clear();

        // target slot
        self.deferred_call_serializer
            .slot_serializer
            .serialize(&call.target_slot, &mut temp_buffer)
            .expect(MESSAGE_SER_ERROR);
        db.put_or_update_entry_value(batch, target_slot_key!(buffer_id, slot_bytes), &temp_buffer);
        temp_buffer.clear();

        // target address
        self.deferred_call_serializer
            .address_serializer
            .serialize(&call.target_address, &mut temp_buffer)
            .expect(MESSAGE_SER_ERROR);
        db.put_or_update_entry_value(
            batch,
            target_address_key!(buffer_id, slot_bytes),
            &temp_buffer,
        );
        temp_buffer.clear();

        // target function
        self.deferred_call_serializer
            .string_serializer
            .serialize(&call.target_function, &mut temp_buffer)
            .expect(MESSAGE_SER_ERROR);
        db.put_or_update_entry_value(
            batch,
            target_function_key!(buffer_id, slot_bytes),
            &temp_buffer,
        );
        temp_buffer.clear();

        // parameters
        self.deferred_call_serializer
            .vec_u8_serializer
            .serialize(&call.parameters, &mut temp_buffer)
            .expect(MESSAGE_SER_ERROR);
        db.put_or_update_entry_value(batch, parameters_key!(buffer_id, slot_bytes), &temp_buffer);
        temp_buffer.clear();

        // coins
        self.deferred_call_serializer
            .amount_serializer
            .serialize(&call.coins, &mut temp_buffer)
            .expect(MESSAGE_SER_ERROR);
        db.put_or_update_entry_value(batch, coins_key!(buffer_id, slot_bytes), &temp_buffer);
        temp_buffer.clear();

        // max gas
        self.deferred_call_serializer
            .u64_var_int_serializer
            .serialize(&call.max_gas, &mut temp_buffer)
            .expect(MESSAGE_SER_ERROR);
        db.put_or_update_entry_value(batch, max_gas_key!(buffer_id, slot_bytes), &temp_buffer);
        temp_buffer.clear();

        // fee
        self.deferred_call_serializer
            .amount_serializer
            .serialize(&call.fee, &mut temp_buffer)
            .expect(MESSAGE_SER_ERROR);
        db.put_or_update_entry_value(batch, fee_key!(buffer_id, slot_bytes), &temp_buffer);
        temp_buffer.clear();

        // cancelled
        self.deferred_call_serializer
            .bool_serializer
            .serialize(&call.cancelled, &mut temp_buffer)
            .expect(MESSAGE_SER_ERROR);
        db.put_or_update_entry_value(batch, cancelled_key!(buffer_id, slot_bytes), &temp_buffer);
    }

    fn delete_entry(&self, id: &DeferredCallId, slot: &Slot, batch: &mut DBBatch) {
        let mut buffer_id = Vec::new();
        self.deferred_call_id_serializer
            .serialize(id, &mut buffer_id)
            .expect(MESSAGE_SER_ERROR);

        let slot_bytes = slot.to_bytes_key();

        let db = self.db.read();

        db.delete_key(batch, sender_address_key!(buffer_id, slot_bytes));
        db.delete_key(batch, target_slot_key!(buffer_id, slot_bytes));
        db.delete_key(batch, target_address_key!(buffer_id, slot_bytes));
        db.delete_key(batch, target_function_key!(buffer_id, slot_bytes));
        db.delete_key(batch, parameters_key!(buffer_id, slot_bytes));
        db.delete_key(batch, coins_key!(buffer_id, slot_bytes));
        db.delete_key(batch, max_gas_key!(buffer_id, slot_bytes));
        db.delete_key(batch, fee_key!(buffer_id, slot_bytes));
        db.delete_key(batch, cancelled_key!(buffer_id, slot_bytes));
    }

    pub fn apply_changes_to_batch(&self, changes: DeferredRegistryChanges, batch: &mut DBBatch) {
        //Note: if a slot gas is zet to 0, delete the slot gas entry
        // same for base fee

        for change in changes.slots_change.iter() {
            let slot = change.0;
            let slot_changes = change.1;
            for (id, call_change) in slot_changes.calls.iter() {
                match call_change {
                    DeferredRegistryCallChange::Set(call) => {
                        self.put_entry(slot, id, call, batch);
                    }
                    DeferredRegistryCallChange::Delete => {
                        self.delete_entry(id, slot, batch);
                    }
                }
            }
            match slot_changes.gas {
                DeferredRegistryGasChange::Set(v) => {
                    let key = deferred_call_slot_total_gas_key!(slot.to_bytes_key());
                    //Note: if a slot gas is zet to 0, delete the slot gas entry
                    if v.eq(&0) {
                        self.db.read().delete_key(batch, key);
                    } else {
                        let mut value_ser = Vec::new();
                        self.deferred_call_serializer
                            .u64_var_int_serializer
                            .serialize(&v, &mut value_ser)
                            .expect(MESSAGE_SER_ERROR);
                        self.db
                            .read()
                            .put_or_update_entry_value(batch, key, &value_ser);
                    }
                }
                DeferredRegistryGasChange::Keep => {}
            }
            // match slot_changes.base_fee {
            //     DeferredRegistryBaseFeeChange::Set(v) => {
            //         if v.eq(&0) {
            //             batch.delete_key(ASYNC_CAL_SLOT_PREFIX, slot, BASE_FEE_TAG);
            //         } else {
            //             let key = BASE_FEE_TAG;
            //             let value = v.to_bytes().to_vec();
            //             batch.put_or_update_entry_value(ASYNC_CAL_SLOT_PREFIX, slot, key, value);
            //         }
            //     }
            //     DeferredRegistryBaseFeeChange::Keep => {}
            // }
        }
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum DeferredRegistryCallChange {
//     Set(DeferredCall),
//     Delete,
// }

// todo put SetOrDelete dans models
pub type DeferredRegistryCallChange = SetOrDelete<DeferredCall>;
pub type DeferredRegistryGasChange<V> = SetOrKeep<V>;
pub type DeferredRegistryBaseFeeChange = SetOrKeep<Amount>;

// impl DeferredRegistryCallChange {
//     pub fn merge(&mut self, other: DeferredRegistryCallChange) {
//         *self = other;
//     }

//     pub fn delete_call(&mut self) {
//         *self = DeferredRegistryCallChange::Delete;
//     }

//     pub fn set_call(&mut self, call: DeferredCall) {
//         *self = DeferredRegistryCallChange::Set(call);
//     }

//     pub fn get_call(&self) -> Option<&DeferredCall> {
//         match self {
//             DeferredRegistryCallChange::Set(v) => Some(v),
//             DeferredRegistryCallChange::Delete => None,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum DeferredRegistryGasChange<V> {
//     Set(V),
//     Keep,
// }

// impl<V> Default for DeferredRegistryGasChange<V> {
//     fn default() -> Self {
//         DeferredRegistryGasChange::Keep
//     }
// }

/// A structure that lists slot calls for a given slot,
/// as well as global gas usage statistics.
#[derive(Debug, Clone)]
pub struct DeferredSlotCalls {
    pub slot: Slot,
    pub slot_calls: BTreeMap<DeferredCallId, DeferredCall>,
    pub slot_gas: u64,
    pub slot_base_fee: Amount,
    pub total_gas: u128,
}

impl DeferredSlotCalls {
    pub fn new(slot: Slot) -> Self {
        Self {
            slot,
            slot_calls: BTreeMap::new(),
            slot_gas: 0,
            slot_base_fee: Amount::zero(),
            total_gas: 0,
        }
    }

    pub fn apply_changes(&mut self, changes: &DeferredRegistryChanges) {
        let Some(slot_changes) = changes.slots_change.get(&self.slot) else {
            return;
        };
        for (id, change) in &slot_changes.calls {
            match change {
                DeferredRegistryCallChange::Set(call) => {
                    self.slot_calls.insert(id.clone(), call.clone());
                }
                DeferredRegistryCallChange::Delete => {
                    self.slot_calls.remove(id);
                }
            }
        }
        match slot_changes.gas {
            DeferredRegistryGasChange::Set(v) => self.slot_gas = v,
            DeferredRegistryGasChange::Keep => {}
        }
        match slot_changes.base_fee {
            DeferredRegistryGasChange::Set(v) => self.slot_base_fee = v,
            DeferredRegistryGasChange::Keep => {}
        }
        match changes.total_gas {
            DeferredRegistryGasChange::Set(v) => self.total_gas = v,
            DeferredRegistryGasChange::Keep => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeferredCall, DeferredCallRegistry, DeferredRegistryChanges};
    use massa_db_exports::{
        DBBatch, Key, MassaDBConfig, MassaDBController, ShareableMassaDBController,
        DEFERRED_CALLS_PREFIX, STATE_CF,
    };
    use massa_db_worker::MassaDB;
    use massa_models::{
        address::Address,
        amount::Amount,
        config::THREAD_COUNT,
        deferred_call_id::{DeferredCallId, DeferredCallIdSerializer},
        slot::Slot,
    };
    use parking_lot::RwLock;
    use std::{str::FromStr, sync::Arc};
    use tempfile::tempdir;

    #[test]
    fn apply_changes() {
        let temp_dir = tempdir().expect("Unable to create a temp folder");
        let db_config = MassaDBConfig {
            path: temp_dir.path().to_path_buf(),
            max_history_length: 100,
            max_final_state_elements_size: 100,
            max_versioning_elements_size: 100,
            thread_count: THREAD_COUNT,
            max_ledger_backups: 100,
        };
        let call_id_serializer = DeferredCallIdSerializer::new();
        let db: ShareableMassaDBController = Arc::new(RwLock::new(
            Box::new(MassaDB::new(db_config)) as Box<(dyn MassaDBController + 'static)>,
        ));

        let registry = DeferredCallRegistry::new(db);

        let mut changes = DeferredRegistryChanges::default();

        let target_slot = Slot {
            thread: 5,
            period: 1,
        };

        let call = DeferredCall::new(
            Address::from_str("AU12dG5xP1RDEB5ocdHkymNVvvSJmUL9BgHwCksDowqmGWxfpm93x").unwrap(),
            target_slot.clone(),
            Address::from_str("AS127QtY6Hzm6BnJc9wqCBfPNvEH9fKer3LiMNNQmcX3MzLwCL6G6").unwrap(),
            "receive".to_string(),
            vec![42, 42, 42, 42],
            Amount::from_raw(100),
            3000000,
            Amount::from_raw(1),
            false,
        );
        let id = DeferredCallId::new(0, target_slot.clone(), 1, &[]).unwrap();
        let mut buf_id = Vec::new();
        call_id_serializer.serialize(&id, &mut buf_id).unwrap();

        changes.set_call(id.clone(), call.clone());

        let mut batch = DBBatch::new();
        registry.apply_changes_to_batch(changes, &mut batch);

        registry.db.write().write_batch(batch, DBBatch::new(), None);

        let result = registry.get_call(&target_slot, &id).unwrap();
        assert!(result.target_function.eq(&call.target_function));
        assert_eq!(result.sender_address, call.sender_address);
    }
}
