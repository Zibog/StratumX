use super::NetTransportService;
use crate::{
    AckWindow, DeliveryVerdict, LanePolicy, NetPacketEnvelope, NetworkFailureReason,
    PacketDeliveryReceipt, PacketId, TransportSession,
};
use engine_core::EngineCoreResult;
use engine_runtime::{ConnectionKey, RuntimeKernel};

impl NetTransportService {
    /// Send packet through an active transport session.
    pub fn send_with_session(
        &mut self,
        runtime: &mut RuntimeKernel,
        session: &TransportSession,
        envelope: NetPacketEnvelope,
    ) -> EngineCoreResult<PacketDeliveryReceipt> {
        self.validate_outbound_envelope(session, &envelope)
            .map_err(NetworkFailureReason::as_engine_error)?;

        let sequence = self
            .sessions_by_id
            .get(&session.session_id)
            .ok_or_else(|| NetworkFailureReason::InvalidSession.as_engine_error())?
            .next_sequence;
        let connection = envelope.connection;
        let payload = envelope.payload;

        runtime.enqueue_connection_publication(ConnectionKey(connection.0), payload)?;

        let record = self
            .sessions_by_id
            .get_mut(&session.session_id)
            .ok_or_else(|| NetworkFailureReason::InvalidSession.as_engine_error())?;
        record.next_sequence = record.next_sequence.saturating_add(1);
        record.in_flight_sequences.insert(sequence);

        let packet_id = PacketId(self.next_packet_id);
        self.next_packet_id = self.next_packet_id.saturating_add(1);

        Ok(PacketDeliveryReceipt {
            packet_id,
            session_id: session.session_id,
            session_epoch: session.session_epoch,
            verdict: DeliveryVerdict::Accepted,
            sequence,
            ack_window: AckWindow {
                highest_sent_sequence: sequence,
                highest_acked_sequence: record.last_acked_sequence,
                in_flight_packets: record.in_flight_sequences.len(),
            },
        })
    }

    fn validate_outbound_envelope(
        &self,
        session: &TransportSession,
        envelope: &NetPacketEnvelope,
    ) -> Result<(), NetworkFailureReason> {
        self.validate_active_session(session)?;

        if envelope.connection != session.connection {
            return Err(NetworkFailureReason::ConnectionMismatch);
        }

        LanePolicy::for_lane(envelope.lane).validate(&envelope.descriptor)?;

        if envelope.payload.len() > self.config.max_packet_size_bytes {
            return Err(NetworkFailureReason::PacketTooLarge);
        }

        Ok(())
    }
}
