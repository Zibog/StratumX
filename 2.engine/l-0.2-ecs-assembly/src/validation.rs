use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcsError {
    EntityNotRegistered,
    ComponentNotFound,
    InvalidAssembly,
}

impl fmt::Display for EcsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntityNotRegistered => write!(f, "Entity not registered"),
            Self::ComponentNotFound => write!(f, "Component not found"),
            Self::InvalidAssembly => write!(f, "Invalid ECS assembly"),
        }
    }
}

impl std::error::Error for EcsError {}
