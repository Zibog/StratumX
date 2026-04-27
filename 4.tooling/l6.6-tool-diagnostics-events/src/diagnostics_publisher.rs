use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticEvent {
    pub event_id: u64,
    pub severity: DiagnosticSeverity,
    pub source: String,
    pub message: String,
    pub timestamp: u64,
    pub command_id: Option<u64>,
}

impl DiagnosticEvent {
    pub fn new(
        event_id: u64,
        severity: DiagnosticSeverity,
        source: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self {
            event_id,
            severity,
            source: source.into(),
            message: message.into(),
            timestamp: now,
            command_id: None,
        }
    }

    pub fn with_command_id(mut self, command_id: u64) -> Self {
        self.command_id = Some(command_id);
        self
    }
}

pub struct DiagnosticsPublisher {
    next_event_id: u64,
    events: Vec<DiagnosticEvent>,
}

impl Default for DiagnosticsPublisher {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsPublisher {
    pub fn new() -> Self {
        Self {
            next_event_id: 1,
            events: Vec::new(),
        }
    }

    pub fn publish(
        &mut self,
        severity: DiagnosticSeverity,
        source: impl Into<String>,
        message: impl Into<String>,
    ) -> u64 {
        let event_id = self.next_event_id;
        self.next_event_id += 1;

        let event = DiagnosticEvent::new(event_id, severity, source, message);
        self.events.push(event);
        event_id
    }

    pub fn publish_with_command(
        &mut self,
        severity: DiagnosticSeverity,
        source: impl Into<String>,
        message: impl Into<String>,
        command_id: u64,
    ) -> u64 {
        let event_id = self.next_event_id;
        self.next_event_id += 1;

        let event =
            DiagnosticEvent::new(event_id, severity, source, message).with_command_id(command_id);
        self.events.push(event);
        event_id
    }

    pub fn events(&self) -> &[DiagnosticEvent] {
        &self.events
    }

    pub fn events_for_command(&self, command_id: u64) -> Vec<&DiagnosticEvent> {
        self.events
            .iter()
            .filter(|e| e.command_id == Some(command_id))
            .collect()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
