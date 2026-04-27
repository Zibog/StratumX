// Command Payload Validation
//
// Validates command payloads before execution.
// All promoted commands must pass validation before being routed.

mod errors;
mod rules;
mod validators;

pub use errors::ValidationResult;
pub use validators::validate_command;
