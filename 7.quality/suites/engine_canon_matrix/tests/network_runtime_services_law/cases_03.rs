#[test]
fn stale_session_epoch_fails() {
    let mut service = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 1024,
    });
    let stale_session = service.open_session(ConnectionHandle(1));
    let fresh_session = service.open_session(ConnectionHandle(1));

    let packet = TransportPacket {
        packet_id: engine_net_transport::PacketId(1),
        session_id: fresh_session.session_id,
        session_epoch: stale_session.session_epoch,
        lane_id: engine_net_transport::NetworkLaneId(0),
        sequence: 1,
        payload_digest: 0,
        timestamp_ms: 0,
    };

    assert_ne!(stale_session.session_epoch, fresh_session.session_epoch);
    assert_eq!(
        service.process_packet(&packet),
        DeliveryVerdict::StaleSessionEpoch
    );
}
