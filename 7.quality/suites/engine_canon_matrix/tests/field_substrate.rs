mod common;
use common::*;

#[test]
fn field_sample_roundtrip_and_identity_hold() {
    let mut substrate = ScalarFieldSubstrate::new(FieldId(1)).unwrap();
    substrate.write_sample((0, 0, 0), [2, 3, 4], 0.75).unwrap();

    assert_eq!(substrate.field_id(), FieldId(1));
    assert_eq!(substrate.sample((0, 0, 0), [2, 3, 4]), Some(0.75));
}

#[test]
fn field_delta_reports_dirty_regions() {
    let mut substrate = ScalarFieldSubstrate::new(FieldId(2)).unwrap();
    let summary = substrate
        .apply_delta(FieldUpdateDelta {
            samples: vec![
                FieldSampleRef {
                    region_key: (1, 0, 0),
                    cell: [0, 0, 0],
                    value: 0.1,
                },
                FieldSampleRef {
                    region_key: (1, 0, 0),
                    cell: [1, 0, 0],
                    value: 0.2,
                },
                FieldSampleRef {
                    region_key: (2, 0, 0),
                    cell: [0, 0, 0],
                    value: 0.3,
                },
            ],
        })
        .unwrap();

    assert_eq!(summary.sample_count, 3);
    assert_eq!(summary.dirty_regions, vec![(1, 0, 0), (2, 0, 0)]);
}

#[test]
fn field_merge_conflict_is_deterministic_and_invalid_values_fail() {
    let mut substrate = ScalarFieldSubstrate::new(FieldId(3)).unwrap();
    substrate
        .apply_delta(FieldUpdateDelta {
            samples: vec![
                FieldSampleRef {
                    region_key: (0, 0, 0),
                    cell: [0, 0, 0],
                    value: 0.2,
                },
                FieldSampleRef {
                    region_key: (0, 0, 0),
                    cell: [0, 0, 0],
                    value: 0.8,
                },
            ],
        })
        .unwrap();

    assert_eq!(substrate.sample((0, 0, 0), [0, 0, 0]), Some(0.8));
    assert!(substrate
        .apply_delta(FieldUpdateDelta {
            samples: vec![FieldSampleRef {
                region_key: (0, 0, 0),
                cell: [512, 0, 0],
                value: f32::NAN,
            }],
        })
        .is_err());
}
