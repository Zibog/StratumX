use engine_handle_refs::RuntimeHandle;
use link_egress_metrics::*;
use sdk_compat::CompatibilityProfile;

// ============================================================================
// Constants
// ============================================================================

#[test]
fn test_max_batch_records_constant() {
    assert_eq!(MAX_BATCH_RECORDS, 256);
}

// ============================================================================
// MetricBatchId Tests
// ============================================================================

#[test]
fn test_metric_batch_id_new() {
    let id = MetricBatchId(42);
    assert_eq!(id.0, 42);
}

#[test]
fn test_metric_batch_id_equality() {
    let a = MetricBatchId(1);
    let b = MetricBatchId(1);
    assert_eq!(a, b);
}

#[test]
fn test_metric_batch_id_ordering() {
    let a = MetricBatchId(1);
    let b = MetricBatchId(2);
    assert!(a < b);
}

#[test]
fn test_metric_batch_id_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let id = MetricBatchId(100);
    let mut h1 = DefaultHasher::new();
    id.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    id.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ============================================================================
// MetricKind Tests
// ============================================================================

#[test]
fn test_metric_kind_counter() {
    let kind = MetricKind::Counter;
    assert!(matches!(kind, MetricKind::Counter));
}

#[test]
fn test_metric_kind_gauge() {
    let kind = MetricKind::Gauge;
    assert!(matches!(kind, MetricKind::Gauge));
}

#[test]
fn test_metric_kind_histogram() {
    let kind = MetricKind::Histogram;
    assert!(matches!(kind, MetricKind::Histogram));
}

#[test]
fn test_metric_kind_equality() {
    let a = MetricKind::Counter;
    let b = MetricKind::Counter;
    assert_eq!(a, b);
}

#[test]
fn test_metric_kind_inequality() {
    let a = MetricKind::Counter;
    let b = MetricKind::Gauge;
    assert_ne!(a, b);
}

#[test]
fn test_metric_kind_all_variants_distinct() {
    let kinds = [
        MetricKind::Counter,
        MetricKind::Gauge,
        MetricKind::Histogram,
    ];
    for i in 0..kinds.len() {
        for j in (i + 1)..kinds.len() {
            assert_ne!(kinds[i], kinds[j]);
        }
    }
}

// ============================================================================
// MetricQualityFlag Tests
// ============================================================================

#[test]
fn test_quality_flag_full() {
    let flag = MetricQualityFlag::Full;
    assert!(matches!(flag, MetricQualityFlag::Full));
}

#[test]
fn test_quality_flag_degraded() {
    let flag = MetricQualityFlag::Degraded;
    assert!(matches!(flag, MetricQualityFlag::Degraded));
}

#[test]
fn test_quality_flag_equality() {
    let a = MetricQualityFlag::Full;
    let b = MetricQualityFlag::Full;
    assert_eq!(a, b);
}

#[test]
fn test_quality_flag_inequality() {
    let a = MetricQualityFlag::Full;
    let b = MetricQualityFlag::Degraded;
    assert_ne!(a, b);
}

// ============================================================================
// MetricRecord Tests
// ============================================================================

#[test]
fn test_metric_record_creation() {
    let record = MetricRecord {
        cursor: 1,
        epoch: 1000,
        name: "fps".to_string(),
        value: 60.0,
        unit: "frames/sec".to_string(),
        kind: MetricKind::Gauge,
    };
    assert_eq!(record.cursor, 1);
    assert_eq!(record.epoch, 1000);
    assert_eq!(record.name, "fps");
    assert_eq!(record.value, 60.0);
    assert_eq!(record.unit, "frames/sec");
    assert!(matches!(record.kind, MetricKind::Gauge));
}

#[test]
fn test_metric_record_counter() {
    let record = MetricRecord {
        cursor: 2,
        epoch: 2000,
        name: "total_requests".to_string(),
        value: 1234.0,
        unit: "count".to_string(),
        kind: MetricKind::Counter,
    };
    assert!(matches!(record.kind, MetricKind::Counter));
}

#[test]
fn test_metric_record_histogram() {
    let record = MetricRecord {
        cursor: 3,
        epoch: 3000,
        name: "latency_p99".to_string(),
        value: 150.5,
        unit: "ms".to_string(),
        kind: MetricKind::Histogram,
    };
    assert!(matches!(record.kind, MetricKind::Histogram));
}

