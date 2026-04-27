//! Validation logic

use crate::entity_state::Entity;

pub fn validate_entity_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    if name.len() > 256 {
        return Err(ValidationError::NameTooLong);
    }
    Ok(())
}

pub fn validate_entity(entity: &Entity) -> Result<(), ValidationError> {
    validate_entity_name(&entity.name)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    EmptyName,
    NameTooLong,
}
