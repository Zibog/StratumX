// Integration tests entry point
// This file includes all integration test modules

#[path = "integration/common.rs"]
mod common;

#[path = "integration/test_migration_workflow.rs"]
mod test_migration_workflow;

#[path = "integration/test_hygiene_workflow.rs"]
mod test_hygiene_workflow;

#[path = "integration/test_waiver_workflow.rs"]
mod test_waiver_workflow;

#[path = "integration/test_rollback_workflow.rs"]
mod test_rollback_workflow;
