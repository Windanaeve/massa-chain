// Copyright (c) 2021 MASSA LABS <info@massa.net>

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use bitvec::prelude::*;
use consensus::ledger::LedgerChanges;
use tokio::{sync::mpsc::Receiver, time::sleep};

use communication::network::{BootstrapPeers, NetworkCommand};
use consensus::{
    BootstrapableGraph, ConsensusCommand, ExportActiveBlock, ExportProofOfStake, LedgerSubset,
    RollCounts, RollUpdate, RollUpdates, ThreadCycleState,
};
use crypto::hash::Hash;
use crypto::signature::{derive_public_key, generate_random_private_key, PrivateKey, PublicKey};
use models::clique::Clique;
use models::ledger::LedgerChange;
use models::ledger::LedgerData;
use models::{
    Address, Amount, Block, BlockHeader, BlockHeaderContent, BlockId, DeserializeCompact,
    Endorsement, EndorsementContent, Operation, OperationContent, SerializeCompact, Slot,
};
use time::UTime;

use super::mock_establisher::Duplex;
use crate::config::BootstrapConfig;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

pub const BASE_BOOTSTRAP_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(169, 202, 0, 10));

pub fn get_dummy_block_id(s: &str) -> BlockId {
    BlockId(Hash::hash(s.as_bytes()))
}

pub fn get_random_public_key() -> PublicKey {
    let priv_key = crypto::generate_random_private_key();
    crypto::derive_public_key(&priv_key)
}

pub fn get_random_address() -> Address {
    let priv_key = crypto::generate_random_private_key();
    let pub_key = crypto::derive_public_key(&priv_key);
    Address::from_public_key(&pub_key).unwrap()
}

pub fn get_dummy_signature(s: &str) -> crypto::signature::Signature {
    let priv_key = crypto::generate_random_private_key();
    crypto::sign(&Hash::hash(&s.as_bytes()), &priv_key).unwrap()
}

pub fn get_bootstrap_config(bootstrap_public_key: PublicKey) -> BootstrapConfig {
    // Init the serialization context with a default,
    // can be overwritten with a more specific one in the test.
    models::init_serialization_context(models::SerializationContext {
        max_block_operations: 1024,
        parent_count: 2,
        max_peer_list_length: 128,
        max_message_size: 3 * 1024 * 1024,
        max_block_size: 3 * 1024 * 1024,
        max_bootstrap_blocks: 100,
        max_bootstrap_cliques: 100,
        max_bootstrap_deps: 100,
        max_bootstrap_children: 100,
        max_ask_blocks_per_message: 10,
        max_operations_per_message: 1024,
        max_endorsements_per_message: 1024,
        max_bootstrap_message_size: 100000000,
        max_bootstrap_pos_entries: 1000,
        max_bootstrap_pos_cycles: 5,
        max_block_endorsments: 8,
    });

    BootstrapConfig {
        bind: Some("0.0.0.0:31244".parse().unwrap()),
        connect_timeout: 200.into(),
        retry_delay: 200.into(),
        max_bootstrap_blocks: 100,
        max_bootstrap_cliques: 100,
        max_bootstrap_deps: 100,
        max_bootstrap_children: 100,
        max_ping: UTime::from(500),
        max_bootstrap_message_size: 100000000,
        read_timeout: 1000.into(),
        write_timeout: 1000.into(),
        max_bootstrap_pos_entries: 1000,
        max_bootstrap_pos_cycles: 5,
        bootstrap_list: vec![(SocketAddr::new(BASE_BOOTSTRAP_IP, 16), bootstrap_public_key)],
        enable_clock_synchronization: true,
        cache_duration: 10000.into(),
        max_simultaneous_bootstraps: 2,
        ip_list_max_size: 10,
        per_ip_min_interval: 10000.into(),
    }
}

pub fn get_keys() -> (PrivateKey, PublicKey) {
    let private_key = generate_random_private_key();
    let public_key = derive_public_key(&private_key);
    (private_key, public_key)
}

pub async fn wait_consensus_command<F, T>(
    consensus_command_receiver: &mut Receiver<ConsensusCommand>,
    timeout: UTime,
    filter_map: F,
) -> Option<T>
where
    F: Fn(ConsensusCommand) -> Option<T>,
{
    let timer = sleep(timeout.into());
    tokio::pin!(timer);
    loop {
        tokio::select! {
            cmd = consensus_command_receiver.recv() => match cmd {
                Some(orig_evt) => if let Some(res_evt) = filter_map(orig_evt) { return Some(res_evt); },
                _ => panic!("network event channel died")
            },
            _ = &mut timer => return None
        }
    }
}

