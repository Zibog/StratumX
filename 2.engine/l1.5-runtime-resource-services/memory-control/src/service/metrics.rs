use super::MemoryControlService;
use crate::pressure::{MemoryDegradeBridge, MemoryMetrics, MemoryPressureSignal, PressureClass};

impl MemoryControlService {
    pub fn metrics(&self) -> MemoryMetrics {
        let total = self.heap_bytes.saturating_add(self.staging_bytes);
        let budget = self
            .config
            .heap_budget_bytes
            .saturating_add(self.config.staging_budget_bytes)
            .max(1);
        let usage = total.saturating_mul(100) / budget;
        let pressure = if usage >= 90 {
            PressureClass::Critical
        } else if usage >= 70 {
            PressureClass::Elevated
        } else {
            PressureClass::Healthy
        };
        MemoryMetrics {
            heap_bytes: self.heap_bytes,
            staging_bytes: self.staging_bytes,
            allocation_count: self.allocations.len(),
            pressure,
        }
    }

    pub fn check_pressure_signal(&mut self) -> Option<MemoryPressureSignal> {
        let metrics = self.metrics();
        if metrics.pressure == self.previous_pressure {
            return None;
        }
        let heap_budget = self.config.heap_budget_bytes.max(1);
        let staging_budget = self.config.staging_budget_bytes.max(1);
        let signal = MemoryPressureSignal {
            previous: self.previous_pressure,
            current: metrics.pressure,
            reserved_heap_bytes: self.heap_bytes,
            reserved_staging_bytes: self.staging_bytes,
            heap_usage_percent: ((self.heap_bytes * 100) / heap_budget) as u8,
            staging_usage_percent: ((self.staging_bytes * 100) / staging_budget) as u8,
            degrade_bridge: MemoryDegradeBridge::from_pressure(metrics.pressure),
        };
        self.previous_pressure = metrics.pressure;
        Some(signal)
    }

    pub fn degrade_bridge(&self) -> MemoryDegradeBridge {
        MemoryDegradeBridge::from_pressure(self.metrics().pressure)
    }
}
