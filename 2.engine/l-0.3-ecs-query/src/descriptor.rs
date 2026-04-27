use crate::types::{
    AccessDescriptor, FilterConstraint, FilterOp, JoinRule, Partitionability, QueryAccessMode,
    QueryLocality,
};
use engine_core::{ComponentTypeId, EngineCoreError, EngineCoreResult};
use engine_ecs_registry::RegistryModel;
use engine_identity::EntityId;
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryDescriptor {
    pub component_set: SmallVec<[ComponentTypeId; 8]>,
    pub locality: QueryLocality,
    pub partitionability: Partitionability,
    pub cache_key: u64,
    pub access: AccessDescriptor,
    pub filters: SmallVec<[FilterConstraint; 4]>,
    pub joins: SmallVec<[JoinRule; 2]>,
}

impl QueryDescriptor {
    pub fn validate_legality(&self) -> EngineCoreResult<()> {
        if self.component_set.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "component set must be explicit and non-empty",
            ));
        }
        if self.access.mode.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "access mode must be explicit",
            ));
        }
        if self.access.mode.contains(QueryAccessMode::WRITE) && self.access.scratch.is_none() {
            return Err(EngineCoreError::InvalidDescriptor(
                "write legality requires explicit scratch class",
            ));
        }
        if self.cache_key == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "cache key must be explicit and stable",
            ));
        }
        Ok(())
    }

    pub fn execute(&self, registry: &RegistryModel) -> EngineCoreResult<Vec<EntityId>> {
        self.validate_legality()?;
        let mut output = Vec::new();
        for entity in registry.entities() {
            let has_all = self
                .component_set
                .iter()
                .all(|component| registry.has_component(entity, *component));
            if !has_all {
                continue;
            }
            let filters_ok = self.filters.iter().all(|filter| match filter.op {
                FilterOp::With => registry.has_component(entity, filter.component),
                FilterOp::Without => !registry.has_component(entity, filter.component),
            });
            if !filters_ok {
                continue;
            }
            let joins_ok = self.joins.iter().all(|join| {
                join.required
                    .iter()
                    .all(|component| registry.has_component(entity, *component))
            });
            if joins_ok {
                output.push(entity);
            }
        }
        Ok(output)
    }
}
