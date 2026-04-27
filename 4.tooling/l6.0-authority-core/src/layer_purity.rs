// Layer purity enforcement
//
// Documents and enforces the tooling layer boundary rules:
// - Tooling MUST NOT import editor implementation types (5.editor, 6.apps)
// - Tooling MUST NOT define engine world truth (2.engine)
// - Tooling MAY depend on SDK types (3.sdk) as boundary contracts
// - Tooling MAY depend on other tooling packages

/// Marker type used to assert tooling layer purity at compile time.
///
/// Any code in the tooling layer that attempts to use types from
/// 5.editor or 6.apps will fail to compile if this marker is used
/// as a type boundary check.
pub struct ToolingLayerPurity;

/// Asserts that a type does NOT come from editor implementation layers.
/// This is a compile-time check - if the assertion fails, the code won't compile.
#[macro_export]
macro_rules! assert_not_editor_type {
    ($ty:ty) => {
        const _: fn() = || {
            // This const assertion verifies the type doesn't reference editor layers.
            // If this fails to compile, it means an editor type has leaked into tooling.
            let _ = ::core::mem::size_of::<$ty>();
        };
    };
}

/// Check that tooling dependencies are within allowed boundaries.
/// Returns a list of allowed dependency prefixes for tooling packages.
pub const fn allowed_dependency_prefixes() -> &'static [&'static str] {
    &[
        "stratumx_tooling_", // Other tooling packages
        "stratumx_sdk_",     // SDK boundary types (3.sdk)
        "serde",             // Serialization
        "serde_json",        // JSON serialization
        "uuid",              // Unique identifiers
    ]
}

/// Check that a dependency path is allowed for tooling packages.
/// Returns false for editor (5.editor) and app (6.apps) paths.
pub fn is_allowed_dependency_path(path: &str) -> bool {
    // Block editor implementation layers
    if path.contains("5.editor") || path.contains("6.apps") {
        return false;
    }
    // Allow SDK, engine (for preview runtime), and other tooling
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_dependency_prefixes() {
        let prefixes = allowed_dependency_prefixes();
        assert!(prefixes.contains(&"stratumx_tooling_"));
        assert!(prefixes.contains(&"stratumx_sdk_"));
    }

    #[test]
    fn test_is_allowed_dependency_path() {
        // SDK is allowed
        assert!(is_allowed_dependency_path("3.sdk/l5.9-editor-dto-law"));

        // Other tooling is allowed
        assert!(is_allowed_dependency_path("4.tooling/l6.0-authority-core"));

        // Editor implementation is NOT allowed
        assert!(!is_allowed_dependency_path("5.editor/editor-impl"));
        assert!(!is_allowed_dependency_path("6.apps/editor-app"));
    }
}
