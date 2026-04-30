#[test]
fn identity_component_issue_live_10() {
    let mut a = IdentityAllocator::new(IdentityDomain::Component);
    let id = a.issue_component().unwrap();
    assert!(a.is_live_component(id));
}
#[test]
fn identity_component_issue_live_11() {
    let mut a = IdentityAllocator::new(IdentityDomain::Component);
    let id = a.issue_component().unwrap();
    assert!(a.is_live_component(id));
}
#[test]
fn identity_component_issue_live_12() {
    let mut a = IdentityAllocator::new(IdentityDomain::Component);
    let id = a.issue_component().unwrap();
    assert!(a.is_live_component(id));
}
#[test]
fn identity_component_issue_live_13() {
    let mut a = IdentityAllocator::new(IdentityDomain::Component);
    let id = a.issue_component().unwrap();
    assert!(a.is_live_component(id));
}
#[test]
fn identity_component_issue_live_14() {
    let mut a = IdentityAllocator::new(IdentityDomain::Component);
    let id = a.issue_component().unwrap();
    assert!(a.is_live_component(id));
}
#[test]
fn identity_reuse_after_epoch_0() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_1() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_2() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_3() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_4() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_5() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_6() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_7() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_8() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_9() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_10() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_11() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_12() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_13() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
#[test]
fn identity_reuse_after_epoch_14() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
}
