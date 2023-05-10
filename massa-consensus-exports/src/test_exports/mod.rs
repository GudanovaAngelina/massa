// Copyright (c) 2022 MASSA LABS <info@massa.net>

mod config;
mod mock;

pub(crate) use config::*;
pub use mock::{
    ConsensusControllerImpl, ConsensusEventReceiver, MockConsensusControllerImpl,
    MockConsensusControllerMessage,
};
