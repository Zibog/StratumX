use engine_core::{ComponentTypeId, Invariant};
use engine_storage_layout::{
    ChunkAccessMode, ChunkDescriptor, ChunkInvalidationLaw, ColumnDescriptor, LayoutClass,
    LocalityClass, StorageLayoutDescriptor,
};
use smallvec::smallvec;

#[test]
fn sparse_layout_without_chunk_is_legal() {
    let descriptor = StorageLayoutDescriptor {
        layout_class: LayoutClass::Sparse,
        chunk: None,
        columns: smallvec![],
        locality_class: LocalityClass::Cache,
    };
    assert!(descriptor.check_invariants().is_ok());
}

#[test]
fn chunk_dense_requires_chunk_descriptor() {
    let descriptor = StorageLayoutDescriptor {
        layout_class: LayoutClass::ChunkDense,
        chunk: None,
        columns: smallvec![],
        locality_class: LocalityClass::Partition,
    };
    assert!(descriptor.check_invariants().is_err());
}

#[test]
fn columnar_layout_with_family_is_legal() {
    let descriptor = StorageLayoutDescriptor {
        layout_class: LayoutClass::Columnar,
        chunk: None,
        columns: smallvec![ColumnDescriptor {
            family: smallvec![ComponentTypeId(1), ComponentTypeId(2)]
        }],
        locality_class: LocalityClass::Cache,
    };
    assert!(descriptor.check_invariants().is_ok());

    let chunk = ChunkDescriptor {
        signature: smallvec![ComponentTypeId(1)],
        access_mode: ChunkAccessMode::READ,
        invalidation_law: ChunkInvalidationLaw::FrozenAtCreation,
    };
    assert_eq!(chunk.signature.len(), 1);
}
