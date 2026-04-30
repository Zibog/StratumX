#[test]
fn ecs_query_write_requires_scratch_7() {
    let q = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(1)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::WRITE,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(q.validate_legality().is_err());
}
#[test]
fn ecs_query_write_requires_scratch_8() {
    let q = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(1)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::WRITE,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(q.validate_legality().is_err());
}
#[test]
fn ecs_query_write_requires_scratch_9() {
    let q = QueryDescriptor {
        component_set: smallvec![ComponentTypeId(1)],
        locality: QueryLocality::Cache,
        partitionability: Partitionability::None,
        cache_key: 1,
        access: QueryAccessDescriptor {
            mode: QueryAccessMode::WRITE,
            publication_rights: false,
            scratch: None,
        },
        filters: smallvec![],
        joins: smallvec![],
    };
    assert!(q.validate_legality().is_err());
}
