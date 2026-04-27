use engine_core::{ComponentTypeId, Invariant};
use engine_storage_layout::{
    ChunkAccessMode, ChunkDescriptor, ChunkInvalidationLaw, LayoutClass, LocalityClass,
    StorageLayoutDescriptor,
};
use smallvec::smallvec;

#[test]
fn chunk_layout_requires_frozen_signature_and_mode() {
    let invalid = StorageLayoutDescriptor {
        layout_class: LayoutClass::ChunkDense,
        chunk: Some(ChunkDescriptor {
            signature: smallvec![],
            access_mode: ChunkAccessMode::READ,
            invalidation_law: ChunkInvalidationLaw::FrozenAtCreation,
        }),
        columns: smallvec![],
        locality_class: LocalityClass::TraversalLane,
    };
    assert!(invalid.check_invariants().is_err());
}

#[test]
fn column_layout_requires_column_families() {
    let invalid = StorageLayoutDescriptor {
        layout_class: LayoutClass::Columnar,
        chunk: None,
        columns: smallvec![],
        locality_class: LocalityClass::Cache,
    };
    assert!(invalid.check_invariants().is_err());
}

#[test]
fn valid_chunk_layout_is_deterministic() {
    let descriptor = StorageLayoutDescriptor {
        layout_class: LayoutClass::ChunkDense,
        chunk: Some(ChunkDescriptor {
            signature: smallvec![ComponentTypeId(1), ComponentTypeId(2)],
            access_mode: ChunkAccessMode::READ | ChunkAccessMode::STAGED_WRITE,
            invalidation_law: ChunkInvalidationLaw::FrozenAtCreation,
        }),
        columns: smallvec![],
        locality_class: LocalityClass::TraversalLane,
    };

    assert_eq!(descriptor.check_invariants(), descriptor.check_invariants());
}
