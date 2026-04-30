mod common;
use common::*;
use engine_net_transport::DeliveryVerdict;

fn run_send_case(payload_len: usize) {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut transport = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    let session = transport.open_session(ConnectionHandle(1));
    let receipt = transport
        .send_with_session(
            &mut rt,
            &session,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; payload_len],
            },
        )
        .unwrap();

    assert_eq!(receipt.verdict, DeliveryVerdict::Accepted);
    assert_eq!(receipt.sequence, 1);
    assert_eq!(receipt.ack_window.highest_sent_sequence, 1);
}

macro_rules! send_case {
    ($name:ident, $len:expr) => {
        #[test]
        fn $name() {
            run_send_case($len);
        }
    };
}

send_case!(net_transport_send_case_0, 1);
send_case!(net_transport_send_case_1, 2);
send_case!(net_transport_send_case_2, 3);
send_case!(net_transport_send_case_3, 4);
send_case!(net_transport_send_case_4, 5);
send_case!(net_transport_send_case_5, 6);
send_case!(net_transport_send_case_6, 7);
send_case!(net_transport_send_case_7, 8);
send_case!(net_transport_send_case_8, 9);
send_case!(net_transport_send_case_9, 10);
send_case!(net_transport_send_case_10, 11);
send_case!(net_transport_send_case_11, 12);
send_case!(net_transport_send_case_12, 13);
send_case!(net_transport_send_case_13, 14);
send_case!(net_transport_send_case_14, 15);
send_case!(net_transport_send_case_15, 16);
send_case!(net_transport_send_case_16, 17);
send_case!(net_transport_send_case_17, 18);
send_case!(net_transport_send_case_18, 19);
send_case!(net_transport_send_case_19, 20);
send_case!(net_transport_send_case_20, 21);
send_case!(net_transport_send_case_21, 22);
send_case!(net_transport_send_case_22, 23);
send_case!(net_transport_send_case_23, 24);
send_case!(net_transport_send_case_24, 25);