pub async fn wait_network_command<F, T>(
    network_command_receiver: &mut Receiver<NetworkCommand>,
    timeout: UTime,
    filter_map: F,
) -> Option<T>
where
    F: Fn(NetworkCommand) -> Option<T>,
{
    let timer = sleep(timeout.into());
    tokio::pin!(timer);
    loop {
        tokio::select! {
            cmd = network_command_receiver.recv() => match cmd {
                Some(orig_evt) => if let Some(res_evt) = filter_map(orig_evt) { return Some(res_evt); },
                _ => panic!("network event channel died")
            },
            _ = &mut timer => return None
        }
    }
}

/// asserts that two ExportProofOfStake are equal
pub fn assert_eq_thread_cycle_states(v1: &ExportProofOfStake, v2: &ExportProofOfStake) {
    assert_eq!(
        v1.cycle_states.len(),
        v2.cycle_states.len(),
        "length mismatch between sent and received pos"
    );
    for (itm1, itm2) in v1.cycle_states.iter().zip(v2.cycle_states.iter()) {
        assert_eq!(
            itm1.len(),
            itm2.len(),
            "subitem length mismatch between sent and received pos"
        );
        for (itm1, itm2) in itm1.iter().zip(itm2.iter()) {
            assert_eq!(
                itm1.cycle, itm2.cycle,
                "ThreadCycleState.cycle mismatch between sent and received pos"
            );
            assert_eq!(
                itm1.last_final_slot, itm2.last_final_slot,
                "ThreadCycleState.last_final_slot mismatch between sent and received pos"
            );
            assert_eq!(
                itm1.roll_count.0, itm2.roll_count.0,
                "ThreadCycleState.roll_count mismatch between sent and received pos"
            );
            assert_eq!(
                itm1.cycle_updates.0.len(),
                itm2.cycle_updates.0.len(),
                "ThreadCycleState.cycle_updates.len() mismatch between sent and received pos"
            );
            for (a1, itm1) in itm1.cycle_updates.0.iter() {
                let itm2 = itm2.cycle_updates.0.get(a1).expect(
                    "ThreadCycleState.cycle_updates element miss between sent and received pos",
                );
                assert_eq!(
                    itm1.to_bytes_compact().unwrap(),
                    itm2.to_bytes_compact().unwrap(),
                    "ThreadCycleState.cycle_updates item mismatch between sent and received pos"
                );
            }
            assert_eq!(
                itm1.rng_seed, itm2.rng_seed,
                "ThreadCycleState.rng_seed mismatch between sent and received pos"
            );
            assert_eq!(
                itm1.production_stats, itm2.production_stats,
                "ThreadCycleState.production_stats mismatch between sent and received pos"
            );
        }
    }
}

/// asserts that two BootstrapableGraph are equal
pub fn assert_eq_bootstrap_graph(v1: &BootstrapableGraph, v2: &BootstrapableGraph) {
    assert_eq!(
        v1.active_blocks.len(),
        v2.active_blocks.len(),
        "length mismatch"
    );
    for (id1, itm1) in v1.active_blocks.iter() {
        let itm2 = v2.active_blocks.get(&id1).unwrap();
        assert_eq!(
            itm1.block.to_bytes_compact().unwrap(),
            itm2.block.to_bytes_compact().unwrap(),
            "block mismatch"
        );
        assert_eq!(
            itm1.block_ledger_changes.0.len(),
            itm2.block_ledger_changes.0.len(),
            "ledger changes length mismatch"
        );
        for (id1, itm1) in itm1.block_ledger_changes.0.iter() {
            let itm2 = itm2.block_ledger_changes.0.get(id1).unwrap();
            assert_eq!(
                itm1.balance_delta, itm2.balance_delta,
                "balance delta mistmatch"
            );
            assert_eq!(
                itm1.balance_increment, itm2.balance_increment,
                "balance increment mismatch"
            );
        }
        assert_eq!(itm1.children, itm2.children, "children mismatch");
        assert_eq!(
            itm1.dependencies, itm2.dependencies,
            "dependencies mismatch"
        );
        assert_eq!(itm1.is_final, itm2.is_final, "is_final mismatch");
        assert_eq!(itm1.parents, itm2.parents, "parents mismatch");
        assert_eq!(
            itm1.production_events, itm2.production_events,
            "production events mismatch"
        );
        assert_eq!(
            itm1.roll_updates.0.len(),
            itm2.roll_updates.0.len(),
            "roll updates len mismatch"
        );
        for (id1, itm1) in itm1.roll_updates.0.iter() {
            let itm2 = itm2.roll_updates.0.get(id1).unwrap();
            assert_eq!(
                itm1.roll_purchases, itm2.roll_purchases,
                "roll purchases mistmatch"
            );
            assert_eq!(itm1.roll_sales, itm2.roll_sales, "roll sales mismatch");
        }
    }
    assert_eq!(v1.best_parents, v2.best_parents, "best parents mismatch");
    assert_eq!(v1.gi_head, v2.gi_head, "gi_head mismatch");
    assert_eq!(
        v1.latest_final_blocks_periods, v2.latest_final_blocks_periods,
        "latest_final_blocks_periods mismatch"
    );
    assert_eq!(v1.ledger.0.len(), v1.ledger.0.len(), "ledger len mismatch");
    for (id1, itm1) in v1.ledger.0.iter() {
        let itm2 = v2.ledger.0.get(id1).unwrap();
        assert_eq!(itm1.balance, itm2.balance, "balance mistmatch");
    }
    assert_eq!(
        v1.max_cliques.len(),
        v2.max_cliques.len(),
        "max_cliques len mismatch"
    );
    for (itm1, itm2) in v1.max_cliques.iter().zip(v2.max_cliques.iter()) {
        assert_eq!(itm1.block_ids, itm2.block_ids, "block_ids mistmatch");
        assert_eq!(itm1.fitness, itm2.fitness, "fitness mistmatch");
        assert_eq!(
            itm1.is_blockclique, itm2.is_blockclique,
            "is_blockclique mistmatch"
        );
    }
}

