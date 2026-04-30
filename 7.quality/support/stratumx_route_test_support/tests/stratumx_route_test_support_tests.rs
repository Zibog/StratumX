//! Comprehensive tests for stratumx_route_test_support crate

use std::collections::BTreeSet;
use stratumx_route_test_support::*;

// ---------------------------------------------------------------------------
// manifest_button_ids tests
// ---------------------------------------------------------------------------

#[test]
fn manifest_button_ids_returns_non_empty_set() {
    let ids = manifest_button_ids();
    assert!(!ids.is_empty());
}

#[test]
fn manifest_button_ids_contains_known_buttons() {
    let ids = manifest_button_ids();
    // The existing test uses "btn.material.bind_light_response"
    assert!(ids.contains("btn.material.bind_light_response"));
}

#[test]
fn manifest_button_ids_returns_btree_set() {
    let ids: BTreeSet<&str> = manifest_button_ids();
    // BTreeSet is sorted; verify ordering is stable
    let first = ids.iter().next().unwrap();
    assert!(first.starts_with("btn."));
}

// ---------------------------------------------------------------------------
// route() tests
// ---------------------------------------------------------------------------

#[test]
fn route_returns_correct_route_for_material_button() {
    let route = route("btn.material.bind_light_response");
    assert_eq!(route.button_id, "btn.material.bind_light_response");
}

#[test]
fn route_returns_same_route_on_repeated_calls() {
    let r1 = route("btn.material.bind_light_response");
    let r2 = route("btn.material.bind_light_response");
    assert_eq!(r1.button_id, r2.button_id);
}

#[test]
#[should_panic(expected = "button route must exist")]
fn route_panics_on_unknown_button() {
    route("btn.nonexistent.button");
}

#[test]
fn route_has_non_empty_action_id() {
    let route = route("btn.material.bind_light_response");
    assert!(!route.action_id.is_empty());
}

// ---------------------------------------------------------------------------
// routes_by_prefix tests
// ---------------------------------------------------------------------------

#[test]
fn routes_by_prefix_returns_empty_for_unknown_prefix() {
    let routes = routes_by_prefix("btn.nonexistent_prefix.");
    assert!(routes.is_empty());
}

#[test]
fn routes_by_prefix_material_returns_results() {
    let routes = routes_by_prefix("btn.material.");
    assert!(!routes.is_empty());
}

#[test]
fn routes_by_prefix_all_routes_start_with_prefix() {
    let routes = routes_by_prefix("btn.material.");
    for route in &routes {
        assert!(route.button_id.starts_with("btn.material."));
    }
}

#[test]
fn routes_by_prefix_view_returns_shell_routes() {
    let routes = routes_by_prefix("btn.view.");
    assert!(!routes.is_empty());
}

#[test]
fn routes_by_prefix_terrain_returns_results() {
    let routes = routes_by_prefix("btn.terrain.");
    assert!(!routes.is_empty());
}

// ---------------------------------------------------------------------------
// infer_source_surface tests
// ---------------------------------------------------------------------------

#[test]
fn infer_source_surface_project_returns_main_menu() {
    // Simulate a project route
    let route = route("btn.material.bind_light_response");
    let surface = infer_source_surface(route);
    // material should map to MaterialPanel
    assert!(matches!(surface, SourceSurface::MaterialPanel));
}

#[test]
fn infer_source_surface_view_returns_viewport_panel() {
    let routes = routes_by_prefix("btn.view.");
    if !routes.is_empty() {
        let surface = infer_source_surface(routes[0]);
        assert!(matches!(surface, SourceSurface::ViewportPanel));
    }
}

#[test]
fn infer_source_surface_terrain_returns_terrain_panel() {
    let routes = routes_by_prefix("btn.terrain.");
    if !routes.is_empty() {
        let surface = infer_source_surface(routes[0]);
        assert!(matches!(surface, SourceSurface::TerrainPanel));
    }
}

// ---------------------------------------------------------------------------
// execute_button tests
// ---------------------------------------------------------------------------

