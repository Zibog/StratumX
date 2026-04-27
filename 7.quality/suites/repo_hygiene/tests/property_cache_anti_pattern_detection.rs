// Property 18: Cache Anti-Pattern Detection
// Validates: Requirements 7.5, 11.4
//
// This property test verifies that the CacheRebuildabilityValidator correctly detects:
// 1. get_or_create_default_truth patterns (authoritative caches)
// 2. Caches as single source of truth (not rebuildable)
// 3. Queries mutating state (should be read-only)

use proptest::prelude::*;
use repo_hygiene::{
    CacheRebuildabilityValidator, CacheViolationType, CodebaseState, OwnerInventory,
    OwnerInventoryEntry, StateClassification, StateField,
};

// Strategy to generate cache fields
#[allow(dead_code)]
fn cache_field_strategy() -> impl Strategy<Value = StateField> {
    (
        prop::sample::select(vec!["cache", "Cache"]),
        "[a-z_]{3,15}",
        prop::bool::ANY,
        "[A-Z][a-zA-Z]{3,15}",
    )
        .prop_map(|(cache_suffix, base_name, has_trait, owner_type)| {
            let name = format!("{}_{}", base_name, cache_suffix);
            let file_path = format!(
                "5.editor/editor-state-containers/src/cache/{}.rs",
                base_name
            );
            let field_type = if has_trait {
                format!("impl RebuildableCache<Key, {}>", owner_type)
            } else {
                format!("HashMap<Key, {}>", owner_type)
            };
            StateField {
                name,
                file_path,
                owner_type,
                field_type,
            }
        })
}

// Strategy to generate query fields
#[allow(dead_code)]
fn query_field_strategy() -> impl Strategy<Value = StateField> {
    (
        prop::sample::select(vec!["query", "Query", "view", "View"]),
        "[a-z_]{3,15}",
        prop::bool::ANY,
    )
        .prop_map(|(query_suffix, base_name, is_mutable)| {
            let name = format!("{}_{}", base_name, query_suffix);
            let file_path = format!(
                "5.editor/editor-state-containers/src/queries/{}.rs",
                base_name
            );
            let field_type = if is_mutable {
                "fn build(&mut self, owner: &mut Owner) -> View".to_string()
            } else {
                "fn build(owner: &Owner) -> View".to_string()
            };
            StateField {
                name,
                file_path,
                owner_type: format!("{}View", base_name),
                field_type,
            }
        })
}

