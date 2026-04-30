#[test]
fn test_entity_id_zero_values() {
    let id = EntityId {
        slot: 0,
        generation: Generation(0),
    };
    assert_eq!(id.slot, 0);
    assert_eq!(id.generation, Generation(0));
}

#[test]
fn test_entity_id_max_values() {
    let id = EntityId {
        slot: u32::MAX,
        generation: Generation(u32::MAX),
    };
    assert_eq!(id.slot, u32::MAX);
    assert_eq!(id.generation, Generation(u32::MAX));
}

#[test]
fn test_identity_entities_are_hashable() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let id1 = EntityId {
        slot: 1,
        generation: Generation(0),
    };
    let id2 = EntityId {
        slot: 1,
        generation: Generation(0),
    };
    let id3 = EntityId {
        slot: 2,
        generation: Generation(0),
    };

    set.insert(id1);
    set.insert(id2); // Duplicate
    set.insert(id3);

    assert_eq!(set.len(), 2);
    assert!(set.contains(&id1));
    assert!(set.contains(&id3));
}
