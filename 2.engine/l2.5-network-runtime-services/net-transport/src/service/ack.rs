use super::NetTransportService;
use crate::{AckWindow, NetworkFailureReason, TransportSession};

impl NetTransportService {
    /// Advance the stateful acknowledgement window for an active session.
    pub fn acknowledge(
        &mut self,
        session: &TransportSession,
        sequence: u64,
    ) -> Result<AckWindow, NetworkFailureReason> {
        self.validate_active_session(session)?;
        let record = self
            .sessions_by_id
            .get_mut(&session.session_id)
            .ok_or(NetworkFailureReason::InvalidSession)?;

        let highest_sent_sequence = record.next_sequence.saturating_sub(1);
        if sequence == 0 || sequence > highest_sent_sequence {
            return Err(NetworkFailureReason::AckOutOfWindow);
        }
        if sequence <= record.last_acked_sequence {
            return Err(NetworkFailureReason::AckRegression);
        }
        if !record.in_flight_sequences.contains(&sequence) {
            return Err(NetworkFailureReason::AckOutOfWindow);
        }

        record.last_acked_sequence = sequence;
        record
            .in_flight_sequences
            .retain(|in_flight| *in_flight > sequence);

        Ok(AckWindow {
            highest_sent_sequence,
            highest_acked_sequence: record.last_acked_sequence,
            in_flight_packets: record.in_flight_sequences.len(),
        })
    }
}