// Strategy to generate inventory entries
fn inventory_entry_strategy() -> impl Strategy<Value = OwnerInventoryEntry> {
    (
        "[a-z_]{3,15}",
        "[A-Z][a-zA-Z]{3,15}",
        prop::sample::select(vec![
            StateClassification::Persistable,
            StateClassification::Transient,
            StateClassification::Derived,
        ]),
    )
        .prop_map(|(name, owner, classification)| OwnerInventoryEntry {
            entity_name: name.clone(),
            current_path: format!("5.editor/src/{}.rs", name),
            current_owner: owner.clone(),
            target_owner: owner,
            classification,
            can_mutate: vec!["Service".to_string()],
            can_read: vec!["QueryLayer".to_string()],
            publishes_changes: None,
            rebuilds_cache: None,
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property: Validator detects caches without RebuildableCache trait
    ///
    /// Given: A cache field in the cache directory
    /// When: The cache does not implement RebuildableCache trait
    /// Then: Validator detects MissingRebuildableTrait violation
    #[test]
    fn property_validator_detects_missing_rebuildable_trait(
        inventory_entries in prop::collection::vec(inventory_entry_strategy(), 1..5),
        base_name in "[a-z_]{3,15}",
    ) {
        let cache_name = format!("{}_cache", base_name);
        let cache_field = StateField {
            name: cache_name.clone(),
            file_path: format!("5.editor/editor-state-containers/src/cache/{}.rs", base_name),
            owner_type: format!("{}Cache", base_name),
            field_type: "HashMap<Key, Value>".to_string(), // No RebuildableCache trait
        };

        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: inventory_entries,
        };

        let codebase_state = CodebaseState {
            state_fields: vec![cache_field.clone()],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        // Should detect violation
        prop_assert!(result.is_err());
        let violations = result.unwrap_err();
        let has_missing_trait_violation = violations.iter().any(|v| {
            v.violation_type == CacheViolationType::MissingRebuildableTrait
                && v.cache_name == cache_name
        });
        prop_assert!(has_missing_trait_violation);
    }

    /// Property: Validator detects get_or_create_default_truth pattern
    ///
    /// Given: A cache field with authoritative pattern
    /// When: Field type contains get_or_create_default_truth or similar
    /// Then: Validator detects AuthoritativeCache violation
    #[test]
    fn property_validator_detects_authoritative_cache_pattern(
        inventory_entries in prop::collection::vec(inventory_entry_strategy(), 1..5),
        base_name in "[a-z_]{3,15}",
        anti_pattern in prop::sample::select(vec![
            "get_or_create_default_truth",
            "get_or_insert_default",
            "cache_as_source_of_truth",
            "create_if_missing",
        ]),
    ) {
        let cache_name = format!("{}_cache", base_name);
        let cache_field = StateField {
            name: cache_name.clone(),
            file_path: format!("5.editor/editor-state-containers/src/cache/{}.rs", base_name),
            owner_type: format!("{}Cache", base_name),
            field_type: format!("{}<Key, Value>", anti_pattern),
        };

        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: inventory_entries,
        };

        let codebase_state = CodebaseState {
            state_fields: vec![cache_field.clone()],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        // Should detect violation
        prop_assert!(result.is_err());
        let violations = result.unwrap_err();
        let has_authoritative_violation = violations.iter().any(|v| {
            v.violation_type == CacheViolationType::AuthoritativeCache
                && v.cache_name == cache_name
        });
        prop_assert!(has_authoritative_violation);
    }

    /// Property: Validator detects cache in owner directory
    ///
    /// Given: A cache field in the owners directory
    /// When: Cache is mixed with authoritative state
    /// Then: Validator detects AuthoritativeCache violation
    #[test]
    fn property_validator_detects_cache_in_owner_directory(
        inventory_entries in prop::collection::vec(inventory_entry_strategy(), 1..5),
        base_name in "[a-z_]{3,15}",
    ) {
        let cache_name = format!("{}_cache", base_name);
        let cache_field = StateField {
            name: cache_name.clone(),
            file_path: format!("5.editor/editor-state-containers/src/owners/{}_owner.rs", base_name),
            owner_type: format!("{}Owner", base_name),
            field_type: "HashMap<String, CachedData>".to_string(),
        };

        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: inventory_entries,
        };

        let codebase_state = CodebaseState {
            state_fields: vec![cache_field.clone()],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        // Should detect violation
        prop_assert!(result.is_err());
        let violations = result.unwrap_err();
        let has_authoritative_violation = violations.iter().any(|v| {
            v.violation_type == CacheViolationType::AuthoritativeCache
                && v.cache_name == cache_name
        });
        prop_assert!(has_authoritative_violation);
    }

    /// Property: Validator detects queries mutating state
    ///
    /// Given: A query field in the queries directory
    /// When: Query method signature contains mutation (&mut)
    /// Then: Validator detects QueryMutatesState violation
    #[test]
    fn property_validator_detects_query_mutation(
        inventory_entries in prop::collection::vec(inventory_entry_strategy(), 1..5),
        base_name in "[a-z_]{3,15}",
        mutation_pattern in prop::sample::select(vec![
            "&mut",
            "mut self",
            "set_",
            "update_",
            "modify_",
        ]),
    ) {
        let query_name = format!("{}_query", base_name);
        let query_field = StateField {
            name: query_name.clone(),
            file_path: format!("5.editor/editor-state-containers/src/queries/{}.rs", base_name),
            owner_type: format!("{}View", base_name),
            field_type: format!("fn build({} owner: &Owner) -> View", mutation_pattern),
        };

        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: inventory_entries,
        };

        let codebase_state = CodebaseState {
            state_fields: vec![query_field.clone()],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        // Should detect violation
        prop_assert!(result.is_err());
        let violations = result.unwrap_err();
        let has_mutation_violation = violations.iter().any(|v| {
            v.violation_type == CacheViolationType::QueryMutatesState
                && v.cache_name == query_name
        });
        prop_assert!(has_mutation_violation);
    }

    /// Property: Validator passes with valid caches and queries
    ///
    /// Given: Caches with RebuildableCache trait and read-only queries
    /// When: No anti-patterns are present
    /// Then: Validator returns Ok
    #[test]
    fn property_validator_passes_with_valid_caches(
        inventory_entries in prop::collection::vec(inventory_entry_strategy(), 1..5),
        base_name in "[a-z_]{3,15}",
    ) {
        let cache_name = format!("{}_cache", base_name);
        let cache_field = StateField {
            name: cache_name.clone(),
            file_path: format!("5.editor/editor-state-containers/src/cache/{}.rs", base_name),
            owner_type: format!("{}Cache", base_name),
            field_type: "impl RebuildableCache<Key, Value>".to_string(),
        };

        let query_name = format!("{}_view", base_name);
        let query_field = StateField {
            name: query_name.clone(),
            file_path: format!("5.editor/editor-state-containers/src/queries/{}.rs", base_name),
            owner_type: format!("{}View", base_name),
            field_type: "fn build(owner: &Owner) -> View".to_string(),
        };

        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: inventory_entries,
        };

        let codebase_state = CodebaseState {
            state_fields: vec![cache_field, query_field],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        // Should pass validation
        prop_assert!(result.is_ok());
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_missing_rebuildable_trait_detected() {
        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![],
        };

        let codebase_state = CodebaseState {
            state_fields: vec![StateField {
                name: "material_cache".to_string(),
                file_path: "5.editor/editor-state-containers/src/cache/material.rs".to_string(),
                owner_type: "MaterialCache".to_string(),
                field_type: "HashMap<Key, Value>".to_string(),
            }],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == CacheViolationType::MissingRebuildableTrait));
    }

    #[test]
    fn test_authoritative_cache_pattern_detected() {
        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![],
        };

        let codebase_state = CodebaseState {
            state_fields: vec![StateField {
                name: "material_cache".to_string(),
                file_path: "5.editor/editor-state-containers/src/cache/material.rs".to_string(),
                owner_type: "MaterialCache".to_string(),
                field_type: "get_or_create_default_truth<Material>".to_string(),
            }],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == CacheViolationType::AuthoritativeCache));
    }

    #[test]
    fn test_cache_in_owner_directory_detected() {
        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![],
        };

        let codebase_state = CodebaseState {
            state_fields: vec![StateField {
                name: "internal_cache".to_string(),
                file_path: "5.editor/editor-state-containers/src/owners/project_owner.rs"
                    .to_string(),
                owner_type: "ProjectOwner".to_string(),
                field_type: "HashMap<String, CachedData>".to_string(),
            }],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == CacheViolationType::AuthoritativeCache));
    }

    #[test]
    fn test_query_mutation_detected() {
        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![],
        };

        let codebase_state = CodebaseState {
            state_fields: vec![StateField {
                name: "project_view".to_string(),
                file_path: "5.editor/editor-state-containers/src/queries/project_queries.rs"
                    .to_string(),
                owner_type: "ProjectView".to_string(),
                field_type: "fn build(&mut self, owner: &mut Owner) -> View".to_string(),
            }],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == CacheViolationType::QueryMutatesState));
    }

    #[test]
    fn test_valid_cache_and_query_pass() {
        let _inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![],
        };

        let codebase_state = CodebaseState {
            state_fields: vec![
                StateField {
                    name: "material_cache".to_string(),
                    file_path: "5.editor/editor-state-containers/src/cache/material.rs".to_string(),
                    owner_type: "MaterialCache".to_string(),
                    field_type: "impl RebuildableCache<Key, Value>".to_string(),
                },
                StateField {
                    name: "project_view".to_string(),
                    file_path: "5.editor/editor-state-containers/src/queries/project_queries.rs"
                        .to_string(),
                    owner_type: "ProjectView".to_string(),
                    field_type: "fn build(owner: &Owner) -> View".to_string(),
                },
            ],
        };

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_ok());
    }
}
