// Graphics validation rules

use crate::validation::errors::ValidationResult;

/// Validate time of day (0.0-24.0)
pub fn validate_time_of_day(hours: f32) -> ValidationResult {
    if !(0.0..24.0).contains(&hours) {
        ValidationResult::Invalid {
            errors: vec!["time_of_day_hours must be between 0.0 and 24.0".to_string()],
        }
    } else {
        ValidationResult::Valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_time_of_day() {
        assert!(validate_time_of_day(12.0).is_valid());
        assert!(!validate_time_of_day(-1.0).is_valid());
        assert!(!validate_time_of_day(25.0).is_valid());
    }
}
