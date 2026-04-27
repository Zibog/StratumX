#![allow(unused_imports)]
use super::*;
use engine_core::contracts::Invariant;
use smallvec::smallvec;

#[test]
fn chunkdense_requires_chunk() {
    let d = StorageLayoutDescriptor {
        layout_class: LayoutClass::ChunkDense,
        chunk: None,
        columns: smallvec![],
        locality_class: LocalityClass::TraversalLane,
    };
    assert!(d.check_invariants().is_err());
}
#[test]
fn chunkdense_rejects_empty_signature() {
    let d = StorageLayoutDescriptor {
        layout_class: LayoutClass::ChunkDense,
        chunk: Some(ChunkDescriptor {
            signature: smallvec![],
            access_mode: ChunkAccessMode::READ,
            invalidation_law: ChunkInvalidationLaw::FrozenAtCreation,
        }),
        columns: smallvec![],
        locality_class: LocalityClass::TraversalLane,
    };
    assert!(d.check_invariants().is_err());
}
#[test]
fn chunkdense_accepts_non_empty_signature() {
    let d = StorageLayoutDescriptor {
        layout_class: LayoutClass::ChunkDense,
        chunk: Some(ChunkDescriptor {
            signature: smallvec![ComponentTypeId(1)],
            access_mode: ChunkAccessMode::READ,
            invalidation_law: ChunkInvalidationLaw::FrozenAtCreation,
        }),
        columns: smallvec![],
        locality_class: LocalityClass::TraversalLane,
    };
    assert!(d.check_invariants().is_ok());
}
#[test]
fn columnar_requires_column_family() {
    let d = StorageLayoutDescriptor {
        layout_class: LayoutClass::Columnar,
        chunk: None,
        columns: smallvec![],
        locality_class: LocalityClass::Cache,
    };
    assert!(d.check_invariants().is_err());
}
#[test]
fn sparse_layout_is_legal_without_chunk() {
    let d = StorageLayoutDescriptor {
        layout_class: LayoutClass::Sparse,
        chunk: None,
        columns: smallvec![],
        locality_class: LocalityClass::Spatial,
    };
    assert!(d.check_invariants().is_ok());
}
