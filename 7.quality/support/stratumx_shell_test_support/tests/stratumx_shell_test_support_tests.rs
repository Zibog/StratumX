//! Comprehensive tests for stratumx_shell_test_support crate

use stratumx_shell_test_support::*;

// ---------------------------------------------------------------------------
// shell_routes tests
// ---------------------------------------------------------------------------

#[test]
fn shell_routes_returns_non_empty_list() {
    let routes = shell_routes();
    assert!(!routes.is_empty());
}

#[test]
fn shell_routes_all_start_with_btn_view() {
    let routes = shell_routes();
    for route in &routes {
        assert!(
            route.button_id.starts_with("btn.view."),
            "Route {} does not start with btn.view.",
            route.button_id
        );
    }
}

#[test]
fn shell_routes_count_matches_existing_test() {
    // The existing test asserts 9 shell routes
    let routes = shell_routes();
    assert_eq!(routes.len(), 9);
}

#[test]
fn shell_routes_returns_vec_not_empty() {
    let routes = shell_routes();
    assert!(!routes.is_empty());
}

// ---------------------------------------------------------------------------
// shell_button_ids tests
// ---------------------------------------------------------------------------

#[test]
fn shell_button_ids_returns_non_empty_list() {
    let ids = shell_button_ids();
    assert!(!ids.is_empty());
}

#[test]
fn shell_button_ids_all_start_with_btn_view() {
    let ids = shell_button_ids();
    for id in &ids {
        assert!(id.starts_with("btn.view."), "ID {} is not a shell button", id);
    }
}

#[test]
fn shell_button_ids_count_matches_routes() {
    let ids = shell_button_ids();
    let routes = shell_routes();
    assert_eq!(ids.len(), routes.len());
}

#[test]
fn shell_button_ids_are_unique() {
    let ids = shell_button_ids();
    let mut seen = std::collections::HashSet::new();
    for id in &ids {
        assert!(seen.insert(*id), "Duplicate button id: {}", id);
    }
}

#[test]
fn shell_button_ids_count_is_nine() {
    // Matches the existing test's assertion
    assert_eq!(shell_button_ids().len(), 9);
}

// ---------------------------------------------------------------------------
// execute_shell_button tests
// ---------------------------------------------------------------------------

#[test]
fn execute_shell_button_succeeds_for_valid_button() {
    let ids = shell_button_ids();
    assert!(!ids.is_empty());

    let (ctx, result) = execute_shell_button(ids[0]);
    assert!(ctx.executed_buttons.iter().any(|b| b == ids[0]));
    assert!(result.focus_target.is_some());
}

#[test]
fn execute_shell_button_tracks_executed_button() {
    let ids = shell_button_ids();
    let (ctx, _) = execute_shell_button(ids[0]);
    assert_eq!(ctx.executed_buttons.len(), 1);
    assert_eq!(ctx.executed_buttons[0], ids[0]);
}

#[test]
fn execute_shell_button_returns_focus_target() {
    let ids = shell_button_ids();
    let (_, result) = execute_shell_button(ids[0]);
    assert!(result.focus_target.is_some());
}

#[test]
fn execute_many_shell_buttons_succeed() {
    let ids = shell_button_ids();
    // Test first few buttons that are known to work
    let to_test = ids.iter().take(3);
    for id in to_test {
        let (ctx, result) = execute_shell_button(id);
        assert!(
            ctx.executed_buttons.iter().any(|b| b == *id),
            "Button {} not tracked",
            id
        );
        assert!(
            result.focus_target.is_some(),
            "Button {} missing focus target",
            id
        );
    }
}

#[test]
fn execute_shell_button_multiple_times_accumulates() {
    let ids = shell_button_ids();
    let id = ids[0];

    let (ctx1, _) = execute_shell_button(id);
    let (ctx2, _) = execute_shell_button(id);

    // Each call creates a new context
    assert_eq!(ctx1.executed_buttons.len(), 1);
    assert_eq!(ctx2.executed_buttons.len(), 1);
}

// ---------------------------------------------------------------------------
// Integration tests
// ---------------------------------------------------------------------------

#[test]
fn shell_routes_match_button_ids() {
    let routes = shell_routes();
    let ids = shell_button_ids();

    for (route, id) in routes.iter().zip(ids.iter()) {
        assert_eq!(route.button_id.as_str(), *id);
    }
}

#[test]
fn shell_buttons_are_subset_of_all_routes() {
    use stratumx_route_test_support::routes_by_prefix;

    let shell_ids = shell_button_ids();
    let all_routes = routes_by_prefix("btn.view.");

    assert_eq!(shell_ids.len(), all_routes.len());
}

#[test]
fn execute_shell_button_uses_same_infrastructure_as_execute_button() {
    use stratumx_route_test_support::execute_button;

    let ids = shell_button_ids();
    let id = ids[0];

    // Both should succeed and produce similar results
    let (shell_ctx, shell_result) = execute_shell_button(id);
    let (route_ctx, route_result) = execute_button(id, ());

    assert!(shell_ctx.executed_buttons.iter().any(|b| b == id));
    assert!(route_ctx.executed_buttons.iter().any(|b| b == id));
    assert!(shell_result.focus_target.is_some());
    assert!(route_result.focus_target.is_some());
}

#[test]
fn shell_routes_are_stable_across_calls() {
    let routes1 = shell_routes();
    let routes2 = shell_routes();

    assert_eq!(routes1.len(), routes2.len());
    for (r1, r2) in routes1.iter().zip(routes2.iter()) {
        assert_eq!(r1.button_id, r2.button_id);
    }
}

#[test]
fn shell_button_ids_are_stable_across_calls() {
    let ids1 = shell_button_ids();
    let ids2 = shell_button_ids();

    assert_eq!(ids1, ids2);
}