#[test]
fn test_metric_record_equality() {
    let a = MetricRecord {
        cursor: 1,
        epoch: 100,
        name: "metric".to_string(),
        value: 42.0,
        unit: "unit".to_string(),
        kind: MetricKind::Counter,
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_metric_record_inequality() {
    let a = MetricRecord {
        cursor: 1,
        epoch: 100,
        name: "metric_a".to_string(),
        value: 42.0,
        unit: "unit".to_string(),
        kind: MetricKind::Counter,
    };
    let b = MetricRecord {
        cursor: 1,
        epoch: 100,
        name: "metric_b".to_string(),
        value: 42.0,
        unit: "unit".to_string(),
        kind: MetricKind::Counter,
    };
    assert_ne!(a, b);
}

#[test]
fn test_metric_record_zero_values() {
    let record = MetricRecord {
        cursor: 0,
        epoch: 0,
        name: "zero".to_string(),
        value: 0.0,
        unit: "".to_string(),
        kind: MetricKind::Counter,
    };
    assert_eq!(record.value, 0.0);
}

#[test]
fn test_metric_record_negative_value() {
    let record = MetricRecord {
        cursor: 1,
        epoch: 100,
        name: "delta".to_string(),
        value: -10.5,
        unit: "units".to_string(),
        kind: MetricKind::Gauge,
    };
    assert!(record.value < 0.0);
}

// ============================================================================
// MetricBatch Tests
// ============================================================================

#[test]
fn test_metric_batch_creation() {
    let rt = RuntimeHandle::new(1);
    let batch = MetricBatch {
        metric_batch_id: MetricBatchId(1),
        metric_kind_set: vec![MetricKind::Counter, MetricKind::Gauge],
        aggregation_window: 60,
        source_runtime_handle: rt,
        publication_cursor: 10,
        quality_flag: MetricQualityFlag::Full,
        profile: CompatibilityProfile::Diagnostics,
        records: vec![],
    };
    assert_eq!(batch.metric_batch_id, MetricBatchId(1));
    assert_eq!(batch.aggregation_window, 60);
    assert!(matches!(batch.quality_flag, MetricQualityFlag::Full));
    assert!(batch.records.is_empty());
}

#[test]
fn test_metric_batch_with_records() {
    let rt = RuntimeHandle::new(1);
    let records = vec![
        MetricRecord {
            cursor: 1,
            epoch: 1000,
            name: "fps".to_string(),
            value: 60.0,
            unit: "fps".to_string(),
            kind: MetricKind::Gauge,
        },
        MetricRecord {
            cursor: 2,
            epoch: 2000,
            name: "requests".to_string(),
            value: 100.0,
            unit: "count".to_string(),
            kind: MetricKind::Counter,
        },
    ];
    let batch = MetricBatch {
        metric_batch_id: MetricBatchId(2),
        metric_kind_set: vec![MetricKind::Gauge, MetricKind::Counter],
        aggregation_window: 30,
        source_runtime_handle: rt,
        publication_cursor: 2,
        quality_flag: MetricQualityFlag::Full,
        profile: CompatibilityProfile::Diagnostics,
        records,
    };
    assert_eq!(batch.records.len(), 2);
}

#[test]
fn test_metric_batch_equality() {
    let rt = RuntimeHandle::new(1);
    let a = MetricBatch {
        metric_batch_id: MetricBatchId(1),
        metric_kind_set: vec![MetricKind::Counter],
        aggregation_window: 10,
        source_runtime_handle: rt,
        publication_cursor: 0,
        quality_flag: MetricQualityFlag::Full,
        profile: CompatibilityProfile::Diagnostics,
        records: vec![],
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_metric_batch_degraded_quality() {
    let rt = RuntimeHandle::new(1);
    let batch = MetricBatch {
        metric_batch_id: MetricBatchId(3),
        metric_kind_set: vec![],
        aggregation_window: 1,
        source_runtime_handle: rt,
        publication_cursor: 0,
        quality_flag: MetricQualityFlag::Degraded,
        profile: CompatibilityProfile::Diagnostics,
        records: vec![],
    };
    assert!(matches!(batch.quality_flag, MetricQualityFlag::Degraded));
}

#[test]
fn test_metric_batch_all_profiles() {
    let rt = RuntimeHandle::new(1);
    let profiles = [
        CompatibilityProfile::ToolRuntime,
        CompatibilityProfile::EditorSurface,
        CompatibilityProfile::Automation,
        CompatibilityProfile::Diagnostics,
    ];
    for profile in profiles {
        let batch = MetricBatch {
            metric_batch_id: MetricBatchId(1),
            metric_kind_set: vec![],
            aggregation_window: 1,
            source_runtime_handle: rt,
            publication_cursor: 0,
            quality_flag: MetricQualityFlag::Full,
            profile,
            records: vec![],
        };
        assert_eq!(batch.profile, profile);
    }
}

// ============================================================================
// batch_after Function Tests
// ============================================================================

#[test]
fn test_batch_after_filters_by_cursor() {
    let rt = RuntimeHandle::new(1);
    let batch = MetricBatch {
        metric_batch_id: MetricBatchId(1),
        metric_kind_set: vec![MetricKind::Counter],
        aggregation_window: 1,
        source_runtime_handle: rt,
        publication_cursor: 3,
        quality_flag: MetricQualityFlag::Full,
        profile: CompatibilityProfile::Diagnostics,
        records: vec![
            MetricRecord {
                cursor: 1,
                epoch: 100,
                name: "a".to_string(),
                value: 1.0,
                unit: "u".to_string(),
                kind: MetricKind::Counter,
            },
            MetricRecord {
                cursor: 2,
                epoch: 200,
                name: "b".to_string(),
                value: 2.0,
                unit: "u".to_string(),
                kind: MetricKind::Counter,
            },
            MetricRecord {
                cursor: 3,
                epoch: 300,
                name: "c".to_string(),
                value: 3.0,
                unit: "u".to_string(),
                kind: MetricKind::Counter,
            },
        ],
    };

    let result = batch_after(&batch, 1, 10);
    // Should include records with cursor > 1: i.e., cursor 2 and 3
    assert_eq!(result.records.len(), 2);
    assert_eq!(result.records[0].cursor, 2);
    assert_eq!(result.records[1].cursor, 3);
}

#[test]
fn test_batch_after_empty_result() {
    let rt = RuntimeHandle::new(1);
    let batch = MetricBatch {
        metric_batch_id: MetricBatchId(1),
        metric_kind_set: vec![],
        aggregation_window: 1,
        source_runtime_handle: rt,
        publication_cursor: 5,
        quality_flag: MetricQualityFlag::Full,
        profile: CompatibilityProfile::Diagnostics,
        records: vec![MetricRecord {
            cursor: 1,
            epoch: 100,
            name: "a".to_string(),
            value: 1.0,
            unit: "u".to_string(),
            kind: MetricKind::Counter,
        }],
    };

    let result = batch_after(&batch, 10, 10);
    assert!(result.records.is_empty());
    assert_eq!(result.publication_cursor, 10); // Falls back to cursor param
}

#[test]
fn test_batch_after_respects_max_records() {
    let rt = RuntimeHandle::new(1);
    let records: Vec<MetricRecord> = (1..=10)
        .map(|i| MetricRecord {
            cursor: i as u64,
            epoch: i as u64 * 100,
            name: format!("metric_{}", i),
            value: i as f64,
            unit: "u".to_string(),
            kind: MetricKind::Counter,
        })
        .collect();

    let batch = MetricBatch {
        metric_batch_id: MetricBatchId(1),
        metric_kind_set: vec![MetricKind::Counter],
        aggregation_window: 1,
        source_runtime_handle: rt,
        publication_cursor: 10,
        quality_flag: MetricQualityFlag::Full,
        profile: CompatibilityProfile::Diagnostics,
        records,
    };

    let result = batch_after(&batch, 0, 3);
    assert_eq!(result.records.len(), 3);
}

#[test]
fn test_batch_after_clamps_max_records() {
    let rt = RuntimeHandle::new(1);
    let batch = MetricBatch {
        metric_batch_id: MetricBatchId(1),
        metric_kind_set: vec![],
        aggregation_window: 1,
        source_runtime_handle: rt,
        publication_cursor: 1,
        quality_flag: MetricQualityFlag::Full,
        profile: CompatibilityProfile::Diagnostics,
        records: vec![MetricRecord {
            cursor: 1,
            epoch: 100,
            name: "a".to_string(),
            value: 1.0,
            unit: "u".to_string(),
            kind: MetricKind::Counter,
        }],
    };

    // Request more than MAX_BATCH_RECORDS, should clamp to MAX_BATCH_RECORDS
    let result = batch_after(&batch, 0, MAX_BATCH_RECORDS + 100);
    assert!(result.records.len() <= MAX_BATCH_RECORDS);
}

#[test]
fn test_batch_after_preserves_metadata() {
    let rt = RuntimeHandle::new(1);
    let batch = MetricBatch {
        metric_batch_id: MetricBatchId(42),
        metric_kind_set: vec![MetricKind::Gauge],
        aggregation_window: 30,
        source_runtime_handle: rt,
        publication_cursor: 5,
        quality_flag: MetricQualityFlag::Degraded,
        profile: CompatibilityProfile::Automation,
        records: vec![MetricRecord {
            cursor: 10,
            epoch: 1000,
            name: "x".to_string(),
            value: 1.0,
            unit: "u".to_string(),
            kind: MetricKind::Gauge,
        }],
    };

    let result = batch_after(&batch, 5, 10);
    assert_eq!(result.metric_batch_id, MetricBatchId(42));
    assert_eq!(result.aggregation_window, 30);
    assert!(matches!(result.quality_flag, MetricQualityFlag::Degraded));
    assert_eq!(result.profile, CompatibilityProfile::Automation);
}
