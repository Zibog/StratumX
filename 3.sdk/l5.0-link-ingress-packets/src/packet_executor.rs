use crate::VerticalSliceIngressPacket;
use link_egress_observations::VerticalSliceObservation;

// REAL RUNTIME PATH: PacketExecutor now owns VerticalSliceSession
// No more synthetic empty observations - all commands go through real engine runtime

pub struct PacketExecutor {
    last_request_id: u64,
    vertical_slice_session: Option<Box<dyn VerticalSliceSessionHandle>>,
}

// Forward declaration - actual implementation in tooling layer
// This allows SDK to call into tooling without circular dependency
pub trait VerticalSliceSessionHandle: Send {
    fn handle_command(
        &mut self,
        packet: VerticalSliceIngressPacket,
    ) -> Result<VerticalSliceObservation, String>;
}

impl Default for PacketExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketExecutor {
    pub fn new() -> Self {
        Self {
            last_request_id: 0,
            vertical_slice_session: None,
        }
    }

    pub fn set_vertical_slice_session(&mut self, session: Box<dyn VerticalSliceSessionHandle>) {
        self.vertical_slice_session = Some(session);
    }

    pub fn execute_packet(
        &mut self,
        packet: VerticalSliceIngressPacket,
    ) -> Result<VerticalSliceObservation, String> {
        self.last_request_id = packet.request_id;

        // REAL PATH: Delegate to actual vertical slice runtime session
        if let Some(session) = &mut self.vertical_slice_session {
            return session.handle_command(packet);
        }

        // HONEST FAILURE: No session means no execution, not fake success
        Err(format!(
            "No vertical slice session initialized for command {:?}",
            packet.command
        ))
    }

    pub fn last_request_id(&self) -> u64 {
        self.last_request_id
    }

    pub fn has_session(&self) -> bool {
        self.vertical_slice_session.is_some()
    }
}
