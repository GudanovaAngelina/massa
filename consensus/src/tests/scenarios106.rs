// Copyright (c) 2021 MASSA LABS <info@massa.net>

// RUST_BACKTRACE=1 cargo test scenarios106 -- --nocapture

use crate::tests::tools::{self, generate_ledger_file};
use models::timeslots;
use models::{BlockHashSet, BlockId, Slot};
use serial_test::serial;
use signature::{generate_random_private_key, PrivateKey};
use std::collections::{HashMap, HashSet};
use time::UTime;

#[tokio::test]
#[serial]
async fn test_unsorted_block() {
    /*stderrlog::new()
    .verbosity(4)
    .timestamp(stderrlog::Timestamp::Millisecond)
    .init()
    .unwrap();*/
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<PrivateKey> = (0..1).map(|_| generate_random_private_key()).collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.future_block_processing_max_periods = 50;
    cfg.max_future_processing_blocks = 10;

    tools::consensus_without_pool_test(
        cfg.clone(),
        async move |mut protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let start_period = 3;
            let genesis_hashes = consensus_command_sender
                .get_block_graph_status(None, None)
                .await
                .expect("could not get block graph status")
                .genesis_blocks;
            // create test blocks

            let (hasht0s1, t0s1, _) = tools::create_block(
                &cfg,
                Slot::new(1 + start_period, 0),
                genesis_hashes.clone(),
                staking_keys[0],
            );

            let (hasht1s1, t1s1, _) = tools::create_block(
                &cfg,
                Slot::new(1 + start_period, 1),
                genesis_hashes.clone(),
                staking_keys[0],
            );

            let (hasht0s2, t0s2, _) = tools::create_block(
                &cfg,
                Slot::new(2 + start_period, 0),
                vec![hasht0s1, hasht1s1],
                staking_keys[0],
            );
            let (hasht1s2, t1s2, _) = tools::create_block(
                &cfg,
                Slot::new(2 + start_period, 1),
                vec![hasht0s1, hasht1s1],
                staking_keys[0],
            );

            let (hasht0s3, t0s3, _) = tools::create_block(
                &cfg,
                Slot::new(3 + start_period, 0),
                vec![hasht0s2, hasht1s2],
                staking_keys[0],
            );
            let (hasht1s3, t1s3, _) = tools::create_block(
                &cfg,
                Slot::new(3 + start_period, 1),
                vec![hasht0s2, hasht1s2],
                staking_keys[0],
            );

            let (hasht0s4, t0s4, _) = tools::create_block(
                &cfg,
                Slot::new(4 + start_period, 0),
                vec![hasht0s3, hasht1s3],
                staking_keys[0],
            );
            let (hasht1s4, t1s4, _) = tools::create_block(
                &cfg,
                Slot::new(4 + start_period, 1),
                vec![hasht0s3, hasht1s3],
                staking_keys[0],
            );

            // send blocks  t0s1, t1s1,
            protocol_controller.receive_block(t0s1).await;
            protocol_controller.receive_block(t1s1).await;
            // send blocks t0s3, t1s4, t0s4, t0s2, t1s3, t1s2
            protocol_controller.receive_block(t0s3).await;
            protocol_controller.receive_block(t1s4).await;
            protocol_controller.receive_block(t0s4).await;
            protocol_controller.receive_block(t0s2).await;
            protocol_controller.receive_block(t1s3).await;
            protocol_controller.receive_block(t1s2).await;

            // block t0s1 and t1s1 are propagated
            let hash_list = vec![hasht0s1, hasht1s1];
            tools::validate_propagate_block_in_list(
                &mut protocol_controller,
                &hash_list,
                3000 + start_period * 1000,
            )
            .await;
            tools::validate_propagate_block_in_list(&mut protocol_controller, &hash_list, 1000)
                .await;
            // block t0s2 and t1s2 are propagated
            let hash_list = vec![hasht0s2, hasht1s2];
            tools::validate_propagate_block_in_list(&mut protocol_controller, &hash_list, 1000)
                .await;
            tools::validate_propagate_block_in_list(&mut protocol_controller, &hash_list, 1000)
                .await;
            // block t0s3 and t1s3 are propagated
            let hash_list = vec![hasht0s3, hasht1s3];
            tools::validate_propagate_block_in_list(&mut protocol_controller, &hash_list, 1000)
                .await;
            tools::validate_propagate_block_in_list(&mut protocol_controller, &hash_list, 1000)
                .await;
            // block t0s4 and t1s4 are propagated
            let hash_list = vec![hasht0s4, hasht1s4];
            tools::validate_propagate_block_in_list(&mut protocol_controller, &hash_list, 1000)
                .await;
            tools::validate_propagate_block_in_list(&mut protocol_controller, &hash_list, 4000)
                .await;
            (
                protocol_controller,
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

//test future_incoming_blocks block in the future with max_future_processing_blocks.
#[tokio::test]
#[serial]
async fn test_unsorted_block_with_to_much_in_the_future() {
    /*stderrlog::new()
    .verbosity(4)
    .timestamp(stderrlog::Timestamp::Millisecond)
    .init()
    .unwrap();*/
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<PrivateKey> = (0..1).map(|_| generate_random_private_key()).collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.genesis_timestamp = UTime::now(0).unwrap().saturating_sub(2000.into()); // slot 1 is in the past
    cfg.future_block_processing_max_periods = 3;
    cfg.max_future_processing_blocks = 5;

    tools::consensus_without_pool_test(
        cfg.clone(),
        async move |mut protocol_controller, consensus_command_sender, consensus_event_receiver| {
            // create test blocks
            let genesis_hashes = consensus_command_sender
                .get_block_graph_status(None, None)
                .await
                .expect("could not get block graph status")
                .genesis_blocks;

            // a block in the past must be propagated
            let (hash1, block1, _) = tools::create_block(
                &cfg,
                Slot::new(1, 0),
                genesis_hashes.clone(),
                staking_keys[0],
            );
            protocol_controller.receive_block(block1).await;
            tools::validate_propagate_block(&mut protocol_controller, hash1, 2500).await;

            // this block is slightly in the future: will wait for it
            let slot = timeslots::get_current_latest_block_slot(
                cfg.thread_count,
                cfg.t0,
                cfg.genesis_timestamp,
                0,
            )
            .unwrap()
            .unwrap();
            let (hash2, block2, _) = tools::create_block(
                &cfg,
                Slot::new(slot.period + 2, slot.thread),
                genesis_hashes.clone(),
                staking_keys[0],
            );
            protocol_controller.receive_block(block2).await;
            assert!(
                !tools::validate_notpropagate_block(&mut protocol_controller, hash2, 500).await
            );
            tools::validate_propagate_block(&mut protocol_controller, hash2, 2500).await;

            // this block is too much in the future: do not process
            let slot = timeslots::get_current_latest_block_slot(
                cfg.thread_count,
                cfg.t0,
                cfg.genesis_timestamp,
                0,
            )
            .unwrap()
            .unwrap();
            let (hash3, block3, _) = tools::create_block(
                &cfg,
                Slot::new(slot.period + 1000, slot.thread),
                genesis_hashes.clone(),
                staking_keys[0],
            );
            protocol_controller.receive_block(block3).await;
            assert!(
                !tools::validate_notpropagate_block(&mut protocol_controller, hash3, 2500).await
            );

            // Check that the block has been silently dropped and not discarded for being too much in the future.
            let block_graph = consensus_command_sender
                .get_block_graph_status(None, None)
                .await
                .unwrap();
            assert!(!block_graph.active_blocks.contains_key(&hash3));
            assert!(!block_graph.discarded_blocks.contains_key(&hash3));
            (
                protocol_controller,
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_too_many_blocks_in_the_future() {
    /*stderrlog::new()
    .verbosity(4)
    .timestamp(stderrlog::Timestamp::Millisecond)
    .init()
    .unwrap();*/
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<PrivateKey> = (0..1).map(|_| generate_random_private_key()).collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.future_block_processing_max_periods = 100;
    cfg.max_future_processing_blocks = 2;
    cfg.delta_f0 = 1000;
    cfg.genesis_timestamp = UTime::now(0).unwrap().saturating_sub(2000.into()); // slot 1 is in the past

    tools::consensus_without_pool_test(
        cfg.clone(),
        async move |mut protocol_controller, consensus_command_sender, consensus_event_receiver| {
            // get genesis block hashes
            let genesis_hashes = consensus_command_sender
                .get_block_graph_status(None, None)
                .await
                .expect("could not get block graph status")
                .genesis_blocks;

            // generate 5 blocks but there is only space for 2 in the waiting line
            let mut expected_block_hashes: HashSet<BlockId> = HashSet::new();
            let mut max_period = 0;
            let slot = timeslots::get_current_latest_block_slot(
                cfg.thread_count,
                cfg.t0,
                cfg.genesis_timestamp,
                0,
            )
            .unwrap()
            .unwrap();
            for period in 0..5 {
                max_period = slot.period + 2 + period;
                let (hash, block, _) = tools::create_block(
                    &cfg,
                    Slot::new(max_period, slot.thread),
                    genesis_hashes.clone(),
                    staking_keys[0],
                );
                protocol_controller.receive_block(block).await;
                if period < 2 {
                    expected_block_hashes.insert(hash);
                }
            }
            // wait for the 2 waiting blocks to propagate
            let mut expected_clone = expected_block_hashes.clone();
            while !expected_block_hashes.is_empty() {
                assert!(
                    expected_block_hashes.remove(
                        &tools::validate_propagate_block_in_list(
                            &mut protocol_controller,
                            &expected_block_hashes.iter().copied().collect(),
                            2500
                        )
                        .await
                    ),
                    "unexpected block propagated"
                );
            }
            // wait until we reach the slot of the last block
            while timeslots::get_current_latest_block_slot(
                cfg.thread_count,
                cfg.t0,
                cfg.genesis_timestamp,
                0,
            )
            .unwrap()
            .unwrap()
                < Slot::new(max_period + 1, 0)
            {}
            // ensure that the graph contains only what we expect
            let graph = consensus_command_sender
                .get_block_graph_status(None, None)
                .await
                .expect("could not get block graph status");
            expected_clone.extend(graph.genesis_blocks);
            assert_eq!(
                expected_clone,
                graph.active_blocks.keys().copied().collect(),
                "unexpected block graph"
            );
            (
                protocol_controller,
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_dep_in_back_order() {
    /*stderrlog::new()
    .verbosity(4)
    .timestamp(stderrlog::Timestamp::Millisecond)
    .init()
    .unwrap();*/
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<PrivateKey> = (0..1).map(|_| generate_random_private_key()).collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.genesis_timestamp = UTime::now(0)
        .unwrap()
        .saturating_sub(cfg.t0.checked_mul(1000).unwrap());
    cfg.max_dependency_blocks = 10;

    tools::consensus_without_pool_test(
        cfg.clone(),
        async move |mut protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let genesis_hashes = consensus_command_sender
                .get_block_graph_status(None, None)
                .await
                .expect("could not get block graph status")
                .genesis_blocks;

            // create test blocks
            let (hasht0s1, t0s1, _) = tools::create_block(
                &cfg,
                Slot::new(1, 0),
                genesis_hashes.clone(),
                staking_keys[0],
            );

            let (hasht1s1, t1s1, _) = tools::create_block(
                &cfg,
                Slot::new(1, 1),
                genesis_hashes.clone(),
                staking_keys[0],
            );

            let (hasht0s2, t0s2, _) = tools::create_block(
                &cfg,
                Slot::new(2, 0),
                vec![hasht0s1, hasht1s1],
                staking_keys[0],
            );
            let (hasht1s2, t1s2, _) = tools::create_block(
                &cfg,
                Slot::new(2, 1),
                vec![hasht0s1, hasht1s1],
                staking_keys[0],
            );

            let (hasht0s3, t0s3, _) = tools::create_block(
                &cfg,
                Slot::new(3, 0),
                vec![hasht0s2, hasht1s2],
                staking_keys[0],
            );
            let (hasht1s3, t1s3, _) = tools::create_block(
                &cfg,
                Slot::new(3, 1),
                vec![hasht0s2, hasht1s2],
                staking_keys[0],
            );

            let (hasht0s4, t0s4, _) = tools::create_block(
                &cfg,
                Slot::new(4, 0),
                vec![hasht0s3, hasht1s3],
                staking_keys[0],
            );
            let (hasht1s4, t1s4, _) = tools::create_block(
                &cfg,
                Slot::new(4, 1),
                vec![hasht0s3, hasht1s3],
                staking_keys[0],
            );

            // send blocks   t0s2, t1s3, t0s1, t0s4, t1s4, t1s1, t0s3, t1s2
            protocol_controller.receive_block(t0s2).await; // not propagated and update wishlist
            tools::validate_wishlist(
                &mut protocol_controller,
                vec![hasht0s1, hasht1s1].into_iter().collect(),
                BlockHashSet::default(),
                500,
            )
            .await;
            tools::validate_notpropagate_block(&mut protocol_controller, hasht0s2, 500).await;

            protocol_controller.receive_block(t1s3).await; // not propagated and no wishlist update
            tools::validate_notpropagate_block(&mut protocol_controller, hasht1s3, 500).await;

            protocol_controller.receive_block(t0s1).await; // we have its parents so it should be integrated right now and update wishlist

            tools::validate_propagate_block(&mut protocol_controller, hasht0s1, 500).await;
            tools::validate_wishlist(
                &mut protocol_controller,
                BlockHashSet::default(),
                vec![hasht0s1].into_iter().collect(),
                500,
            )
            .await;

            protocol_controller.receive_block(t0s4).await; // not propagated and no wishlist update
            tools::validate_notpropagate_block(&mut protocol_controller, hasht0s4, 500).await;

            protocol_controller.receive_block(t1s4).await; // not propagated and no wishlist update
            tools::validate_notpropagate_block(&mut protocol_controller, hasht1s4, 500).await;

            protocol_controller.receive_block(t1s1).await; // assert t1s1 is integrated and t0s2 is integrated and wishlist updated
            tools::validate_propagate_block_in_list(
                &mut protocol_controller,
                &vec![hasht1s1, hasht0s2],
                500,
            )
            .await;

            tools::validate_propagate_block_in_list(
                &mut protocol_controller,
                &vec![hasht1s1, hasht0s2],
                500,
            )
            .await;
            tools::validate_wishlist(
                &mut protocol_controller,
                vec![].into_iter().collect(),
                vec![hasht1s1].into_iter().collect(),
                500,
            )
            .await;

            protocol_controller.receive_block(t0s3).await; // not propagated and no wishlist update
            tools::validate_notpropagate_block(&mut protocol_controller, hasht0s3, 500).await;

            protocol_controller.receive_block(t1s2).await;

            // All remaining blocks are propagated
            let integrated = vec![hasht1s2, hasht0s3, hasht1s3, hasht0s4, hasht1s4];
            tools::validate_propagate_block_in_list(&mut protocol_controller, &integrated, 1000)
                .await;
            tools::validate_propagate_block_in_list(&mut protocol_controller, &integrated, 1000)
                .await;
            tools::validate_propagate_block_in_list(&mut protocol_controller, &integrated, 1000)
                .await;
            tools::validate_propagate_block_in_list(&mut protocol_controller, &integrated, 1000)
                .await;
            tools::validate_propagate_block_in_list(&mut protocol_controller, &integrated, 1000)
                .await;
            tools::validate_wishlist(
                &mut protocol_controller,
                BlockHashSet::default(),
                vec![hasht1s2].into_iter().collect(),
                500,
            )
            .await;
            (
                protocol_controller,
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_dep_in_back_order_with_max_dependency_blocks() {
    /*stderrlog::new()
    .verbosity(4)
    .timestamp(stderrlog::Timestamp::Millisecond)
    .init()
    .unwrap();*/
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<PrivateKey> = (0..1).map(|_| generate_random_private_key()).collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.genesis_timestamp = UTime::now(0)
        .unwrap()
        .saturating_sub(cfg.t0.checked_mul(1000).unwrap());
    cfg.max_dependency_blocks = 2;

    tools::consensus_without_pool_test(
        cfg.clone(),
        async move |mut protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let genesis_hashes = consensus_command_sender
                .get_block_graph_status(None, None)
                .await
                .expect("could not get block graph status")
                .genesis_blocks;

            // create test blocks

            let (hasht0s1, t0s1, _) = tools::create_block(
                &cfg,
                Slot::new(1, 0),
                genesis_hashes.clone(),
                staking_keys[0],
            );

            let (hasht1s1, t1s1, _) = tools::create_block(
                &cfg,
                Slot::new(1, 1),
                genesis_hashes.clone(),
                staking_keys[0],
            );

            let (hasht0s2, t0s2, _) = tools::create_block(
                &cfg,
                Slot::new(2, 0),
                vec![hasht0s1, hasht1s1],
                staking_keys[0],
            );
            let (hasht1s2, t1s2, _) = tools::create_block(
                &cfg,
                Slot::new(2, 1),
                vec![hasht0s1, hasht1s1],
                staking_keys[0],
            );

            let (hasht0s3, t0s3, _) = tools::create_block(
                &cfg,
                Slot::new(3, 0),
                vec![hasht0s2, hasht1s2],
                staking_keys[0],
            );
            let (hasht1s3, t1s3, _) = tools::create_block(
                &cfg,
                Slot::new(3, 1),
                vec![hasht0s2, hasht1s2],
                staking_keys[0],
            );

            // send blocks   t0s2, t1s3, t0s1, t0s4, t1s4, t1s1, t0s3, t1s2
            protocol_controller.receive_block(t0s2).await;
            tools::validate_wishlist(
                &mut protocol_controller,
                vec![hasht0s1, hasht1s1].into_iter().collect(),
                BlockHashSet::default(),
                500,
            )
            .await;
            tools::validate_notpropagate_block(&mut protocol_controller, hasht0s2, 500).await;

            protocol_controller.receive_block(t1s3).await;
            tools::validate_notpropagate_block(&mut protocol_controller, hasht1s3, 500).await;

            protocol_controller.receive_block(t0s1).await;
            tools::validate_propagate_block(&mut protocol_controller, hasht0s1, 500).await;
            tools::validate_wishlist(
                &mut protocol_controller,
                BlockHashSet::default(),
                vec![hasht0s1].into_iter().collect(),
                500,
            )
            .await;
            protocol_controller.receive_block(t0s3).await;
            tools::validate_notpropagate_block(&mut protocol_controller, hasht0s3, 500).await;

            protocol_controller.receive_block(t1s2).await;
            tools::validate_notpropagate_block(&mut protocol_controller, hasht1s2, 500).await;

            protocol_controller.receive_block(t1s1).await;
            tools::validate_propagate_block_in_list(
                &mut protocol_controller,
                &vec![hasht1s1, hasht1s2],
                500,
            )
            .await;
            tools::validate_propagate_block_in_list(
                &mut protocol_controller,
                &vec![hasht1s1, hasht1s2],
                500,
            )
            .await;
            tools::validate_wishlist(
                &mut protocol_controller,
                BlockHashSet::default(),
                vec![hasht1s1].into_iter().collect(),
                500,
            )
            .await;

            (
                protocol_controller,
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_add_block_that_depends_on_invalid_block() {
    /*stderrlog::new()
    .verbosity(4)
    .timestamp(stderrlog::Timestamp::Millisecond)
    .init()
    .unwrap();*/
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<PrivateKey> = (0..1).map(|_| generate_random_private_key()).collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.genesis_timestamp = UTime::now(0)
        .unwrap()
        .saturating_sub(cfg.t0.checked_mul(1000).unwrap());
    cfg.max_dependency_blocks = 7;

    tools::consensus_without_pool_test(
        cfg.clone(),
        async move |mut protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let genesis_hashes = consensus_command_sender
                .get_block_graph_status(None, None)
                .await
                .expect("could not get block graph status")
                .genesis_blocks;

            // create test blocks
            let (hasht0s1, t0s1, _) = tools::create_block(
                &cfg,
                Slot::new(1, 0),
                genesis_hashes.clone(),
                staking_keys[0],
            );

            let (hasht1s1, t1s1, _) = tools::create_block(
                &cfg,
                Slot::new(1, 1),
                genesis_hashes.clone(),
                staking_keys[0],
            );

            // blocks t3s2 with wrong thread and (t0s1, t1s1) parents.
            let (hasht3s2, t3s2, _) = tools::create_block(
                &cfg,
                Slot::new(2, 3),
                vec![hasht0s1, hasht1s1],
                staking_keys[0],
            );

            // blocks t0s3 and t1s3 with (t3s2, t1s2) parents.
            let (hasht0s3, t0s3, _) = tools::create_block(
                &cfg,
                Slot::new(3, 0),
                vec![hasht3s2, hasht1s1],
                staking_keys[0],
            );
            let (hasht1s3, t1s3, _) = tools::create_block(
                &cfg,
                Slot::new(3, 1),
                vec![hasht3s2, hasht1s1],
                staking_keys[0],
            );

            // add block in this order t0s1, t1s1, t0s3, t1s3, t3s2
            // send blocks   t0s2, t1s3, t0s1, t0s4, t1s4, t1s1, t0s3, t1s2
            protocol_controller.receive_block(t0s1).await;
            protocol_controller.receive_block(t1s1).await;
            protocol_controller.receive_block(t0s3).await;
            protocol_controller.receive_block(t1s3).await;
            protocol_controller.receive_block(t3s2).await;

            // block t0s1 and t1s1 are propagated
            let hash_list = vec![hasht0s1, hasht1s1];
            tools::validate_propagate_block_in_list(&mut protocol_controller, &hash_list, 1000)
                .await;
            tools::validate_propagate_block_in_list(&mut protocol_controller, &hash_list, 1000)
                .await;

            // block  t0s3, t1s3 are not propagated
            let hash_list = vec![hasht0s3, hasht1s3];
            assert!(
                !tools::validate_notpropagate_block_in_list(
                    &mut protocol_controller,
                    &hash_list,
                    2000
                )
                .await
            );
            assert!(
                !tools::validate_notpropagate_block_in_list(
                    &mut protocol_controller,
                    &hash_list,
                    2000
                )
                .await
            );
            (
                protocol_controller,
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}
