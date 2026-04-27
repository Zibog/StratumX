use stratumx_route_test_support::{execute_button, routes_by_prefix, CanonicalButtonRoute};
pub use stratumx_route_test_support::{CommandResult, ToolSessionContext};

pub fn shell_routes() -> Vec<&'static CanonicalButtonRoute> {
    routes_by_prefix("btn.view.")
}

pub fn shell_button_ids() -> Vec<&'static str> {
    shell_routes()
        .into_iter()
        .map(|route| route.button_id.as_str())
        .collect()
}

pub fn execute_shell_button(button_id: &str) -> (ToolSessionContext, CommandResult) {
    execute_button(button_id, ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_discovers_shell_routes() {
        assert_eq!(shell_button_ids().len(), 9);
    }
}
