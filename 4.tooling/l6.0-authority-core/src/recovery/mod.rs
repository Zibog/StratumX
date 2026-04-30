// Authority Core Recovery
//
// Recovery policies for authority container operations.
// Split by role so retry, rollback, focus repair, and evidence tracking stay isolated.

mod errors;
mod evidence_recovery;
mod focus_recovery;
mod policy_ids;
mod retry_policy;
mod rollback_policy;

pub use errors::{ErrorClass, ErrorState};
pub use evidence_recovery::RecoveryManager;
pub use policy_ids::{RecoveryStrategy, RecoveryTarget};
pub use retry_policy::RecoveryContext;