pub fn get_boot_state() -> (ExportProofOfStake, BootstrapableGraph) {
    let private_key = crypto::generate_random_private_key();
    let public_key = crypto::derive_public_key(&private_key);
    let address = Address::from_public_key(&public_key).unwrap();

    let mut ledger_subset = LedgerSubset::default();
    ledger_subset.0.insert(
        address,
        LedgerData {
            balance: Amount::from_str("10").unwrap(),
        },
    );

    let cycle_state = ThreadCycleState {
        cycle: 1,
        last_final_slot: Slot::new(1, 1),
        roll_count: RollCounts(
            vec![(get_random_address(), 123), (get_random_address(), 456)]
                .into_iter()
                .collect(),
        ),
        cycle_updates: RollUpdates(
            vec![
                (
                    get_random_address(),
                    RollUpdate {
                        roll_purchases: 147,
                        roll_sales: 44788,
                    },
                ),
                (
                    get_random_address(),
                    RollUpdate {
                        roll_purchases: 8887,
                        roll_sales: 114,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        ),
        rng_seed: bitvec![Lsb0, u8 ; 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1],
        production_stats: vec![
            (get_random_address(), (1, 2)),
            (get_random_address(), (3, 4)),
        ]
        .into_iter()
        .collect(),
    };
    let boot_pos = ExportProofOfStake {
        cycle_states: vec![
            vec![cycle_state.clone()].into_iter().collect(),
            vec![cycle_state.clone()].into_iter().collect(),
        ],
    };

    let block1 = ExportActiveBlock {
        block: Block {
            header: BlockHeader {
                content: BlockHeaderContent {
                    creator: get_random_public_key(),
                    slot: Slot::new(1, 1),
                    parents: vec![get_dummy_block_id("p1"), get_dummy_block_id("p2")],
                    operation_merkle_root: Hash::hash("op_hash".as_bytes()),
                    endorsements: vec![
                        Endorsement {
                            content: EndorsementContent {
                                sender_public_key: get_random_public_key(),
                                slot: Slot::new(1, 0),
                                index: 1,
                                endorsed_block: get_dummy_block_id("p1"),
                            },
                            signature: get_dummy_signature("dummy_sig_0"),
                        },
                        Endorsement {
                            content: EndorsementContent {
                                sender_public_key: get_random_public_key(),
                                slot: Slot::new(4, 1),
                                index: 3,
                                endorsed_block: get_dummy_block_id("p1"),
                            },
                            signature: get_dummy_signature("dummy_sig_00"),
                        },
                    ],
                },
                signature: get_dummy_signature("dummy_sig_1"),
            },
            operations: vec![
                Operation {
                    content: OperationContent {
                        sender_public_key: get_random_public_key(),
                        fee: Amount::from_str("1524878").unwrap(),
                        expire_period: 5787899,
                        op: models::OperationType::Transaction {
                            recipient_address: get_random_address(),
                            amount: Amount::from_str("1259787").unwrap(),
                        },
                    },
                    signature: get_dummy_signature("dummy_sig_2"),
                },
                Operation {
                    content: OperationContent {
                        sender_public_key: get_random_public_key(),
                        fee: Amount::from_str("878763222").unwrap(),
                        expire_period: 4557887,
                        op: models::OperationType::RollBuy { roll_count: 45544 },
                    },
                    signature: get_dummy_signature("dummy_sig_3"),
                },
                Operation {
                    content: OperationContent {
                        sender_public_key: get_random_public_key(),
                        fee: Amount::from_str("4545").unwrap(),
                        expire_period: 452524,
                        op: models::OperationType::RollSell {
                            roll_count: 4888787,
                        },
                    },
                    signature: get_dummy_signature("dummy_sig_4"),
                },
            ],
        },
        parents: vec![
            (get_dummy_block_id("b1"), 4777),
            (get_dummy_block_id("b2"), 8870),
        ],
        children: vec![
            vec![
                (get_dummy_block_id("b3"), 101),
                (get_dummy_block_id("b4"), 455),
            ]
            .into_iter()
            .collect(),
            vec![(get_dummy_block_id("b3_2"), 889)]
                .into_iter()
                .collect(),
        ],
        dependencies: vec![get_dummy_block_id("b5"), get_dummy_block_id("b6")]
            .into_iter()
            .collect(),
        is_final: true,
        block_ledger_changes: LedgerChanges(
            vec![
                (
                    get_random_address(),
                    LedgerChange {
                        balance_increment: true,
                        balance_delta: Amount::from_str("157").unwrap(),
                    },
                ),
                (
                    get_random_address(),
                    LedgerChange {
                        balance_increment: false,
                        balance_delta: Amount::from_str("44").unwrap(),
                    },
                ),
                (
                    get_random_address(),
                    LedgerChange {
                        balance_increment: false,
                        balance_delta: Amount::from_str("878").unwrap(),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        ),
        roll_updates: RollUpdates(
            vec![
                (
                    get_random_address(),
                    RollUpdate {
                        roll_purchases: 778,
                        roll_sales: 54851,
                    },
                ),
                (
                    get_random_address(),
                    RollUpdate {
                        roll_purchases: 788778,
                        roll_sales: 11451,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        ),
        production_events: vec![
            (12, get_random_address(), true),
            (31, get_random_address(), false),
        ],
    };

    // check reserialization
    assert_eq_thread_cycle_states(
        &ExportProofOfStake::from_bytes_compact(&boot_pos.to_bytes_compact().unwrap())
            .unwrap()
            .0,
        &boot_pos,
    );

    let boot_graph = BootstrapableGraph {
        active_blocks: vec![(get_dummy_block_id("block1"), block1)]
            .into_iter()
            .collect(),
        best_parents: vec![
            (get_dummy_block_id("parent1"), 2),
            (get_dummy_block_id("parent2"), 3),
        ],
        latest_final_blocks_periods: vec![
            (get_dummy_block_id("parent1"), 10),
            (get_dummy_block_id("parent2"), 10),
        ],
        gi_head: vec![
            (get_dummy_block_id("parent1"), Default::default()),
            (get_dummy_block_id("parent2"), Default::default()),
        ]
        .into_iter()
        .collect(),
        max_cliques: vec![Clique {
            block_ids: vec![get_dummy_block_id("parent1"), get_dummy_block_id("parent2")]
                .into_iter()
                .collect(),
            fitness: 123,
            is_blockclique: true,
        }],
        ledger: ledger_subset,
    };

    assert_eq_bootstrap_graph(
        &BootstrapableGraph::from_bytes_compact(&boot_graph.to_bytes_compact().unwrap())
            .unwrap()
            .0,
        &boot_graph,
    );

    (boot_pos, boot_graph)
}

pub fn get_peers() -> BootstrapPeers {
    BootstrapPeers(vec![
        "82.245.123.77".parse().unwrap(),
        "82.220.123.78".parse().unwrap(),
    ])
}

pub async fn bridge_mock_streams(mut side1: Duplex, mut side2: Duplex) {
    let mut buf1 = vec![0u8; 1024];
    let mut buf2 = vec![0u8; 1024];
    loop {
        tokio::select! {
            res1 = side1.read(&mut buf1) => match res1 {
                Ok(n1) => {
                    if n1 == 0 {
                        return;
                    }
                    if side2.write_all(&buf1[..n1]).await.is_err() {
                        return;
                    }
                },
                Err(_err) => {
                    return;
                }
            },
            res2 = side2.read(&mut buf2) => match res2 {
                Ok(n2) => {
                    if n2 == 0 {
                        return;
                    }
                    if side1.write_all(&buf2[..n2]).await.is_err() {
                        return;
                    }
                },
                Err(_err) => {
                    return;
                }
            },
        }
    }
}
