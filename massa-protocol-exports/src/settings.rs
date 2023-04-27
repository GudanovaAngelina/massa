// Copyright (c) 2022 MASSA LABS <info@massa.net>

use std::{collections::HashMap, net::{SocketAddr, IpAddr}, path::PathBuf};

use massa_time::MassaTime;
use peernet::transports::TransportType;
use serde::Deserialize;

/// Dynamic protocol configuration mix in static settings and constants configurations.
#[derive(Debug, Deserialize, Clone)]
pub struct ProtocolConfig {
    /// self keypair
    pub keypair_file: PathBuf,
    /// listeners from where we can receive messages
    pub listeners: HashMap<SocketAddr, TransportType>,
    /// initial peers path
    pub initial_peers: PathBuf,
    /// max number of in connections
    pub max_in_connections: usize,
    /// max number of out connections
    pub max_out_connections: usize,
    /// after `ask_block_timeout` milliseconds we try to ask a block to another node
    pub ask_block_timeout: MassaTime,
    /// Max known blocks we keep in block_handler
    pub max_known_blocks_saved_size: usize,
    /// max known blocks of current nodes we keep in memory
    pub max_known_blocks_size: usize,
    /// max known blocks of foreign nodes we keep in memory (by node)
    pub max_node_known_blocks_size: usize,
    /// max wanted blocks per node kept in memory
    pub max_node_wanted_blocks_size: usize,
    /// max known operations current node kept in memory
    pub max_known_ops_size: usize,
    /// max known operations of foreign nodes we keep in memory (by node)
    pub max_node_known_ops_size: usize,
    /// max known endorsements by our node that we kept in memory
    pub max_known_endorsements_size: usize,
    /// max known endorsements of foreign nodes we keep in memory (by node)
    pub max_node_known_endorsements_size: usize,
    /// we ask for the same block `max_simultaneous_ask_blocks_per_node` times at the same time
    pub max_simultaneous_ask_blocks_per_node: usize,
    /// Max wait time for sending a Network or Node event.
    pub max_send_wait: MassaTime,
    /// Maximum number of batches in the memory buffer.
    /// Dismiss the new batches if overflow
    pub operation_batch_buffer_capacity: usize,
    /// Maximum number of operations in the announcement buffer.
    /// Immediately announce if overflow.
    pub operation_announcement_buffer_capacity: usize,
    /// Start processing batches in the buffer each `operation_batch_proc_period` in millisecond
    pub operation_batch_proc_period: MassaTime,
    /// Maximum number of asked operations in the memory buffer.
    pub asked_operations_buffer_capacity: usize,
    /// All operations asked are prune each `operation_asked_pruning_period` millisecond
    pub asked_operations_pruning_period: MassaTime,
    /// Interval at which operations are announced in batches.
    pub operation_announcement_interval: MassaTime,
    /// Maximum time we keep an operation in the storage
    pub max_operation_storage_time: MassaTime,
    /// Maximum of operations sent in one message.
    pub max_operations_per_message: u64,
    /// Maximum of operations sent in one block.
    pub max_operations_per_block: u32,
    /// Maximum size in bytes of all serialized operations size in a block
    pub max_serialized_operations_size_per_block: usize,
    /// Controller channel size
    pub controller_channel_size: usize,
    /// Event channel size
    pub event_channel_size: usize,
    /// t0
    pub t0: MassaTime,
    /// Genesis timestamp
    pub genesis_timestamp: MassaTime,
    /// max time we propagate operations
    pub max_operations_propagation_time: MassaTime,
    /// max time we propagate endorsements
    pub max_endorsements_propagation_time: MassaTime,
    /// number of thread tester
    pub thread_tester_count: u8,
    /// Max size of the channel for command to the connectivity thread
    pub max_size_channel_commands_connectivity: usize,
    /// Max size of channel to send commands to retrieval thread of operations
    pub max_size_channel_commands_retrieval_operations: usize,
    /// Max size of channel to send commands to propagation thread of operations
    pub max_size_channel_commands_propagation_operations: usize,
    /// Max size of channel to send commands to retrieval thread of endorsements
    pub max_size_channel_commands_retrieval_endorsements: usize,
    /// Max size of channel to send commands to propagation thread of blocks
    pub max_size_channel_commands_propagation_endorsements: usize,
    /// Max size of channel to send commands to retrieval thread of blocks
    pub max_size_channel_commands_retrieval_blocks: usize,
    /// Max size of channel to send commands to propagation thread of blocks
    pub max_size_channel_commands_propagation_blocks: usize,
    /// Max size of channel to send commands to thread of peers
    pub max_size_channel_commands_peers: usize,
    /// Max size of channel to send commands to the peer testers
    pub max_size_channel_commands_peer_testers: usize,
    /// Max size of channel that transfer message from network to operation handler
    pub max_size_channel_network_to_operation_handler: usize,
    /// Max size of channel that transfer message from network to block handler
    pub max_size_channel_network_to_block_handler: usize,
    /// Max size of channel that transfer message from network to endorsement handler
    pub max_size_channel_network_to_endorsement_handler: usize,
    /// Max size of channel that transfer message from network to peer handler
    pub max_size_channel_network_to_peer_handler: usize,
    /// endorsements per block
    pub endorsement_count: u32,
    /// running threads count
    pub thread_count: u8,
    /// Max of block infos you can send
    pub max_size_block_infos: u64,
    /// Maximum size of an value user datastore
    pub max_size_value_datastore: u64,
    /// Maximum size of a function name
    pub max_size_function_name: u16,
    /// Maximum size of a parameter of a call in ops
    pub max_size_call_sc_parameter: u32,
    // Maximum size of an op datastore in ops
    pub max_op_datastore_entry_count: u64,
    // Maximum size of a key the in op datastore in ops
    pub max_op_datastore_key_length: u8,
    // Maximum size of a value in the op datastore in ops
    pub max_op_datastore_value_length: u64,
    /// Maximum number of denunciations in a block header
    pub max_denunciations_in_block_header: u32,
    /// Maximum number of endorsements that can be propagated in one message
    pub max_endorsements_per_message: u64,
    /// Maximum number of peers per announcement
    pub max_size_peers_announcement: u64,
    /// Maximum number of listeners per peer
    pub max_size_listeners_per_peer: u64,
    /// Last start period
    pub last_start_period: u64,
    /// Number of bytes per second that can be read/write in a connection (should be a 10 multiplier)
    pub read_write_limit_bytes_per_second: u128,
    /// Optional routable ip
    pub routable_ip: Option<IpAddr>,
    /// debug prints
    pub debug: bool,
}
