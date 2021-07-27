// Copyright (c) 2021 MASSA LABS <info@massa.net>
use communication::network::{NetworkCommand, NetworkCommandSender};
use consensus::{ConsensusCommand, ConsensusCommandSender};
use models::SerializeCompact;
use serial_test::serial;
use std::str::FromStr;
use tokio::sync::mpsc;

use crate::{get_state, start_bootstrap_server};

use super::{
    mock_establisher,
    tools::{
        bridge_mock_streams, get_boot_state, get_bootstrap_config, get_keys, get_peers,
        wait_consensus_command, wait_network_command,
    },
};

#[tokio::test]
#[serial]
async fn test_bootstrap_server() {
    let (private_key, public_key) = get_keys();
    let cfg = get_bootstrap_config(public_key);

    let (consensus_cmd_tx, mut consensus_cmd_rx) = mpsc::channel::<ConsensusCommand>(5);
    let (network_cmd_tx, mut network_cmd_rx) = mpsc::channel::<NetworkCommand>(5);

    let (bootstrap_establisher, bootstrap_interface) = mock_establisher::new();
    let bootstrap_manager = start_bootstrap_server(
        ConsensusCommandSender(consensus_cmd_tx),
        NetworkCommandSender(network_cmd_tx),
        cfg.clone(),
        bootstrap_establisher,
        private_key,
        0,
        Default::default(),
    )
    .await
    .unwrap()
    .unwrap();

    // launch the get_state process
    let (remote_establisher, mut remote_interface) = mock_establisher::new();
    let cfg_copy = cfg.clone();
    let get_state_h = tokio::spawn(async move {
        get_state(cfg_copy, remote_establisher, Default::default())
            .await
            .unwrap()
    });

    // accept connection attempt from remote
    let (remote_r, remote_w, conn_addr, resp) = tokio::time::timeout(
        std::time::Duration::from_millis(1000),
        remote_interface.wait_connection_attempt_from_controller(),
    )
    .await
    .expect("timeout waiting for connection attempt from remote")
    .expect("error receiving connection attempt from remote");
    let expect_conn_addr = cfg.bootstrap_list[0].0.clone();
    assert_eq!(
        conn_addr, expect_conn_addr,
        "client connected to wrong bootstrap ip"
    );
    resp.send(true)
        .expect("could not send connection accept to remote");

    // connect to bootstrap
    let remote_addr = std::net::SocketAddr::from_str("82.245.72.98:10000").unwrap(); // not checked
    let (bootstrap_r, bootstrap_w) = tokio::time::timeout(
        std::time::Duration::from_millis(1000),
        bootstrap_interface.connect_to_controller(&remote_addr),
    )
    .await
    .expect("timeout while connecting to bootstrap")
    .expect("could not connect to bootstrap");

    // launch bridges
    let bridge_h1 = tokio::spawn(async move {
        bridge_mock_streams(remote_r, bootstrap_w).await;
    });
    let bridge_h2 = tokio::spawn(async move {
        bridge_mock_streams(bootstrap_r, remote_w).await;
    });

    // wait for bootstrap to ask network for peers, send them
    let response = match wait_network_command(&mut network_cmd_rx, 1000.into(), |cmd| match cmd {
        NetworkCommand::GetBootstrapPeers(resp) => Some(resp),
        _ => None,
    })
    .await
    {
        Some(resp) => resp,
        None => panic!("timeout waiting for get peers command"),
    };
    let sent_peers = get_peers();
    response.send(sent_peers.clone()).unwrap();

    // wait for bootstrap to ask consensus for bootstrap graph, send it
    let response = match wait_consensus_command(&mut consensus_cmd_rx, 1000.into(), |cmd| match cmd
    {
        ConsensusCommand::GetBootstrapState(resp) => Some(resp),
        _ => None,
    })
    .await
    {
        Some(resp) => resp,
        None => panic!("timeout waiting for get boot graph consensus command"),
    };
    let (sent_pos, sent_graph) = get_boot_state();
    response
        .send((sent_pos.clone(), sent_graph.clone()))
        .unwrap();

    // wait for get_state
    let (maybe_recv_pos, maybe_recv_graph, _comp, maybe_recv_peers) = get_state_h
        .await
        .expect("error while waiting for get_state to finish");

    // wait for bridges
    bridge_h1.await.expect("bridge 1 join failed");
    bridge_h2.await.expect("bridge 2 join failed");

    // check states
    let recv_pos = maybe_recv_pos.unwrap();
    assert_eq!(
        sent_pos.to_bytes_compact().unwrap(),
        recv_pos.to_bytes_compact().unwrap(),
        "mismatch between sent and received pos"
    );
    let recv_graph = maybe_recv_graph.unwrap();
    assert_eq!(
        sent_graph.to_bytes_compact().unwrap(),
        recv_graph.to_bytes_compact().unwrap(),
        "mismatch between sent and received graphs"
    );

    // check peers
    let recv_peers = maybe_recv_peers.unwrap();
    assert_eq!(
        sent_peers.to_bytes_compact().unwrap(),
        recv_peers.to_bytes_compact().unwrap(),
        "mismatch between sent and received peers"
    );

    // stop bootstrap server
    bootstrap_manager
        .stop()
        .await
        .expect("could not stop bootstrap server");
}
