use crate::{ChunkAccessMode, ChunkInvalidationLaw, LayoutClass, LocalityClass};
use engine_core::{ComponentTypeId, EngineCoreError, EngineCoreResult, Invariant};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkDescriptor {
    pub signature: SmallVec<[ComponentTypeId; 8]>,
    pub access_mode: ChunkAccessMode,
    pub invalidation_law: ChunkInvalidationLaw,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnDescriptor {
    pub family: SmallVec<[ComponentTypeId; 8]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageLayoutDescriptor {
    pub layout_class: LayoutClass,
    pub chunk: Option<ChunkDescriptor>,
    pub columns: SmallVec<[ColumnDescriptor; 4]>,
    pub locality_class: LocalityClass,
}

impl Invariant for StorageLayoutDescriptor {
    fn check_invariants(&self) -> EngineCoreResult<()> {
        if matches!(self.layout_class, LayoutClass::ChunkDense) {
            let chunk = self
                .chunk
                .as_ref()
                .ok_or(EngineCoreError::InvalidDescriptor(
                    "chunk descriptor required",
                ))?;
            if chunk.signature.is_empty() {
                return Err(EngineCoreError::InvalidDescriptor(
                    "chunk signature must be frozen and non-empty",
                ));
            }
            if chunk.access_mode.is_empty() {
                return Err(EngineCoreError::InvalidDescriptor(
                    "chunk access mode must be frozen",
                ));
            }
        }

        if matches!(self.layout_class, LayoutClass::Columnar) && self.columns.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "columnar layout requires at least one column family",
            ));
        }

        Ok(())
    }
}
