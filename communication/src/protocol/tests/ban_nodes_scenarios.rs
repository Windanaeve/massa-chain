// Copyright (c) 2021 MASSA LABS <info@massa.net>

use super::tools::protocol_test;
use super::{mock_network_controller::MockNetworkController, tools};
use crate::protocol::{start_protocol_controller, ProtocolEvent};
use crate::{network::NetworkCommand, protocol::ProtocolPoolEvent};
use models::Slot;
use serial_test::serial;
use std::collections::{HashMap, HashSet};

#[tokio::test]
#[serial]
async fn test_protocol_bans_node_sending_block_with_invalid_signature() {
    let protocol_config = tools::create_protocol_config();
    protocol_test(
        protocol_config,
        async move |mut network_controller,
                    mut protocol_event_receiver,
                    protocol_command_sender,
                    protocol_manager,
                    protocol_pool_event_receiver| {
            // Create 1 node.
            let mut nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

            let creator_node = nodes.pop().expect("Failed to get node info.");

            // 1. Create a block coming from one node.
            let mut block = tools::create_block(&creator_node.private_key, &creator_node.id.0);

            // 2. Change the slot.
            block.header.content.slot = Slot::new(1, 1);

            // 3. Send block to protocol.
            network_controller.send_block(creator_node.id, block).await;

            // The node is banned.
            tools::assert_banned_node(creator_node.id, &mut network_controller).await;

            // Check protocol does not send block to consensus.
            match tools::wait_protocol_event(&mut protocol_event_receiver, 1000.into(), |evt| {
                match evt {
                    evt @ ProtocolEvent::ReceivedBlock { .. } => Some(evt),
                    evt @ ProtocolEvent::ReceivedBlockHeader { .. } => Some(evt),
                    _ => None,
                }
            })
            .await
            {
                None => {}
                _ => panic!("Protocol unexpectedly sent block or header."),
            }
            (
                network_controller,
                protocol_event_receiver,
                protocol_command_sender,
                protocol_manager,
                protocol_pool_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_protocol_bans_node_sending_operation_with_invalid_signature() {
    let protocol_config = tools::create_protocol_config();
    protocol_test(
        protocol_config,
        async move |mut network_controller,
                    protocol_event_receiver,
                    protocol_command_sender,
                    protocol_manager,
                    mut protocol_pool_event_receiver| {
            // Create 1 node.
            let mut nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

            let creator_node = nodes.pop().expect("Failed to get node info.");

            // 1. Create an operation
            let mut operation = tools::create_operation();

            // 2. Change the validity period.
            operation.content.expire_period += 10;

            // 3. Send block to protocol.
            network_controller
                .send_operations(creator_node.id, vec![operation])
                .await;

            // The node is banned.
            tools::assert_banned_node(creator_node.id, &mut network_controller).await;

            // Check protocol does not send operation to pool.
            match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedOperations(..) => Some(evt),
                },
            )
            .await
            {
                None => {}
                _ => panic!("Protocol unexpectedly sent operation."),
            }
            (
                network_controller,
                protocol_event_receiver,
                protocol_command_sender,
                protocol_manager,
                protocol_pool_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_protocol_bans_node_sending_header_with_invalid_signature() {
    let protocol_config = tools::create_protocol_config();

    let (mut network_controller, network_command_sender, network_event_receiver) =
        MockNetworkController::new();

    // start protocol controller
    let (_, mut protocol_event_receiver, protocol_pool_event_receiver, protocol_manager) =
        start_protocol_controller(
            protocol_config.clone(),
            5u64,
            network_command_sender,
            network_event_receiver,
        )
        .await
        .expect("could not start protocol controller");

    // Create 1 node.
    let mut nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

    let to_ban_node = nodes.pop().expect("Failed to get node info.");

    // 1. Create a block coming from one node.
    let mut block = tools::create_block(&to_ban_node.private_key, &to_ban_node.id.0);

    // 2. Change the slot.
    block.header.content.slot = Slot::new(1, 1);

    // 3. Send header to protocol.
    network_controller
        .send_header(to_ban_node.id, block.header)
        .await;

    // The node is banned.
    tools::assert_banned_node(to_ban_node.id, &mut network_controller).await;

    // Check protocol does not send block to consensus.
    match tools::wait_protocol_event(&mut protocol_event_receiver, 1000.into(), |evt| match evt {
        evt @ ProtocolEvent::ReceivedBlock { .. } => Some(evt),
        evt @ ProtocolEvent::ReceivedBlockHeader { .. } => Some(evt),
        _ => None,
    })
    .await
    {
        None => {}
        _ => panic!("Protocol unexpectedly sent block or header."),
    }

    // Create another node.
    let not_banned = tools::create_and_connect_nodes(1, &mut network_controller)
        .await
        .pop()
        .expect("Node not created.");

    // Create a valid block from the other node.
    let block = tools::create_block(&not_banned.private_key, &not_banned.id.0);

    // 3. Send header to protocol, via the banned node.
    network_controller
        .send_header(to_ban_node.id, block.header)
        .await;

    // Check protocol does not send block to consensus.
    match tools::wait_protocol_event(&mut protocol_event_receiver, 1000.into(), |evt| match evt {
        evt @ ProtocolEvent::ReceivedBlock { .. } => Some(evt),
        evt @ ProtocolEvent::ReceivedBlockHeader { .. } => Some(evt),
        _ => None,
    })
    .await
    {
        None => {}
        _ => panic!("Protocol unexpectedly sent header coming from banned node."),
    }

    protocol_manager
        .stop(protocol_event_receiver, protocol_pool_event_receiver)
        .await
        .expect("Failed to shutdown protocol.");
}

#[tokio::test]
#[serial]
async fn test_protocol_does_not_asks_for_block_from_banned_node_who_propagated_header() {
    let protocol_config = tools::create_protocol_config();
    let (mut network_controller, network_command_sender, network_event_receiver) =
        MockNetworkController::new();

    let ask_for_block_cmd_filter = |cmd| match cmd {
        cmd @ NetworkCommand::AskForBlocks { .. } => Some(cmd),
        _ => None,
    };

    // start protocol controller
    let (
        mut protocol_command_sender,
        mut protocol_event_receiver,
        protocol_pool_event_receiver,
        protocol_manager,
    ) = start_protocol_controller(
        protocol_config.clone(),
        5u64,
        network_command_sender,
        network_event_receiver,
    )
    .await
    .expect("could not start protocol controller");

    let mut nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

    let creator_node = nodes.pop().expect("Failed to get node info.");

    // 1. Create a block coming from node creator_node.
    let block = tools::create_block(&creator_node.private_key, &creator_node.id.0);

    // 2. Send header to protocol.
    network_controller
        .send_header(creator_node.id, block.header.clone())
        .await;

    // Check protocol sends header to consensus.
    let received_hash =
        match tools::wait_protocol_event(&mut protocol_event_receiver, 1000.into(), |evt| match evt
        {
            evt @ ProtocolEvent::ReceivedBlockHeader { .. } => Some(evt),
            _ => None,
        })
        .await
        {
            Some(ProtocolEvent::ReceivedBlockHeader { block_id, .. }) => block_id,
            _ => panic!("Unexpected or no protocol event."),
        };

    // 3. Check that protocol sent the right header to consensus.
    let expected_hash = block
        .header
        .compute_block_id()
        .expect("Failed to compute hash.");
    assert_eq!(expected_hash, received_hash);

    // 4. Get the node banned.
    let mut block = tools::create_block(&creator_node.private_key, &creator_node.id.0);
    block.header.content.slot = Slot::new(1, 1);
    network_controller
        .send_header(creator_node.id, block.header)
        .await;
    tools::assert_banned_node(creator_node.id, &mut network_controller).await;

    // 5. Ask for block.
    protocol_command_sender
        .send_wishlist_delta(vec![expected_hash].into_iter().collect(), HashSet::new())
        .await
        .expect("Failed to ask for block.");

    // 6. Make sure protocol did not ask for the block from the banned node.
    let got_more_commands = network_controller
        .wait_command(100.into(), ask_for_block_cmd_filter)
        .await;
    assert!(
        got_more_commands.is_none(),
        "unexpected command {:?}",
        got_more_commands
    );

    protocol_manager
        .stop(protocol_event_receiver, protocol_pool_event_receiver)
        .await
        .expect("Failed to shutdown protocol.");
}

#[tokio::test]
#[serial]
async fn test_protocol_does_not_send_blocks_when_asked_for_by_banned_node() {
    let protocol_config = tools::create_protocol_config();

    let send_block_or_header_cmd_filter = |cmd| match cmd {
        cmd @ NetworkCommand::SendBlock { .. } => Some(cmd),
        cmd @ NetworkCommand::SendBlockHeader { .. } => Some(cmd),
        _ => None,
    };

    let (mut network_controller, network_command_sender, network_event_receiver) =
        MockNetworkController::new();

    // start protocol controller
    let (
        mut protocol_command_sender,
        mut protocol_event_receiver,
        protocol_pool_event_receiver,
        protocol_manager,
    ) = start_protocol_controller(
        protocol_config.clone(),
        5u64,
        network_command_sender,
        network_event_receiver,
    )
    .await
    .expect("could not start protocol controller");

    let mut nodes = tools::create_and_connect_nodes(4, &mut network_controller).await;

    let creator_node = nodes.pop().expect("Failed to get node info.");

    // 1. Close one connection.
    network_controller.close_connection(nodes[2].id).await;

    // 2. Create a block coming from creator_node.
    let block = tools::create_block(&creator_node.private_key, &creator_node.id.0);

    let expected_hash = block
        .header
        .compute_block_id()
        .expect("Failed to compute hash.");

    // 3. Simulate two nodes asking for a block.
    for n in 0..2 {
        network_controller
            .send_ask_for_block(nodes[n].id, vec![expected_hash])
            .await;

        // Check protocol sends get block event to consensus.
        let received_hash =
            match tools::wait_protocol_event(&mut protocol_event_receiver, 1000.into(), |evt| {
                match evt {
                    evt @ ProtocolEvent::GetBlocks(..) => Some(evt),
                    _ => None,
                }
            })
            .await
            {
                Some(ProtocolEvent::GetBlocks(mut list)) => {
                    list.pop().expect("Received empty list of hashes.")
                }
                _ => panic!("Unexpected or no protocol event."),
            };

        // Check that protocol sent the right hash to consensus.
        assert_eq!(expected_hash, received_hash);
    }

    // Get one node banned.
    let mut bad_block = tools::create_block(&nodes[1].private_key, &nodes[1].id.0);
    bad_block.header.content.slot = Slot::new(1, 1);
    network_controller
        .send_header(nodes[1].id, bad_block.header.clone())
        .await;
    tools::assert_banned_node(nodes[1].id, &mut network_controller).await;

    // 4. Simulate consensus sending block.
    let mut results = HashMap::new();
    results.insert(expected_hash.clone(), Some(block));
    protocol_command_sender
        .send_get_blocks_results(results)
        .await
        .expect("Failed to send get block results");

    // 5. Check that protocol sends the non-banned node the full block.
    let mut expecting_block = HashSet::new();
    expecting_block.insert(nodes[0].id.clone());
    loop {
        match network_controller
            .wait_command(1000.into(), send_block_or_header_cmd_filter)
            .await
        {
            Some(NetworkCommand::SendBlock { node, block }) => {
                let hash = block
                    .header
                    .compute_block_id()
                    .expect("Failed to compute hash.");
                assert_eq!(expected_hash, hash);
                assert!(expecting_block.remove(&node));
            }
            Some(NetworkCommand::SendBlockHeader { .. }) => {
                panic!("unexpected header sent");
            }
            None => {
                if expecting_block.is_empty() {
                    break;
                } else {
                    panic!("expecting a block to be sent");
                }
            }
            _ => panic!("Unexpected network command."),
        }
    }

    // 7. Make sure protocol did not send block to the banned node.
    let got_more_commands = network_controller
        .wait_command(100.into(), send_block_or_header_cmd_filter)
        .await;
    assert!(got_more_commands.is_none());

    protocol_manager
        .stop(protocol_event_receiver, protocol_pool_event_receiver)
        .await
        .expect("Failed to shutdown protocol.");
}

#[tokio::test]
#[serial]
async fn test_protocol_bans_all_nodes_propagating_an_attack_attempt() {
    let protocol_config = tools::create_protocol_config();

    let (mut network_controller, network_command_sender, network_event_receiver) =
        MockNetworkController::new();

    // start protocol controller
    let (
        mut protocol_command_sender,
        mut protocol_event_receiver,
        protocol_pool_event_receiver,
        protocol_manager,
    ) = start_protocol_controller(
        protocol_config.clone(),
        5u64,
        network_command_sender,
        network_event_receiver,
    )
    .await
    .expect("could not start protocol controller");

    // Create 4 nodes.
    let nodes = tools::create_and_connect_nodes(4, &mut network_controller).await;

    // Create a block coming from one node.
    let block = tools::create_block(&nodes[0].private_key, &nodes[0].id.0);

    let expected_hash = block
        .header
        .compute_block_id()
        .expect("Failed to compute hash.");

    // Propagate the block via 4 nodes.
    for creator_node in nodes.iter() {
        // Send block to protocol.
        network_controller
            .send_header(creator_node.id, block.header.clone())
            .await;

        // Check protocol sends header to consensus.
        let received_hash =
            match tools::wait_protocol_event(&mut protocol_event_receiver, 1000.into(), |evt| {
                match evt {
                    evt @ ProtocolEvent::ReceivedBlockHeader { .. } => Some(evt),
                    _ => None,
                }
            })
            .await
            {
                Some(ProtocolEvent::ReceivedBlockHeader { block_id, .. }) => block_id,
                _ => panic!("Unexpected or no protocol event."),
            };

        // Check that protocol sent the right header to consensus.
        assert_eq!(expected_hash, received_hash);
    }

    // Have one node send that they don't know about the block.
    let not_banned_nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;
    network_controller
        .send_block_not_found(not_banned_nodes[0].id, expected_hash)
        .await;

    // Simulate consensus notifying an attack attempt.
    protocol_command_sender
        .notify_block_attack(expected_hash)
        .await
        .expect("Failed to ask for block.");

    // Make sure all nodes are banned.
    let node_ids = nodes.into_iter().map(|node_info| node_info.id).collect();
    tools::assert_banned_nodes(node_ids, &mut network_controller).await;

    // Make sure protocol did not ban the node that did not know about the block.
    let ban_cmd_filter = |cmd| match cmd {
        cmd @ NetworkCommand::Ban { .. } => Some(cmd),
        _ => None,
    };
    let got_more_commands = network_controller
        .wait_command(100.into(), ban_cmd_filter)
        .await;
    assert!(
        got_more_commands.is_none(),
        "unexpected command {:?}",
        got_more_commands
    );

    protocol_manager
        .stop(protocol_event_receiver, protocol_pool_event_receiver)
        .await
        .expect("Failed to shutdown protocol.");
}

#[tokio::test]
#[serial]
async fn test_protocol_removes_banned_node_on_disconnection() {
    let protocol_config = tools::create_protocol_config();

    let (mut network_controller, network_command_sender, network_event_receiver) =
        MockNetworkController::new();

    // start protocol controller
    let (_, mut protocol_event_receiver, protocol_pool_event_receiver, protocol_manager) =
        start_protocol_controller(
            protocol_config.clone(),
            5u64,
            network_command_sender,
            network_event_receiver,
        )
        .await
        .expect("could not start protocol controller");

    let mut nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

    let creator_node = nodes.pop().expect("Failed to get node info.");

    // Get the node banned.
    let mut block = tools::create_block(&creator_node.private_key, &creator_node.id.0);
    block.header.content.slot = Slot::new(1, 1);
    network_controller
        .send_header(creator_node.id, block.header)
        .await;
    tools::assert_banned_node(creator_node.id, &mut network_controller).await;

    // Close the connection.
    network_controller.close_connection(creator_node.id).await;

    // Re-connect the node.
    network_controller.new_connection(creator_node.id).await;

    // The node is not banned anymore.
    let block = tools::create_block(&creator_node.private_key, &creator_node.id.0);
    network_controller
        .send_header(creator_node.id, block.header.clone())
        .await;

    // Check protocol sends header to consensus.
    let received_hash =
        match tools::wait_protocol_event(&mut protocol_event_receiver, 1000.into(), |evt| match evt
        {
            evt @ ProtocolEvent::ReceivedBlockHeader { .. } => Some(evt),
            _ => None,
        })
        .await
        {
            Some(ProtocolEvent::ReceivedBlockHeader { block_id, .. }) => block_id,
            _ => panic!("Unexpected or no protocol event."),
        };

    // Check that protocol sent the right header to consensus.
    let expected_hash = block
        .header
        .compute_block_id()
        .expect("Failed to compute hash.");
    assert_eq!(expected_hash, received_hash);

    protocol_manager
        .stop(protocol_event_receiver, protocol_pool_event_receiver)
        .await
        .expect("Failed to shutdown protocol.");
}
