mod error;
mod model;
mod pressure;
mod service;

pub use error::{MemoryControlResult, MemoryFailure, MemoryFailureReason};
pub use model::{
    AllocationDescriptor, MemoryAllocationId, MemoryAllocationPool, MemoryAllocationRecord,
    MemoryConfig, MemoryReleaseReceipt, MemoryReservationReceipt,
};
pub use pressure::{MemoryDegradeBridge, MemoryMetrics, MemoryPressureSignal, PressureClass};
pub use service::MemoryControlService;
