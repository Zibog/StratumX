// Validation error types

/// Validation result for command payloads
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    Valid,
    Invalid { errors: Vec<String> },
}

impl ValidationResult {
    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// Get validation errors
    pub fn errors(&self) -> Vec<String> {
        match self {
            ValidationResult::Valid => vec![],
            ValidationResult::Invalid { errors } => errors.clone(),
        }
    }

    /// Add an error to the validation result
    pub fn add_error(mut self, error: impl Into<String>) -> Self {
        match &mut self {
            ValidationResult::Valid => ValidationResult::Invalid {
                errors: vec![error.into()],
            },
            ValidationResult::Invalid { errors } => {
                errors.push(error.into());
                self
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_add_error() {
        let result = ValidationResult::Valid
            .add_error("error1")
            .add_error("error2");
        assert!(!result.is_valid());
        assert_eq!(result.errors().len(), 2);
    }
}
