use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    EntityNotRegistered,
    ComponentNotRegistered,
    DuplicateEntity,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntityNotRegistered => write!(f, "Entity not registered"),
            Self::ComponentNotRegistered => write!(f, "Component not registered"),
            Self::DuplicateEntity => write!(f, "Entity already registered"),
        }
    }
}

impl std::error::Error for RegistryError {}
