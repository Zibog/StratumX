use crate::CodebaseState;

/// Validator for cache rebuildability rules
pub struct CacheRebuildabilityValidator;

impl Default for CacheRebuildabilityValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheRebuildabilityValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validate cache rebuildability rules
    pub fn validate(&self, codebase_state: &CodebaseState) -> Result<(), Vec<CacheViolation>> {
        let mut violations = Vec::new();

        // Check that all caches implement RebuildableCache trait
        if let Err(mut v) = self.check_all_caches_implement_trait(codebase_state) {
            violations.append(&mut v);
        }

        // Check that caches are not authoritative (can be cleared and rebuilt)
        if let Err(mut v) = self.check_caches_not_authoritative(codebase_state) {
            violations.append(&mut v);
        }

        // Check that queries are read-only (no mutation logic)
        if let Err(mut v) = self.check_queries_read_only(codebase_state) {
            violations.append(&mut v);
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    /// Check that all cache types implement RebuildableCache trait
    fn check_all_caches_implement_trait(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<CacheViolation>> {
        let mut violations = Vec::new();

        // Cache naming patterns
        let cache_patterns = ["Cache", "cache"];

        for field in &codebase_state.state_fields {
            // Check if this looks like a cache
            let is_cache = cache_patterns
                .iter()
                .any(|pattern| field.name.contains(pattern) || field.field_type.contains(pattern));

            // Check if it's in the cache directory
            let is_in_cache_dir = field.file_path.contains("cache/");

            if is_cache && is_in_cache_dir {
                // Check if the type implements RebuildableCache
                // This is a heuristic: we look for the trait implementation
                // In practice, we'd need to parse the actual code or use compiler metadata
                let implements_rebuildable = field.field_type.contains("RebuildableCache")
                    || field.owner_type.contains("RebuildableCache");

                if !implements_rebuildable {
                    violations.push(CacheViolation {
                        cache_name: field.name.clone(),
                        file_path: field.file_path.clone(),
                        violation_type: CacheViolationType::MissingRebuildableTrait,
                        message: format!(
                            "Cache '{}' in {} does not implement RebuildableCache trait. All caches must be rebuildable.",
                            field.name, field.file_path
                        ),
                    });
                }
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    /// Check that caches are not authoritative (can be cleared and rebuilt from owners)
    fn check_caches_not_authoritative(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<CacheViolation>> {
        let mut violations = Vec::new();

        // Anti-patterns that suggest authoritative caches
        let authoritative_patterns = vec![
            "get_or_create_default_truth",
            "get_or_insert_default",
            "cache_as_source_of_truth",
            "create_if_missing",
        ];

        for field in &codebase_state.state_fields {
            // Check if this is a cache field
            if field.name.contains("cache") || field.field_type.contains("Cache") {
                // Check for authoritative patterns in field type or name
                for pattern in &authoritative_patterns {
                    if field.field_type.contains(pattern) || field.name.contains(pattern) {
                        violations.push(CacheViolation {
                            cache_name: field.name.clone(),
                            file_path: field.file_path.clone(),
                            violation_type: CacheViolationType::AuthoritativeCache,
                            message: format!(
                                "Cache '{}' in {} contains authoritative pattern '{}'. Caches must be rebuildable from owners, not single source of truth.",
                                field.name, field.file_path, pattern
                            ),
                        });
                    }
                }

                // Check if cache is in owner directory (caches should be separate)
                if field.file_path.contains("owners/") {
                    violations.push(CacheViolation {
                        cache_name: field.name.clone(),
                        file_path: field.file_path.clone(),
                        violation_type: CacheViolationType::AuthoritativeCache,
                        message: format!(
                            "Cache '{}' found in owner directory at {}. Caches should be in cache/ directory, not mixed with authoritative state.",
                            field.name, field.file_path
                        ),
                    });
                }
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    /// Check that queries are read-only (take &Owner, not &mut Owner)
    fn check_queries_read_only(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<CacheViolation>> {
        let mut violations = Vec::new();

        // Query naming patterns
        let query_patterns = ["Query", "query", "View", "view"];

        for field in &codebase_state.state_fields {
            // Check if this looks like a query
            let is_query = query_patterns
                .iter()
                .any(|pattern| field.name.contains(pattern) || field.field_type.contains(pattern));

            // Check if it's in the queries directory
            let is_in_queries_dir = field.file_path.contains("queries/");

            if is_query && is_in_queries_dir {
                // Check for mutation patterns
                let mutation_patterns = vec!["&mut", "mut self", "set_", "update_", "modify_"];

                for pattern in &mutation_patterns {
                    if field.field_type.contains(pattern) {
                        violations.push(CacheViolation {
                            cache_name: field.name.clone(),
                            file_path: field.file_path.clone(),
                            violation_type: CacheViolationType::QueryMutatesState,
                            message: format!(
                                "Query '{}' in {} contains mutation pattern '{}'. Queries must be read-only.",
                                field.name, field.file_path, pattern
                            ),
                        });
                    }
                }
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

/// Represents a violation of cache rebuildability rules
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheViolation {
    pub cache_name: String,
    pub file_path: String,
    pub violation_type: CacheViolationType,
    pub message: String,
}

/// Types of cache violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheViolationType {
    /// Cache does not implement RebuildableCache trait
    MissingRebuildableTrait,
    /// Cache is authoritative (single source of truth)
    AuthoritativeCache,
    /// Query mutates state instead of being read-only
    QueryMutatesState,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OwnerInventory, OwnerInventoryEntry, StateClassification, StateField};

    fn create_test_inventory() -> OwnerInventory {
        OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![OwnerInventoryEntry {
                entity_name: "material_registry_cache".to_string(),
                current_path: "5.editor/editor-state-containers/src/cache/rebuildable_caches.rs"
                    .to_string(),
                current_owner: "CacheLayer".to_string(),
                target_owner: "CacheLayer".to_string(),
                classification: StateClassification::Derived,
                can_mutate: vec!["CacheLayer".to_string()],
                can_read: vec!["QueryLayer".to_string()],
                publishes_changes: None,
                rebuilds_cache: None,
            }],
        }
    }

    fn create_test_codebase_state() -> CodebaseState {
        CodebaseState {
            state_fields: vec![StateField {
                name: "material_registry_cache".to_string(),
                file_path: "5.editor/editor-state-containers/src/cache/rebuildable_caches.rs"
                    .to_string(),
                owner_type: "MaterialRegistryCache".to_string(),
                field_type: "impl RebuildableCache<Key, Value>".to_string(),
            }],
        }
    }

    #[test]
    fn test_validator_detects_missing_rebuildable_trait() {
        let _inventory = create_test_inventory();
        let mut codebase_state = create_test_codebase_state();

        // Add cache without RebuildableCache trait
        codebase_state.state_fields.push(StateField {
            name: "terrain_preview_cache".to_string(),
            file_path: "5.editor/editor-state-containers/src/cache/terrain_cache.rs".to_string(),
            owner_type: "TerrainPreviewCache".to_string(),
            field_type: "HashMap<ChunkId, PreviewData>".to_string(),
        });

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == CacheViolationType::MissingRebuildableTrait));
    }

    #[test]
    fn test_validator_detects_authoritative_cache() {
        let _inventory = create_test_inventory();
        let mut codebase_state = create_test_codebase_state();

        // Add cache with authoritative pattern
        codebase_state.state_fields.push(StateField {
            name: "material_cache".to_string(),
            file_path: "5.editor/editor-state-containers/src/cache/material_cache.rs".to_string(),
            owner_type: "MaterialCache".to_string(),
            field_type: "get_or_create_default_truth<Material>".to_string(),
        });

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == CacheViolationType::AuthoritativeCache));
    }

    #[test]
    fn test_validator_detects_cache_in_owner_directory() {
        let _inventory = create_test_inventory();
        let mut codebase_state = create_test_codebase_state();

        // Add cache in owner directory
        codebase_state.state_fields.push(StateField {
            name: "internal_cache".to_string(),
            file_path: "5.editor/editor-state-containers/src/owners/project_owner.rs".to_string(),
            owner_type: "ProjectOwner".to_string(),
            field_type: "HashMap<String, CachedData>".to_string(),
        });

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == CacheViolationType::AuthoritativeCache));
    }

    #[test]
    fn test_validator_detects_query_mutation() {
        let _inventory = create_test_inventory();
        let mut codebase_state = create_test_codebase_state();

        // Add query with mutation
        codebase_state.state_fields.push(StateField {
            name: "project_identity_view".to_string(),
            file_path: "5.editor/editor-state-containers/src/queries/project_queries.rs"
                .to_string(),
            owner_type: "ProjectIdentityView".to_string(),
            field_type: "fn build(&mut self, owner: &mut ProjectOwner)".to_string(),
        });

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == CacheViolationType::QueryMutatesState));
    }

    #[test]
    fn test_validator_passes_with_valid_caches() {
        let _inventory = create_test_inventory();
        let codebase_state = create_test_codebase_state();

        let validator = CacheRebuildabilityValidator::new();
        let result = validator.validate(&codebase_state);

        assert!(result.is_ok());
    }
}
