use serial_test::serial;
use std::collections::HashMap;

use communication::protocol::ProtocolCommand;
use crypto::hash::Hash;
use models::SerializeCompact;
use models::{Address, Block, BlockHeader, BlockHeaderContent, Slot};
use pool::PoolCommand;
use time::UTime;

use crate::{
    ledger::LedgerData,
    pos::{RollCounts, RollUpdate, RollUpdates},
    start_consensus_controller,
    tests::{
        mock_pool_controller::{MockPoolController, PoolCommandSink},
        mock_protocol_controller::MockProtocolController,
        tools::{self, create_roll_transaction, create_transaction, generate_ledger_file},
    },
};

// implement test of issue !424.
#[tokio::test]
#[serial]
async fn test_block_creation_with_draw() {
    let thread_count = 2;
    //define addresses use for the test
    // addresses a and b both in thread 0
    // addr 1 has 1 roll and 0 coins
    // addr 2 is in consensus and has 0 roll and 1000 coins
    let mut priv_1 = crypto::generate_random_private_key();
    let mut pubkey_1 = crypto::derive_public_key(&priv_1);
    let mut address_1 = Address::from_public_key(&pubkey_1).unwrap();
    while 0 != address_1.get_thread(thread_count) {
        priv_1 = crypto::generate_random_private_key();
        pubkey_1 = crypto::derive_public_key(&priv_1);
        address_1 = Address::from_public_key(&pubkey_1).unwrap();
    }
    assert_eq!(0, address_1.get_thread(thread_count));

    let mut priv_2 = crypto::generate_random_private_key();
    let mut pubkey_2 = crypto::derive_public_key(&priv_2);
    let mut address_2 = Address::from_public_key(&pubkey_2).unwrap();
    while 0 != address_2.get_thread(thread_count) {
        priv_2 = crypto::generate_random_private_key();
        pubkey_2 = crypto::derive_public_key(&priv_2);
        address_2 = Address::from_public_key(&pubkey_2).unwrap();
    }
    assert_eq!(0, address_2.get_thread(thread_count));

    let mut ledger = HashMap::new();
    ledger.insert(address_2, LedgerData { balance: 1000 });
    let ledger_file = generate_ledger_file(&ledger);
    let staking_keys: Vec<crypto::signature::PrivateKey> = vec![priv_1, priv_2];

    //init roll cont
    let mut roll_counts: Vec<RollCounts> = vec![RollCounts::new(); 2];
    let update = RollUpdate {
        roll_purchases: 1,
        roll_sales: 0,
    };
    let mut updates = RollUpdates::new();
    updates.apply(&address_1, &update).unwrap();
    roll_counts[0].apply_subset(&updates, None).unwrap();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);

    let roll_counts_file = tools::generate_roll_counts_file(&roll_counts);
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.roll_price = 1000;
    cfg.periods_per_cycle = 10_000;
    cfg.t0 = 1000.into();
    cfg.pos_lookback_cycles = 2;
    cfg.thread_count = thread_count;
    cfg.delta_f0 = 3;
    cfg.genesis_timestamp = UTime::now(0)
        .unwrap()
        .checked_sub((cfg.t0.to_millis() * cfg.periods_per_cycle * 3).into())
        .unwrap()
        .checked_add(2000.into())
        .unwrap();
    cfg.block_reward = 0;
    cfg.disable_block_creation = false;
    cfg.operation_validity_periods = 100;
    cfg.operation_batch_size = 3;
    cfg.max_operations_per_block = 50;

    let operation_fee = 0;

    tools::consensus_pool_test(
        cfg.clone(),
        async move |pool_controller,
                    mut protocol_controller,
                    consensus_command_sender,
                    consensus_event_receiver| {
            let genesis_ids = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status")
                .genesis_blocks;

            // initial block: addr2 buys 1 roll
            println!("1");
            let op1 = create_roll_transaction(priv_2, pubkey_2, 1, true, 10, operation_fee);
            let (initial_block_id, block, _) = tools::create_block_with_operations(
                &cfg,
                Slot::new(1, 0),
                &genesis_ids,
                staking_keys[0].clone(),
                vec![op1],
            );
            tools::propagate_block(&mut protocol_controller, block, true, 1000).await;

            // make cycle 0 final/finished by sending enough blocks in each thread in cycle 1
            // note that blocks in cycle 3 may be created during this, so make sure that their clique is overrun by sending a large amount of blocks
            let mut cur_parents = vec![initial_block_id, genesis_ids[1]];
            for delta_period in 0u64..10 {
                for thread in 0..cfg.thread_count {
                    println!("2: {:?}", thread);
                    let res_block_id = tools::create_and_test_block(
                        &mut protocol_controller,
                        &cfg,
                        Slot::new(cfg.periods_per_cycle + delta_period, thread),
                        cur_parents.clone(),
                        true,
                        false,
                        staking_keys[0].clone(),
                    )
                    .await;
                    cur_parents[thread as usize] = res_block_id;
                    println!("2.1: {:?}", thread);
                }
            }

            println!("3");

            // get draws for cycle 3 (lookback = cycle 0)
            let draws = consensus_command_sender
                .get_selection_draws(
                    Slot::new(3 * cfg.periods_per_cycle, 0),
                    Slot::new(4 * cfg.periods_per_cycle, 0),
                )
                .await
                .unwrap();
            let nb_address1_draws = draws.iter().filter(|(_, addr)| *addr == address_1).count();
            // fair coin test. See https://en.wikipedia.org/wiki/Checking_whether_a_coin_is_fair
            // note: this is a statistical test. It may fail in rare occasions.
            assert!(
                (0.5 - ((nb_address1_draws as f32)
                    / ((cfg.thread_count as u64 * cfg.periods_per_cycle) as f32)))
                    .abs()
                    < 0.15
            );

            // check 10 draws
            let draws: HashMap<Slot, Address> = draws.into_iter().collect();
            let mut cur_slot = Slot::new(cfg.periods_per_cycle * 3, 0);
            for _ in 0..10 {
                // wait block propagation
                let block_creator = protocol_controller
                    .wait_command(3000.into(), |cmd| match cmd {
                        ProtocolCommand::IntegratedBlock { block, .. } => {
                            if block.header.content.slot == cur_slot {
                                Some(block.header.content.creator)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .await
                    .expect("block did not propagate in time");
                assert_eq!(
                    draws[&cur_slot],
                    Address::from_public_key(&block_creator).unwrap(),
                    "wrong block creator"
                );
                cur_slot = cur_slot.get_next_slot(cfg.thread_count).unwrap();
            }
            (
                pool_controller,
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
async fn test_order_of_inclusion() {
    // // setup logging
    // stderrlog::new()
    //     .verbosity(4)
    //     .timestamp(stderrlog::Timestamp::Millisecond)
    //     .init()
    //     .unwrap();
    let thread_count = 2;
    //define addresses use for the test
    // addresses a and b both in thread 0
    let mut priv_a = crypto::generate_random_private_key();
    let mut pubkey_a = crypto::derive_public_key(&priv_a);
    let mut address_a = Address::from_public_key(&pubkey_a).unwrap();
    while 0 != address_a.get_thread(thread_count) {
        priv_a = crypto::generate_random_private_key();
        pubkey_a = crypto::derive_public_key(&priv_a);
        address_a = Address::from_public_key(&pubkey_a).unwrap();
    }
    assert_eq!(0, address_a.get_thread(thread_count));

    let mut priv_b = crypto::generate_random_private_key();
    let mut pubkey_b = crypto::derive_public_key(&priv_b);
    let mut address_b = Address::from_public_key(&pubkey_b).unwrap();
    while 0 != address_b.get_thread(thread_count) {
        priv_b = crypto::generate_random_private_key();
        pubkey_b = crypto::derive_public_key(&priv_b);
        address_b = Address::from_public_key(&pubkey_b).unwrap();
    }
    assert_eq!(0, address_b.get_thread(thread_count));

    let mut ledger = HashMap::new();
    ledger.insert(address_a, LedgerData { balance: 100 });
    let ledger_file = generate_ledger_file(&ledger);
    let staking_keys: Vec<crypto::signature::PrivateKey> = (0..1)
        .map(|_| crypto::generate_random_private_key())
        .collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);

    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.delta_f0 = 32;
    cfg.disable_block_creation = false;
    cfg.thread_count = thread_count;
    cfg.operation_validity_periods = 10;
    cfg.operation_batch_size = 3;
    cfg.max_operations_per_block = 50;
    //to avoid timing pb for block in the future
    cfg.genesis_timestamp = UTime::now(0).unwrap();

    let op1 = create_transaction(priv_a, pubkey_a, address_b, 5, 10, 1);
    let op2 = create_transaction(priv_a, pubkey_a, address_b, 50, 10, 10);
    let op3 = create_transaction(priv_b, pubkey_b, address_a, 10, 10, 15);

    // there is only one node so it should be drawn at every slot

    // mock protocol & pool
    let (mut protocol_controller, protocol_command_sender, protocol_event_receiver) =
        MockProtocolController::new();
    let (mut pool_controller, pool_command_sender) = MockPoolController::new();

    // launch consensus controller
    let (_consensus_command_sender, consensus_event_receiver, consensus_manager) =
        start_consensus_controller(
            cfg.clone(),
            protocol_command_sender.clone(),
            protocol_event_receiver,
            pool_command_sender,
            None,
            None,
            None,
            0,
        )
        .await
        .expect("could not start consensus controller");

    //wait for fisrt slot
    pool_controller
        .wait_command(cfg.t0.checked_mul(2).unwrap(), |cmd| match cmd {
            PoolCommand::UpdateCurrentSlot(s) => {
                if s == Slot::new(1, 0) {
                    Some(())
                } else {
                    None
                }
            }
            _ => None,
        })
        .await
        .expect("timeout while waiting for slot");

    // respond to first pool batch command
    pool_controller
        .wait_command(300.into(), |cmd| match cmd {
            PoolCommand::GetOperationBatch { response_tx, .. } => {
                response_tx
                    .send(vec![
                        (op3.get_operation_id().unwrap(), op3.clone(), 50),
                        (op2.get_operation_id().unwrap(), op2.clone(), 50),
                        (op1.get_operation_id().unwrap(), op1.clone(), 50),
                    ])
                    .unwrap();
                Some(())
            }
            _ => None,
        })
        .await
        .expect("timeout while waiting for 1st operation batch request");

    // respond to second pool batch command
    pool_controller
        .wait_command(300.into(), |cmd| match cmd {
            PoolCommand::GetOperationBatch {
                response_tx,
                exclude,
                ..
            } => {
                assert!(!exclude.is_empty());
                response_tx.send(vec![]).unwrap();
                Some(())
            }
            _ => None,
        })
        .await
        .expect("timeout while waiting for 2nd operation batch request");

    // wait for block
    let (_block_id, block) = protocol_controller
        .wait_command(300.into(), |cmd| match cmd {
            ProtocolCommand::IntegratedBlock { block_id, block } => Some((block_id, block)),
            _ => None,
        })
        .await
        .expect("timeout while waiting for block");

    // assert it's the expected block
    assert_eq!(block.header.content.slot, Slot::new(1, 0));
    let expected = vec![op2.clone(), op1.clone()];
    let res = block.operations.clone();
    assert_eq!(block.operations.len(), 2);
    for i in 0..2 {
        assert_eq!(
            expected[i].get_operation_id().unwrap(),
            res[i].get_operation_id().unwrap()
        );
    }

    // stop controller while ignoring all commands
    let pool_sink = PoolCommandSink::new(pool_controller).await;
    let stop_fut = consensus_manager.stop(consensus_event_receiver);
    tokio::pin!(stop_fut);
    protocol_controller
        .ignore_commands_while(stop_fut)
        .await
        .unwrap();
    pool_sink.stop().await;
}

#[tokio::test]
#[serial]
async fn test_block_filling() {
    // // setup logging
    // stderrlog::new()
    //     .verbosity(2)
    //     .timestamp(stderrlog::Timestamp::Millisecond)
    //     .init()
    //     .unwrap();
    let thread_count = 2;
    //define addresses use for the test
    // addresses a and b both in thread 0
    let mut priv_a = crypto::generate_random_private_key();
    let mut pubkey_a = crypto::derive_public_key(&priv_a);
    let mut address_a = Address::from_public_key(&pubkey_a).unwrap();
    while 0 != address_a.get_thread(thread_count) {
        priv_a = crypto::generate_random_private_key();
        pubkey_a = crypto::derive_public_key(&priv_a);
        address_a = Address::from_public_key(&pubkey_a).unwrap();
    }
    assert_eq!(0, address_a.get_thread(thread_count));

    let mut ledger = HashMap::new();
    ledger.insert(
        address_a,
        LedgerData {
            balance: 1_000_000_000,
        },
    );
    let ledger_file = generate_ledger_file(&ledger);
    let staking_keys: Vec<crypto::signature::PrivateKey> = (0..1)
        .map(|_| crypto::generate_random_private_key())
        .collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.delta_f0 = 32;
    cfg.disable_block_creation = false;
    cfg.thread_count = thread_count;
    cfg.operation_validity_periods = 10;
    cfg.operation_batch_size = 500;
    cfg.max_operations_per_block = 5000;
    cfg.max_block_size = 500;
    let mut ops = Vec::new();
    for _ in 0..500 {
        ops.push(create_transaction(priv_a, pubkey_a, address_a, 5, 10, 1))
    }

    // there is only one node so it should be drawn at every slot

    // mock protocol & pool
    let (mut protocol_controller, protocol_command_sender, protocol_event_receiver) =
        MockProtocolController::new();
    let (mut pool_controller, pool_command_sender) = MockPoolController::new();

    // launch consensus controller
    cfg.genesis_timestamp = UTime::now(0).unwrap();
    let (_consensus_command_sender, consensus_event_receiver, consensus_manager) =
        start_consensus_controller(
            cfg.clone(),
            protocol_command_sender.clone(),
            protocol_event_receiver,
            pool_command_sender,
            None,
            None,
            None,
            0,
        )
        .await
        .expect("could not start consensus controller");

    let op_size = 10;

    //wait for fisrt slot
    pool_controller
        .wait_command(cfg.t0.checked_mul(2).unwrap(), |cmd| match cmd {
            PoolCommand::UpdateCurrentSlot(s) => {
                if s == Slot::new(1, 0) {
                    Some(())
                } else {
                    None
                }
            }
            _ => None,
        })
        .await
        .expect("timeout while waiting for slot");

    // respond to first pool batch command
    pool_controller
        .wait_command(300.into(), |cmd| match cmd {
            PoolCommand::GetOperationBatch { response_tx, .. } => {
                response_tx
                    .send(
                        ops.iter()
                            .map(|op| (op.get_operation_id().unwrap(), op.clone(), op_size))
                            .collect(),
                    )
                    .unwrap();
                Some(())
            }
            _ => None,
        })
        .await
        .expect("timeout while waiting for 1st operation batch request");

    // respond to second pool batch command
    pool_controller
        .wait_command(300.into(), |cmd| match cmd {
            PoolCommand::GetOperationBatch {
                response_tx,
                exclude,
                ..
            } => {
                assert!(!exclude.is_empty());
                response_tx.send(vec![]).unwrap();
                Some(())
            }
            _ => None,
        })
        .await
        .expect("timeout while waiting for 2nd operation batch request");

    // wait for block
    let (_block_id, block) = protocol_controller
        .wait_command(500.into(), |cmd| match cmd {
            ProtocolCommand::IntegratedBlock { block_id, block } => Some((block_id, block)),
            _ => None,
        })
        .await
        .expect("timeout while waiting for block");

    // assert it's the expected block
    assert_eq!(block.header.content.slot, Slot::new(1, 0));
    // create empty block
    let (_block_id, header) = BlockHeader::new_signed(
        &priv_a,
        BlockHeaderContent {
            creator: block.header.content.creator,
            slot: block.header.content.slot,
            parents: block.header.content.parents.clone(),
            operation_merkle_root: Hash::hash(&Vec::new()[..]),
        },
    )
    .unwrap();
    let empty = Block {
        header,
        operations: Vec::new(),
    };
    let remaining_block_space = (cfg.max_block_size as usize)
        .checked_sub(empty.to_bytes_compact().unwrap().len() as usize)
        .unwrap();

    let nb = remaining_block_space / (op_size as usize);
    assert_eq!(block.operations.len(), nb);

    // stop controller while ignoring all commands
    let pool_sink = PoolCommandSink::new(pool_controller).await;
    let stop_fut = consensus_manager.stop(consensus_event_receiver);
    tokio::pin!(stop_fut);
    protocol_controller
        .ignore_commands_while(stop_fut)
        .await
        .unwrap();
    pool_sink.stop().await;
}
