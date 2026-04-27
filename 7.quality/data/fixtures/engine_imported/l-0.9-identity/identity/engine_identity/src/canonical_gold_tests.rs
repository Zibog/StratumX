#![allow(unused_imports)]
use super::*;

#[test]
fn entity_issue_is_live() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let id = a.issue_entity().unwrap();
    assert!(a.is_live_entity(id));
}
#[test]
fn component_issue_is_live() {
    let mut a = IdentityAllocator::new(IdentityDomain::Component);
    let id = a.issue_component().unwrap();
    assert!(a.is_live_component(id));
}
#[test]
fn wrong_domain_returns_none() {
    let mut a = IdentityAllocator::new(IdentityDomain::Component);
    assert!(a.issue_entity().is_none());
}
#[test]
fn retire_kills_entity() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let id = a.issue_entity().unwrap();
    assert!(a.retire_entity(id));
    assert!(!a.is_live_entity(id));
}
#[test]
fn advance_epoch_enables_slot_reuse() {
    let mut a = IdentityAllocator::new(IdentityDomain::Entity);
    let first = a.issue_entity().unwrap();
    a.retire_entity(first);
    a.advance_epoch();
    let second = a.issue_entity().unwrap();
    assert_eq!(second.slot, first.slot);
}
