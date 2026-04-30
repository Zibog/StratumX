// Common validation rules

use crate::validation::errors::ValidationResult;

/// Validate a path field
pub fn validate_path(path: &str, field_name: &str) -> ValidationResult {
    if path.is_empty() {
        ValidationResult::Invalid {
            errors: vec![format!("{} cannot be empty", field_name)],
        }
    } else {
        ValidationResult::Valid
    }
}

/// Validate a non-empty string field
pub fn validate_non_empty(value: &str, field_name: &str) -> ValidationResult {
    if value.is_empty() {
        ValidationResult::Invalid {
            errors: vec![format!("{} cannot be empty", field_name)],
        }
    } else {
        ValidationResult::Valid
    }
}

/// Validate a value is within a range
pub fn validate_range(value: f32, min: f32, max: f32, field_name: &str) -> ValidationResult {
    if value < min || value > max {
        ValidationResult::Invalid {
            errors: vec![format!(
                "{} must be between {} and {}",
                field_name, min, max
            )],
        }
    } else {
        ValidationResult::Valid
    }
}

/// Validate a value is positive
pub fn validate_positive(value: f32, field_name: &str) -> ValidationResult {
    if value < 0.0 {
        ValidationResult::Invalid {
            errors: vec![format!("{} must be positive", field_name)],
        }
    } else {
        ValidationResult::Valid
    }
}

/// Validate project name
pub fn validate_project_name(name: &str) -> ValidationResult {
    if name.is_empty() {
        return ValidationResult::Invalid {
            errors: vec!["project_name cannot be empty".to_string()],
        };
    }
    if name.len() > 64 {
        return ValidationResult::Invalid {
            errors: vec!["project_name cannot exceed 64 characters".to_string()],
        };
    }
    ValidationResult::Valid
}

/// Validate project creation fields
pub fn validate_project_create(
    project_name: &str,
    project_root: &str,
    world_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if project_name.is_empty() {
        result = result.add_error("project_name cannot be empty");
    }
    if project_root.is_empty() {
        result = result.add_error("project_root cannot be empty");
    }
    if world_name.is_empty() {
        result = result.add_error("world_name cannot be empty");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name() {
        assert!(validate_project_name("MyProject").is_valid());
        assert!(!validate_project_name("").is_valid());
        assert!(!validate_project_name(&"a".repeat(65)).is_valid());
    }
}
