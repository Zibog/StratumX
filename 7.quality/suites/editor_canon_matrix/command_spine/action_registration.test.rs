//! Unit tests for action handler implementations
//!
//! These tests verify that the registered action handlers return appropriate
//! results and focus targets.

use super::*;
use crate::{ActionContext, ActionId};
use uuid::Uuid;

fn create_test_context() -> ActionContext {
    ActionContext::from_legacy(
        Some(Uuid::new_v4()),
        Some(Uuid::new_v4()),
        vec![],
        None,
        false,
    )
}

#[test]
fn test_world_open_returns_success_with_viewport_focus() {
    let mut registry = ActionRegistry::new();
    register_core_actions(&mut registry);
    
    let definition = registry.get_handler(&ActionId::new("world.open")).unwrap();
    let context = create_test_context();
    let result = (definition.handler)(context);
    
    assert!(result.success);
    assert!(result.error_code.is_none());
    assert_eq!(result.focus_target, FocusTarget::SpecificPanel("viewport".to_string()));
    assert!(result.artifact_ref.is_none());
    assert!(result.next_legal_recovery_action.is_none());
}

#[test]
fn test_world_close_returns_success_with_no_focus() {
    let mut registry = ActionRegistry::new();
    register_core_actions(&mut registry);
    
    let definition = registry.get_handler(&ActionId::new("world.close")).unwrap();
    let context = create_test_context();
    let result = (definition.handler)(context);
    
    assert!(result.success);
    assert!(result.error_code.is_none());
    assert_eq!(result.focus_target, FocusTarget::NoChange);
}

#[test]
fn test_world_save_returns_success_with_no_focus() {
    let mut registry = ActionRegistry::new();
    register_core_actions(&mut registry);
    
    let definition = registry.get_handler(&ActionId::new("world.save")).unwrap();
    let context = create_test_context();
    let result = (definition.handler)(context);
    
    assert!(result.success);
    assert!(result.error_code.is_none());
    assert_eq!(result.focus_target, FocusTarget::NoChange);
}

#[test]
fn test_project_new_returns_success_with_no_focus() {
    let mut registry = ActionRegistry::new();
    register_core_actions(&mut registry);
    
    let definition = registry.get_handler(&ActionId::new("project.new")).unwrap();
    let context = create_test_context();
    let result = (definition.handler)(context);
    
    assert!(result.success);
    assert!(result.error_code.is_none());
    assert_eq!(result.focus_target, FocusTarget::NoChange);
}

#[test]
fn test_project_open_returns_success_with_no_focus() {
    let mut registry = ActionRegistry::new();
    register_core_actions(&mut registry);
    
    let definition = registry.get_handler(&ActionId::new("project.open")).unwrap();
    let context = create_test_context();
    let result = (definition.handler)(context);
    
    assert!(result.success);
    assert!(result.error_code.is_none());
    assert_eq!(result.focus_target, FocusTarget::NoChange);
}

#[test]
fn test_all_registered_actions_have_handlers() {
    let mut registry = ActionRegistry::new();
    register_core_actions(&mut registry);
    
    // Verify each action can be retrieved and has a handler
    let action_ids = [
        "world.open", "world.close", "world.save",
        "terrain.sculpt", "terrain.paint", "terrain.configure_layers",
        "environment.configure_sky", "environment.configure_weather", "environment.set_time",
        "runtime.play", "runtime.pause", "runtime.step",
        "project.new", "project.open", "project.close",
        "panel.open", "panel.close",
        "material.author_profile", "material.bind_surface", "material.inspect",
        "audio.author_source", "audio.configure_zone", "audio.preview",
    ];
    
    for action_id in action_ids {
        let definition = registry.get_handler(&ActionId::new(action_id));
        assert!(definition.is_some(), "Action {} should be registered", action_id);
        
        // Verify handler can be called
        let def = definition.unwrap();
        let context = create_test_context();
        let result = (def.handler)(context);
        
        // All placeholder handlers should return success
        assert!(result.success, "Handler for {} should return success", action_id);
    }
}

#[test]
fn test_world_actions_have_correct_focus_targets() {
    let mut registry = ActionRegistry::new();
    register_core_actions(&mut registry);
    
    let context = create_test_context();
    
    // world.open should focus viewport
    let world_open = registry.get_handler(&ActionId::new("world.open")).unwrap();
    let result = (world_open.handler)(context.clone());
    assert_eq!(result.focus_target, FocusTarget::SpecificPanel("viewport".to_string()));
    
    // world.close should not change focus
    let world_close = registry.get_handler(&ActionId::new("world.close")).unwrap();
    let result = (world_close.handler)(context.clone());
    assert_eq!(result.focus_target, FocusTarget::NoChange);
    
    // world.save should not change focus
    let world_save = registry.get_handler(&ActionId::new("world.save")).unwrap();
    let result = (world_save.handler)(context);
    assert_eq!(result.focus_target, FocusTarget::NoChange);
}

#[test]
fn test_project_actions_have_no_focus_change() {
    let mut registry = ActionRegistry::new();
    register_core_actions(&mut registry);
    
    let context = create_test_context();
    
    // project.new should not change focus
    let project_new = registry.get_handler(&ActionId::new("project.new")).unwrap();
    let result = (project_new.handler)(context.clone());
    assert_eq!(result.focus_target, FocusTarget::NoChange);
    
    // project.open should not change focus
    let project_open = registry.get_handler(&ActionId::new("project.open")).unwrap();
    let result = (project_open.handler)(context);
    assert_eq!(result.focus_target, FocusTarget::NoChange);
}
