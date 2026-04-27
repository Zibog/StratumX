use engine_storage_layout::*;

// === LayoutClass Tests ===

#[test]
fn test_layout_class_variants() {
    let sparse = LayoutClass::Sparse;
    let dense = LayoutClass::ChunkDense;
    let columnar = LayoutClass::Columnar;

    assert_ne!(sparse, dense);
    assert_ne!(sparse, columnar);
    assert_ne!(dense, columnar);
}

#[test]
fn test_layout_class_equality() {
    assert_eq!(LayoutClass::Sparse, LayoutClass::Sparse);
    assert_eq!(LayoutClass::ChunkDense, LayoutClass::ChunkDense);
}

// === LocalityClass Tests ===

#[test]
fn test_locality_class_variants() {
    let cache = LocalityClass::Cache;
    let spatial = LocalityClass::Spatial;
    let partition = LocalityClass::Partition;
    let traversal = LocalityClass::TraversalLane;

    assert_ne!(cache, spatial);
    assert_ne!(cache, partition);
    assert_ne!(cache, traversal);
    assert_ne!(spatial, partition);
}

#[test]
fn test_locality_class_equality() {
    assert_eq!(LocalityClass::Cache, LocalityClass::Cache);
    assert_eq!(LocalityClass::Partition, LocalityClass::Partition);
}

// === ChunkAccessMode Tests ===

#[test]
fn test_chunk_access_mode_read() {
    let mode = ChunkAccessMode::READ;
    assert!(mode.contains(ChunkAccessMode::READ));
    assert!(!mode.contains(ChunkAccessMode::STAGED_WRITE));
}

#[test]
fn test_chunk_access_mode_staged_write() {
    let mode = ChunkAccessMode::STAGED_WRITE;
    assert!(mode.contains(ChunkAccessMode::STAGED_WRITE));
    assert!(!mode.contains(ChunkAccessMode::READ));
}

#[test]
fn test_chunk_access_mode_both() {
    let mode = ChunkAccessMode::READ | ChunkAccessMode::STAGED_WRITE;
    assert!(mode.contains(ChunkAccessMode::READ));
    assert!(mode.contains(ChunkAccessMode::STAGED_WRITE));
    assert_eq!(mode.bits(), 0b0011);
}

#[test]
fn test_chunk_access_mode_empty() {
    let mode = ChunkAccessMode::empty();
    assert!(mode.is_empty());
}

#[test]
fn test_chunk_access_mode_from_bits() {
    let mode = ChunkAccessMode::from_bits(0b0011);
    assert!(mode.is_some());
    let mode = mode.unwrap();
    assert!(mode.contains(ChunkAccessMode::READ));
    assert!(mode.contains(ChunkAccessMode::STAGED_WRITE));
}

#[test]
fn test_chunk_access_mode_from_bits_invalid() {
    assert!(ChunkAccessMode::from_bits(0b1000).is_none());
}

#[test]
fn test_chunk_access_mode_bits() {
    assert_eq!(ChunkAccessMode::READ.bits(), 0b0001);
    assert_eq!(ChunkAccessMode::STAGED_WRITE.bits(), 0b0010);
}

// === ChunkInvalidationLaw Tests ===

#[test]
fn test_chunk_invalidation_law_variant() {
    let law = ChunkInvalidationLaw::FrozenAtCreation;
    assert!(matches!(law, ChunkInvalidationLaw::FrozenAtCreation));
}

// === Serialization Tests ===

#[test]
fn test_layout_class_serialization_roundtrip() {
    for class in [
        LayoutClass::Sparse,
        LayoutClass::ChunkDense,
        LayoutClass::Columnar,
    ] {
        let json = serde_json::to_string(&class).unwrap();
        let loaded: LayoutClass = serde_json::from_str(&json).unwrap();
        assert_eq!(class, loaded);
    }
}

#[test]
fn test_locality_class_serialization_roundtrip() {
    for class in [
        LocalityClass::Cache,
        LocalityClass::Spatial,
        LocalityClass::Partition,
        LocalityClass::TraversalLane,
    ] {
        let json = serde_json::to_string(&class).unwrap();
        let loaded: LocalityClass = serde_json::from_str(&json).unwrap();
        assert_eq!(class, loaded);
    }
}

#[test]
fn test_chunk_access_mode_serialization_roundtrip() {
    for mode in [
        ChunkAccessMode::READ,
        ChunkAccessMode::STAGED_WRITE,
        ChunkAccessMode::READ | ChunkAccessMode::STAGED_WRITE,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let loaded: ChunkAccessMode = serde_json::from_str(&json).unwrap();
        assert_eq!(mode, loaded);
    }
}

#[test]
fn test_chunk_invalidation_law_serialization_roundtrip() {
    let law = ChunkInvalidationLaw::FrozenAtCreation;
    let json = serde_json::to_string(&law).unwrap();
    let loaded: ChunkInvalidationLaw = serde_json::from_str(&json).unwrap();
    assert_eq!(law, loaded);
}

// === BitFlags Interop Tests ===

#[test]
fn test_chunk_access_mode_intersects() {
    let read = ChunkAccessMode::READ;
    let write = ChunkAccessMode::STAGED_WRITE;
    let both = read | write;

    assert!(both.intersects(read));
    assert!(both.intersects(write));
    assert!(!read.intersects(write));
}

#[test]
fn test_chunk_access_mode_remove() {
    let mut both = ChunkAccessMode::READ | ChunkAccessMode::STAGED_WRITE;
    both.remove(ChunkAccessMode::STAGED_WRITE);
    assert!(both.contains(ChunkAccessMode::READ));
    assert!(!both.contains(ChunkAccessMode::STAGED_WRITE));
}
