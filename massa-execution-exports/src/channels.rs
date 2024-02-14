// Copyright (c) 2023 MASSA LABS <info@massa.net>

use crate::types::{SlotExecutionOperationTraces, SlotExecutionOutput};

/// channels used by the execution worker
#[derive(Clone)]
pub struct ExecutionChannels {
    /// Broadcast channel for new slot execution outputs
    pub slot_execution_output_sender: tokio::sync::broadcast::Sender<SlotExecutionOutput>,
    /// Broadcast channel for execution traces
    pub slot_execution_traces_sender: tokio::sync::broadcast::Sender<SlotExecutionOperationTraces>,
}
