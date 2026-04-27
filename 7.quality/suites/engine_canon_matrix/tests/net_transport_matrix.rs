mod common;
use common::*;

#[test]
fn net_transport_send_case_0() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 1],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_1() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 2],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_2() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 3],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_3() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 4],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_4() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 5],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_5() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 6],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_6() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 7],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_7() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 8],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_8() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 9],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_9() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 10],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_10() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 11],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_11() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 12],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_12() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 13],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_13() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 14],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_14() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 15],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_15() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 16],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_16() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 17],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_17() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 18],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_18() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 19],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_19() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 20],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_20() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 21],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_21() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 22],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_22() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 23],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_23() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 24],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
#[test]
fn net_transport_send_case_24() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let mut t = NetTransportService::new(NetTransportConfig {
        max_packet_size_bytes: 128,
    });
    t.open_session(ConnectionHandle(1));
    let m = t
        .send(
            &mut rt,
            NetPacketEnvelope {
                connection: ConnectionHandle(1),
                descriptor: PacketDescriptor {
                    reliable: true,
                    ordered: true,
                },
                lane: PacketLane::Control,
                payload: vec![1u8; 25],
            },
        )
        .unwrap();
    assert_eq!(m.queued_packets, 1);
}
