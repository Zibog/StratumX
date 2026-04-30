#[test]
fn test_query_descriptor_validation_requires_cache_key() {
    let descriptor = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(100)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 0,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(descriptor.validate_legality().is_err());
}

#[test]
fn test_query_descriptor_validation_write_requires_scratch() {
    let descriptor = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(100)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: AccessDescriptor {
            mode: QueryAccessMode::WRITE,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(descriptor.validate_legality().is_err());
}

#[test]
fn test_query_descriptor_valid_read() {
    let descriptor = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(100)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(descriptor.validate_legality().is_ok());
}

#[test]
fn test_query_descriptor_valid_write_with_scratch() {
    let descriptor = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(100)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: AccessDescriptor {
            mode: QueryAccessMode::WRITE,
            publication_rights: false,
            scratch: Some(ScratchClass::Borrowed),
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(descriptor.validate_legality().is_ok());
}

// === ComponentTypeId Tests ===

#[test]
fn test_component_type_id_equality() {
    let a = ComponentTypeId(42);
    let b = ComponentTypeId(42);
    let c = ComponentTypeId(43);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_component_type_id_ordering() {
    let a = ComponentTypeId(10);
    let b = ComponentTypeId(20);
    assert!(a < b);
}

// === BitFlags Tests ===

#[test]
fn test_query_access_mode_bits() {
    let read = QueryAccessMode::READ;
    let write = QueryAccessMode::WRITE;
    let both = read | write;
    assert_eq!(read.bits(), 0b0001);
    assert_eq!(write.bits(), 0b0010);
    assert_eq!(both.bits(), 0b0011);
}

#[test]
fn test_query_access_mode_from_bits() {
    let mode = QueryAccessMode::from_bits(0b0011);
    assert!(mode.is_some());
    let mode = mode.unwrap();
    assert!(mode.contains(QueryAccessMode::READ));
    assert!(mode.contains(QueryAccessMode::WRITE));
}

#[test]
fn test_query_access_mode_from_bits_invalid() {
    let mode = QueryAccessMode::from_bits(0b1000);
    assert!(mode.is_none());
}

// === Integration: Query Descriptor Construction ===

#[test]
fn test_build_complex_query_descriptor() {
    let descriptor = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(100), ComponentTypeId(200)],
        locality: QueryLocality::Spatial,
        partitionability: Partitionability::Region,
        cache_key: 12345,
        access: AccessDescriptor {
            mode: QueryAccessMode::READ | QueryAccessMode::WRITE,
            publication_rights: true,
            scratch: Some(ScratchClass::Owned),
        },
        filters: smallvec![
            FilterConstraint {
                op: FilterOp::With,
                component: ComponentTypeId(100),
            },
            FilterConstraint {
                op: FilterOp::Without,
                component: ComponentTypeId(999),
            },
        ],
        joins: smallvec![JoinRule {
            required: smallvec![ComponentTypeId(200)],
        }],
    };

    assert!(descriptor.validate_legality().is_ok());
    assert_eq!(descriptor.component_set.len(), 2);
    assert_eq!(descriptor.filters.len(), 2);
    assert_eq!(descriptor.joins.len(), 1);
}
