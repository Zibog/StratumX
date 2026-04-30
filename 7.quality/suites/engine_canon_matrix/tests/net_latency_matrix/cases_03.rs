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
