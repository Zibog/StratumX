use engine_core::{ComponentTypeId, EngineCoreError, Invariant};
use engine_storage_layout::{
    ChunkAccessMode, ChunkDescriptor, ChunkInvalidationLaw, ColumnDescriptor, LayoutClass,
    LocalityClass, StorageLayoutDescriptor,
};

fn chunk_dense_descriptor() -> StorageLayoutDescriptor {
    StorageLayoutDescriptor {
        layout_class: LayoutClass::ChunkDense,
        chunk: Some(ChunkDescriptor {
            signature: vec![ComponentTypeId(1), ComponentTypeId(2)].into(),
            access_mode: ChunkAccessMode::READ | ChunkAccessMode::STAGED_WRITE,
            invalidation_law: ChunkInvalidationLaw::FrozenAtCreation,
        }),
        columns: Vec::<ColumnDescriptor>::new().into(),
        locality_class: LocalityClass::Spatial,
    }
}

#[test]
fn chunk_dense_layout_requires_chunk_signature_and_access_mode() {
    let descriptor = chunk_dense_descriptor();
    descriptor.check_invariants().unwrap();

    let mut missing_signature = descriptor.clone();
    missing_signature.chunk.as_mut().unwrap().signature.clear();
    assert_eq!(
        missing_signature.check_invariants(),
        Err(EngineCoreError::InvalidDescriptor(
            "chunk signature must be frozen and non-empty",
        ))
    );

    let mut missing_access_mode = descriptor;
    missing_access_mode.chunk.as_mut().unwrap().access_mode = ChunkAccessMode::empty();
    assert_eq!(
        missing_access_mode.check_invariants(),
        Err(EngineCoreError::InvalidDescriptor(
            "chunk access mode must be frozen",
        ))
    );
}

#[test]
fn columnar_layout_requires_at_least_one_family() {
    let descriptor = StorageLayoutDescriptor {
        layout_class: LayoutClass::Columnar,
        chunk: None,
        columns: Vec::<ColumnDescriptor>::new().into(),
        locality_class: LocalityClass::Partition,
    };

    assert_eq!(
        descriptor.check_invariants(),
        Err(EngineCoreError::InvalidDescriptor(
            "columnar layout requires at least one column family",
        ))
    );
}

#[test]
fn layout_descriptor_roundtrips_through_json() {
    let descriptor = chunk_dense_descriptor();
    let json = serde_json::to_string(&descriptor).unwrap();
    let restored: StorageLayoutDescriptor = serde_json::from_str(&json).unwrap();

    assert_eq!(restored, descriptor);
}
