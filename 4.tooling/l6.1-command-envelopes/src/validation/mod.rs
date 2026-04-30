// Command Payload Validation
//
// Validates command payloads before execution.
// All promoted commands must pass validation before being routed.

mod errors;
mod rules;
mod validators;
mod verdicts;

pub use validators::validate_command;
pub use verdicts::ValidationResult;