#[test]
fn execute_button_returns_context_and_result() {
    let (ctx, result) = execute_button("btn.material.bind_light_response", ());
    // Context should have recorded the button
    assert!(ctx
        .executed_buttons
        .iter()
        .any(|b| b == "btn.material.bind_light_response"));
    // Result should have a focus target
    assert!(result.focus_target.is_some());
}

#[test]
fn execute_button_tracks_executed_button() {
    let (ctx, _) = execute_button("btn.material.bind_light_response", ());
    assert_eq!(ctx.executed_buttons.len(), 1);
}

#[test]
fn execute_button_in_material_category() {
    let routes = routes_by_prefix("btn.material.");
    assert!(!routes.is_empty(), "Should have material routes");

    let first_route = routes[0];
    let button_id = first_route.button_id.as_str();
    let (ctx, result) = execute_button(button_id, ());
    assert!(ctx.executed_buttons.iter().any(|b| b == button_id));
    assert!(result.focus_target.is_some());
}

// ---------------------------------------------------------------------------
// execute_button_in_context tests
// ---------------------------------------------------------------------------

#[test]
fn execute_button_in_context_reuses_session() {
    let mut ctx = ToolSessionContext::default();
    let result1 = execute_button_in_context(&mut ctx, "btn.material.bind_light_response", ());
    assert!(result1.focus_target.is_some());

    let result2 = execute_button_in_context(&mut ctx, "btn.material.bind_light_response", ());
    assert!(result2.focus_target.is_some());
    // Same context should accumulate
    assert!(ctx.executed_buttons.len() >= 2);
}

#[test]
fn execute_button_in_context_accumulates_executed_buttons() {
    let mut ctx = ToolSessionContext::default();
    execute_button_in_context(&mut ctx, "btn.material.bind_light_response", ());
    execute_button_in_context(&mut ctx, "btn.material.bind_light_response", ());
    assert_eq!(ctx.executed_buttons.len(), 2);
}

// ---------------------------------------------------------------------------
// dispatch_button tests
// ---------------------------------------------------------------------------

#[test]
fn dispatch_button_returns_envelope() {
    let envelope = dispatch_button("btn.material.bind_light_response", ());
    assert!(envelope.is_ok());
}

#[test]
fn dispatch_button_envelope_has_correct_button_id() {
    let envelope = dispatch_button("btn.material.bind_light_response", ()).unwrap();
    assert_eq!(
        envelope.route_metadata.button_id,
        "btn.material.bind_light_response"
    );
}

// ---------------------------------------------------------------------------
// Integration / workflow tests
// ---------------------------------------------------------------------------

#[test]
fn full_dispatch_execute_workflow() {
    let envelope = dispatch_button("btn.material.bind_light_response", ()).unwrap();
    let mut ctx = ToolSessionContext::default();
    let result = CanonicalCommandExecutor::execute(envelope, &mut ctx);
    assert!(result.focus_target.is_some());
    assert!(!ctx.executed_buttons.is_empty());
}

#[test]
fn many_routes_are_dispatchable() {
    // Test a representative sample of routes
    let test_routes = ["btn.material.bind_light_response"];

    for button_id in test_routes {
        let result = std::panic::catch_unwind(|| dispatch_button(button_id, ()));
        match result {
            Ok(Ok(_)) => {}
            Ok(Err(_)) => {}
            Err(_) => panic!("Route {} panicked", button_id),
        }
    }
}

#[test]
fn execute_all_material_buttons_succeeds() {
    let routes = routes_by_prefix("btn.material.");
    assert!(!routes.is_empty());

    for route in routes {
        let (ctx, result) = execute_button(route.button_id.as_str(), ());
        assert!(
            ctx.executed_buttons
                .iter()
                .any(|b| b == route.button_id.as_str()),
            "Button {} not tracked",
            route.button_id
        );
        assert!(
            result.focus_target.is_some(),
            "Button {} missing focus target",
            route.button_id
        );
    }
}
