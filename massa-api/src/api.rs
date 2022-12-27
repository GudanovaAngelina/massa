//! Copyright (c) 2022 MASSA LABS <info@massa.net>
//! Json RPC API for a massa-node
use std::net::SocketAddr;

use crate::api_trait::MassaApiServer;
use crate::{APIConfig, ApiServer, ApiV2, StopHandle, API};
use async_trait::async_trait;
use jsonrpsee::core::{Error as JsonRpseeError, RpcResult};
use jsonrpsee::types::SubscriptionResult;
use jsonrpsee::SubscriptionSink;
use massa_consensus_exports::ConsensusController;
use massa_models::version::Version;
use massa_protocol_exports::ProtocolCommandSender;

impl API<ApiV2> {
    /// generate a new massa API
    pub fn new(
        consensus_controller: Box<dyn ConsensusController>,
        protocol_command_sender: ProtocolCommandSender,
        api_settings: APIConfig,
        version: Version,
    ) -> Self {
        API(ApiV2 {
            consensus_controller,
            protocol_command_sender,
            api_settings,
            version,
        })
    }
}

#[async_trait]
impl ApiServer for API<ApiV2> {
    async fn serve(
        self,
        url: &SocketAddr,
        api_config: &APIConfig,
    ) -> Result<StopHandle, JsonRpseeError> {
        crate::serve(self.into_rpc(), url, api_config).await
    }
}

#[doc(hidden)]
#[async_trait]
impl MassaApiServer for API<ApiV2> {
    async fn get_version(&self) -> RpcResult<Version> {
        Ok(self.0.version)
    }

    fn subscribe_new_blocks(&self, sink: SubscriptionSink) -> SubscriptionResult {
        let consensus_controller = self.0.consensus_controller.clone();
        consensus_controller.subscribe_new_blocks(sink);
        Ok(())
    }

    fn subscribe_new_blocks_headers(&self, sink: SubscriptionSink) -> SubscriptionResult {
        let consensus_controller = self.0.consensus_controller.clone();
        consensus_controller.subscribe_new_blocks_headers(sink);
        Ok(())
    }

    fn subscribe_new_filled_blocks(&self, sink: SubscriptionSink) -> SubscriptionResult {
        let consensus_controller = self.0.consensus_controller.clone();
        consensus_controller.subscribe_new_filled_blocks(sink);
        Ok(())
    }

    fn subscribe_new_operations(&self, sink: SubscriptionSink) -> SubscriptionResult {
        let protocol_command_sender = self.0.protocol_command_sender.clone();
        tokio::task::spawn_blocking(move || protocol_command_sender.subscribe_new_operations(sink));
        Ok(())
    }
}
