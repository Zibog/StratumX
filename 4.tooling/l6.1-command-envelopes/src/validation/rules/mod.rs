// Validation rules organized by domain

pub mod assets;
pub mod audio;
pub mod common;
pub mod diagnostics;
pub mod editor;
pub mod graphics;
pub mod materials;

// Re-export all validation functions for backward compatibility
pub use assets::*;
pub use audio::*;
pub use common::*;
pub use graphics::*;
pub use materials::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name() {
        assert!(common::validate_project_name("MyProject").is_valid());
        assert!(!common::validate_project_name("").is_valid());
        assert!(!common::validate_project_name(&"a".repeat(65)).is_valid());
    }

    #[test]
    fn test_validate_terrain_sculpt() {
        assert!(assets::validate_terrain_sculpt(&[0.0, 0.0], 1.0, 0.5).is_valid());
        assert!(!assets::validate_terrain_sculpt(&[0.0, 0.0], -1.0, 0.5).is_valid());
        assert!(!assets::validate_terrain_sculpt(&[0.0, 0.0], 1.0, 1.5).is_valid());
    }

    #[test]
    fn test_validate_time_of_day() {
        assert!(graphics::validate_time_of_day(12.0).is_valid());
        assert!(!graphics::validate_time_of_day(-1.0).is_valid());
        assert!(!graphics::validate_time_of_day(25.0).is_valid());
    }
}
