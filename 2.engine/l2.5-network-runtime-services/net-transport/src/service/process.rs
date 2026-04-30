use super::NetTransportService;
use crate::failure::map_reject_reason_to_verdict;
use crate::{DeliveryVerdict, SequenceAcceptancePolicy, SessionState, TransportPacket};

impl NetTransportService {
    /// Process an inbound packet using the default monotonic sequence policy.
    pub fn process_packet(&mut self, packet: &TransportPacket) -> DeliveryVerdict {
        self.process_packet_with_policy(packet, SequenceAcceptancePolicy::default())
    }

    /// Process an inbound packet with an explicit sequence acceptance policy.
    pub fn process_packet_with_policy(
        &mut self,
        packet: &TransportPacket,
        policy: SequenceAcceptancePolicy,
    ) -> DeliveryVerdict {
        if let Err(reason) = packet.validate() {
            return map_reject_reason_to_verdict(reason);
        }

        {
            let Some(record) = self.sessions_by_id.get(&packet.session_id) else {
                return DeliveryVerdict::InvalidSession;
            };

            if record.session.session_epoch != packet.session_epoch {
                return DeliveryVerdict::StaleSessionEpoch;
            }
            if record.session.state != SessionState::Active {
                return DeliveryVerdict::InvalidSession;
            }
            if self.session_by_connection.get(&record.session.connection)
                != Some(&packet.session_id)
            {
                return DeliveryVerdict::InvalidSession;
            }
        }

        let Some(record) = self.sessions_by_id.get_mut(&packet.session_id) else {
            return DeliveryVerdict::InvalidSession;
        };
        if record.processed_sequences.contains(&packet.sequence) {
            return DeliveryVerdict::Duplicate;
        }
        if let Err(reason) = policy.validate(record.last_processed_sequence, packet.sequence) {
            return map_reject_reason_to_verdict(reason);
        }

        record.last_processed_sequence = packet.sequence;
        record.processed_sequences.insert(packet.sequence);
        DeliveryVerdict::Accepted
    }
}
