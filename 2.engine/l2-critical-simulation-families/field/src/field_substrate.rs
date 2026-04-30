use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FieldId(pub u64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSampleRef {
    pub region_key: (i32, i32, i32),
    pub cell: [u16; 3],
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUpdateDelta {
    pub samples: Vec<FieldSampleRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDirtySummary {
    pub dirty_regions: Vec<(i32, i32, i32)>,
    pub sample_count: usize,
}

type SampleKey = ((i32, i32, i32), [u16; 3]);

#[derive(Debug, Clone)]
pub struct ScalarFieldSubstrate {
    field_id: FieldId,
    samples: BTreeMap<SampleKey, f32>,
}

impl ScalarFieldSubstrate {
    pub fn new(field_id: FieldId) -> EngineCoreResult<Self> {
        if field_id.0 == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "field substrate requires non-zero field id",
            ));
        }
        Ok(Self {
            field_id,
            samples: BTreeMap::new(),
        })
    }

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }

    pub fn sample(&self, region_key: (i32, i32, i32), cell: [u16; 3]) -> Option<f32> {
        self.samples.get(&(region_key, cell)).copied()
    }

    pub fn write_sample(
        &mut self,
        region_key: (i32, i32, i32),
        cell: [u16; 3],
        value: f32,
    ) -> EngineCoreResult<()> {
        validate_sample(cell, value)?;
        self.samples.insert((region_key, cell), value);
        Ok(())
    }

    pub fn apply_delta(&mut self, delta: FieldUpdateDelta) -> EngineCoreResult<FieldDirtySummary> {
        let mut canonical_samples = BTreeMap::new();
        for sample in delta.samples {
            validate_sample(sample.cell, sample.value)?;
            canonical_samples
                .entry((sample.region_key, sample.cell))
                .and_modify(|current: &mut f32| *current = current.max(sample.value))
                .or_insert(sample.value);
        }

        let mut dirty_regions = BTreeSet::new();
        let sample_count = canonical_samples.len();
        for ((region_key, cell), value) in canonical_samples {
            self.samples.insert((region_key, cell), value);
            dirty_regions.insert(region_key);
        }
        Ok(FieldDirtySummary {
            sample_count,
            dirty_regions: dirty_regions.into_iter().collect(),
        })
    }
}

fn validate_sample(cell: [u16; 3], value: f32) -> EngineCoreResult<()> {
    if cell.iter().any(|axis| *axis >= 512) {
        return Err(EngineCoreError::InvalidDescriptor(
            "field sample cell must stay within canonical cell-space bounds",
        ));
    }
    if !value.is_finite() {
        return Err(EngineCoreError::InvalidDescriptor(
            "field sample value must be finite",
        ));
    }
    Ok(())
}
