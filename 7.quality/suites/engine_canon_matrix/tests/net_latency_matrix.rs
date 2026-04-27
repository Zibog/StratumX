mod common;
use common::*;

#[test]
fn net_latency_bucket_case_0() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(10);
    s.record_sample(60);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_1() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(11);
    s.record_sample(61);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_2() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(12);
    s.record_sample(62);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_3() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(13);
    s.record_sample(63);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_4() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(14);
    s.record_sample(64);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_5() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(15);
    s.record_sample(65);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_6() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(16);
    s.record_sample(66);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_7() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(17);
    s.record_sample(67);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_8() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(18);
    s.record_sample(68);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_9() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(19);
    s.record_sample(69);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_10() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(20);
    s.record_sample(70);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_11() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(21);
    s.record_sample(71);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_12() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(22);
    s.record_sample(72);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_13() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(23);
    s.record_sample(73);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_14() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(24);
    s.record_sample(74);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_15() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(25);
    s.record_sample(75);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_16() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(26);
    s.record_sample(76);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_17() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(27);
    s.record_sample(77);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_18() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(28);
    s.record_sample(78);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_19() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(29);
    s.record_sample(79);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_20() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(30);
    s.record_sample(80);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_21() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(31);
    s.record_sample(81);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_22() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(32);
    s.record_sample(82);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_23() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(33);
    s.record_sample(83);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_24() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(34);
    s.record_sample(84);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_25() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(35);
    s.record_sample(85);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_26() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(36);
    s.record_sample(86);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_27() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(37);
    s.record_sample(87);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_28() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(38);
    s.record_sample(88);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_29() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(39);
    s.record_sample(89);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_reconcile_case_0() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(0),
        predicted_tick: Tick(1),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_1() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(1),
        predicted_tick: Tick(2),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_2() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(2),
        predicted_tick: Tick(3),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_3() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(3),
        predicted_tick: Tick(4),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_4() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(4),
        predicted_tick: Tick(5),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_5() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(5),
        predicted_tick: Tick(6),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_6() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(6),
        predicted_tick: Tick(7),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_7() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(7),
        predicted_tick: Tick(8),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_8() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(8),
        predicted_tick: Tick(9),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_9() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(9),
        predicted_tick: Tick(10),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_10() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(10),
        predicted_tick: Tick(11),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_11() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(11),
        predicted_tick: Tick(12),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_12() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(12),
        predicted_tick: Tick(13),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_13() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(13),
        predicted_tick: Tick(14),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_14() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(14),
        predicted_tick: Tick(15),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_15() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(15),
        predicted_tick: Tick(16),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_16() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(16),
        predicted_tick: Tick(17),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_17() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(17),
        predicted_tick: Tick(18),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_18() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(18),
        predicted_tick: Tick(19),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_19() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(19),
        predicted_tick: Tick(20),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_20() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(20),
        predicted_tick: Tick(21),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_21() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(21),
        predicted_tick: Tick(22),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_22() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(22),
        predicted_tick: Tick(23),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_23() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(23),
        predicted_tick: Tick(24),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_24() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(24),
        predicted_tick: Tick(25),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_25() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(25),
        predicted_tick: Tick(26),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_26() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(26),
        predicted_tick: Tick(27),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_27() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(27),
        predicted_tick: Tick(28),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_28() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(28),
        predicted_tick: Tick(29),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
#[test]
fn net_latency_reconcile_case_29() {
    let s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    let r = s.reconcile(PredictionContext {
        authoritative_tick: Tick(29),
        predicted_tick: Tick(30),
    });
    assert!(r.rewound);
    assert_eq!(r.delta_ticks, 1);
}
