// Authority Core Recovery
//
// Recovery strategies for authority container operations.
// Defines explicit recovery targets and strategies for common failure modes.

mod policies;
mod strategies;

pub use policies::{ErrorState, RecoveryContext, RecoveryManager, RecoveryTarget};
pub use strategies::{ErrorClass, RecoveryStrategy};
