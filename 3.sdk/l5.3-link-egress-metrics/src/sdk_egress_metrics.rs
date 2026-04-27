use engine_handle_refs::RuntimeHandle;
use sdk_compat::CompatibilityProfile;

pub const MAX_BATCH_RECORDS: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetricBatchId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetricKind {
    Counter,
    Gauge,
    Histogram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricQualityFlag {
    Full,
    Degraded,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricRecord {
    pub cursor: u64,
    pub epoch: u64,
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub kind: MetricKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricBatch {
    pub metric_batch_id: MetricBatchId,
    pub metric_kind_set: Vec<MetricKind>,
    pub aggregation_window: u32,
    pub source_runtime_handle: RuntimeHandle,
    pub publication_cursor: u64,
    pub quality_flag: MetricQualityFlag,
    pub profile: CompatibilityProfile,
    pub records: Vec<MetricRecord>,
}

pub fn batch_after(batch: &MetricBatch, cursor: u64, max_records: usize) -> MetricBatch {
    let max_records = max_records.clamp(1, MAX_BATCH_RECORDS);
    let records = batch
        .records
        .iter()
        .filter(|record| record.cursor > cursor)
        .take(max_records)
        .cloned()
        .collect::<Vec<_>>();
    let publication_cursor = records.last().map(|record| record.cursor).unwrap_or(cursor);
    MetricBatch {
        publication_cursor,
        records,
        ..batch.clone()
    }
}
