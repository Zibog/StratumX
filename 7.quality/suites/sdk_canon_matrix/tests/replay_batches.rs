// SDK Canon Matrix - replay batches test

mod common;
use common::*;
use stratumx_test_support::*;

fn test_replay_batch(case: usize) {
    let (mut runtime, session, object) = seed_runtime(case);

    runtime
        .ingest_packet(BridgePacket {
            session,
            sequence: 1,
            lane: PacketLane::Diagnostics,
            topic: "observe".into(),
            payload: vec![1; 8 + (case % 24)],
        })
        .unwrap();

    runtime
        .apply_control(BridgeControl {
            session,
            sequence: 2,
            object: Some(object),
            kind: BridgeControlKind::AddTag {
                tag: format!("tag-{case}"),
            },
        })
        .unwrap();

    runtime.record_metric("sdk.case", case as f64, "count");
    runtime.publish_snapshot(format!("replay-{case}")).unwrap();

    let first = runtime.read_observation_batch(0, 2);
    let second = runtime.read_observation_batch(first.next_cursor, 32);

    assert!(!first.records.is_empty());
    assert!(second.next_cursor >= first.next_cursor);

    let metrics = runtime.read_metric_batch(0, 64);
    assert!(metrics
        .records
        .iter()
        .any(|metric| metric.name == "sdk.case"));
}

#[test]
fn replay_batches_0() {
    test_replay_batch(0);
}

#[test]
fn replay_batches_1() {
    test_replay_batch(1);
}

#[test]
fn replay_batches_2() {
    test_replay_batch(2);
}

#[test]
fn replay_batches_3() {
    test_replay_batch(3);
}

#[test]
fn replay_batches_4() {
    test_replay_batch(4);
}

#[test]
fn replay_batches_5() {
    test_replay_batch(5);
}

#[test]
fn replay_batches_6() {
    test_replay_batch(6);
}

#[test]
fn replay_batches_7() {
    test_replay_batch(7);
}

#[test]
fn replay_batches_8() {
    test_replay_batch(8);
}

#[test]
fn replay_batches_9() {
    test_replay_batch(9);
}

#[test]
fn replay_batches_10() {
    test_replay_batch(10);
}

#[test]
fn replay_batches_11() {
    test_replay_batch(11);
}

#[test]
fn replay_batches_12() {
    test_replay_batch(12);
}

#[test]
fn replay_batches_13() {
    test_replay_batch(13);
}

#[test]
fn replay_batches_14() {
    test_replay_batch(14);
}

#[test]
fn replay_batches_15() {
    test_replay_batch(15);
}

#[test]
fn replay_batches_16() {
    test_replay_batch(16);
}

#[test]
fn replay_batches_17() {
    test_replay_batch(17);
}

#[test]
fn replay_batches_18() {
    test_replay_batch(18);
}

#[test]
fn replay_batches_19() {
    test_replay_batch(19);
}

#[test]
fn replay_batches_20() {
    test_replay_batch(20);
}

#[test]
fn replay_batches_21() {
    test_replay_batch(21);
}

#[test]
fn replay_batches_22() {
    test_replay_batch(22);
}

#[test]
fn replay_batches_23() {
    test_replay_batch(23);
}

#[test]
fn replay_batches_24() {
    test_replay_batch(24);
}

#[test]
fn replay_batches_25() {
    test_replay_batch(25);
}
