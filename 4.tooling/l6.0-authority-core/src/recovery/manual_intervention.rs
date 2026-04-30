use super::strategies::RecoveryStrategy;

pub(crate) fn unknown_error_strategy() -> RecoveryStrategy {
    RecoveryStrategy::ManualIntervention(
        "Unknown error - manual investigation required".to_string(),
    )
}

pub(crate) fn unknown_error_description() -> &'static str {
    "Unknown error - manual intervention required"
}

pub(crate) fn retry_exhausted_description(description: &str, max_retries: u32) -> String {
    format!("{} (max retries {} exceeded)", description, max_retries)
}
