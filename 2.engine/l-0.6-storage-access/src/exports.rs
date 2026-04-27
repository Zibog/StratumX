// Public API for storage access module

pub use crate::queries::{
    access_summary, has_read_mode, has_staged_mode, has_write_mode, is_write_capable,
};
pub use crate::runtime::{ReadView, WriteWindow};
pub use crate::types::{AccessDescriptor, AccessMode, ScratchClass, TraversalPlanId};
pub use crate::validation::{
    validate_read_mode, validate_traversal_entry, validate_write_capable, validate_write_staging,
};

// Factories for views/windows
pub use crate::factories::{make_read_view, make_write_window};
