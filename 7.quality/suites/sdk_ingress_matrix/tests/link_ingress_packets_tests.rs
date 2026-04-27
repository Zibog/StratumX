use engine_handle_refs::SessionHandle;
use link_ingress_packets::*;
use sdk_compat::{BridgeVersion, CompatVersionId};
use transport_policies::{
    default_transport_policy, FramingKind, TransportPolicy, TransportPolicyId,
};

// ============================================================================
// Constants
// ============================================================================

#[test]
fn test_canonical_level_constant() {
    assert_eq!(CANONICAL_LEVEL, "L5.0");
}

// ============================================================================
// PacketId Tests
// ============================================================================

#[test]
fn test_packet_id_new() {
    let id = PacketId(42);
    assert_eq!(id.0, 42);
}

#[test]
fn test_packet_id_equality() {
    let a = PacketId(1);
    let b = PacketId(1);
    assert_eq!(a, b);
}

#[test]
fn test_packet_id_ordering() {
    let a = PacketId(1);
    let b = PacketId(2);
    assert!(a < b);
}

#[test]
fn test_packet_id_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let id = PacketId(100);
    let mut h1 = DefaultHasher::new();
    id.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    id.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ============================================================================
// PacketDecodeStatus Tests
// ============================================================================

#[test]
fn test_decode_status_pending() {
    let status = PacketDecodeStatus::Pending;
    assert!(matches!(status, PacketDecodeStatus::Pending));
}

#[test]
fn test_decode_status_accepted() {
    let status = PacketDecodeStatus::Accepted;
    assert!(matches!(status, PacketDecodeStatus::Accepted));
}

#[test]
fn test_decode_status_rejected() {
    let status = PacketDecodeStatus::Rejected;
    assert!(matches!(status, PacketDecodeStatus::Rejected));
}

#[test]
fn test_decode_status_equality() {
    let a = PacketDecodeStatus::Accepted;
    let b = PacketDecodeStatus::Accepted;
    assert_eq!(a, b);
}

#[test]
fn test_decode_status_inequality() {
    let a = PacketDecodeStatus::Pending;
    let b = PacketDecodeStatus::Rejected;
    assert_ne!(a, b);
}

#[test]
fn test_decode_status_ordering() {
    // Pending < Accepted < Rejected (lexicographic by enum variant)
    assert!(PacketDecodeStatus::Pending < PacketDecodeStatus::Accepted);
    assert!(PacketDecodeStatus::Accepted < PacketDecodeStatus::Rejected);
}

// ============================================================================
// BridgePacket Tests
// ============================================================================

fn make_session_handle() -> SessionHandle {
    SessionHandle::new(1)
}

fn make_transport_policy() -> (TransportPolicyId, TransportPolicy) {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    (policy.transport_policy_id, policy)
}

fn make_compat_version() -> (CompatVersionId, BridgeVersion) {
    (CompatVersionId(1), BridgeVersion::new(1, 0, 0))
}

#[test]
fn test_bridge_packet_creation() {
    let session = make_session_handle();
    let (policy_id, policy) = make_transport_policy();
    let (version_id, version) = make_compat_version();

    let packet = BridgePacket {
        packet_id: PacketId(1),
        session_handle: session,
        transport_policy_id: policy_id,
        transport_policy: policy,
        compat_version_id: version_id,
        compat_version: version,
        payload_bytes: vec![1, 2, 3, 4],
        received_at_tick: 1000,
        decode_status: PacketDecodeStatus::Pending,
    };
    assert_eq!(packet.packet_id, PacketId(1));
    assert_eq!(packet.payload_bytes.len(), 4);
    assert!(matches!(packet.decode_status, PacketDecodeStatus::Pending));
    assert_eq!(packet.received_at_tick, 1000);
}

#[test]
fn test_bridge_packet_empty_payload() {
    let session = make_session_handle();
    let (policy_id, policy) = make_transport_policy();
    let (version_id, version) = make_compat_version();

    let packet = BridgePacket {
        packet_id: PacketId(1),
        session_handle: session,
        transport_policy_id: policy_id,
        transport_policy: policy,
        compat_version_id: version_id,
        compat_version: version,
        payload_bytes: vec![],
        received_at_tick: 0,
        decode_status: PacketDecodeStatus::Pending,
    };
    assert!(packet.payload_bytes.is_empty());
}

#[test]
fn test_bridge_packet_large_payload() {
    let session = make_session_handle();
    let (policy_id, policy) = make_transport_policy();
    let (version_id, version) = make_compat_version();

    let packet = BridgePacket {
        packet_id: PacketId(1),
        session_handle: session,
        transport_policy_id: policy_id,
        transport_policy: policy,
        compat_version_id: version_id,
        compat_version: version,
        payload_bytes: vec![0u8; 1024],
        received_at_tick: 500,
        decode_status: PacketDecodeStatus::Accepted,
    };
    assert_eq!(packet.payload_bytes.len(), 1024);
    assert!(matches!(packet.decode_status, PacketDecodeStatus::Accepted));
}

#[test]
fn test_bridge_packet_equality() {
    let session = make_session_handle();
    let (policy_id, policy) = make_transport_policy();
    let (version_id, version) = make_compat_version();

    let a = BridgePacket {
        packet_id: PacketId(1),
        session_handle: session,
        transport_policy_id: policy_id,
        transport_policy: policy.clone(),
        compat_version_id: version_id,
        compat_version: version,
        payload_bytes: vec![1, 2, 3],
        received_at_tick: 100,
        decode_status: PacketDecodeStatus::Pending,
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_bridge_packet_inequality() {
    let session = make_session_handle();
    let (policy_id, policy) = make_transport_policy();
    let (version_id, version) = make_compat_version();

    let a = BridgePacket {
        packet_id: PacketId(1),
        session_handle: session,
        transport_policy_id: policy_id,
        transport_policy: policy.clone(),
        compat_version_id: version_id,
        compat_version: version,
        payload_bytes: vec![1, 2, 3],
        received_at_tick: 100,
        decode_status: PacketDecodeStatus::Pending,
    };
    let (policy_id2, policy2) = make_transport_policy();
    let b = BridgePacket {
        packet_id: PacketId(2),
        session_handle: session,
        transport_policy_id: policy_id2,
        transport_policy: policy2,
        compat_version_id: version_id,
        compat_version: version,
        payload_bytes: vec![1, 2, 3],
        received_at_tick: 100,
        decode_status: PacketDecodeStatus::Pending,
    };
    assert_ne!(a, b);
}

// ============================================================================
// packet_legality Function Tests
// ============================================================================

#[test]
fn test_packet_legality_legal_small_payload() {
    let session = make_session_handle();
    let (policy_id, policy) = make_transport_policy();
    let (version_id, version) = make_compat_version();

    let packet = BridgePacket {
        packet_id: PacketId(1),
        session_handle: session,
        transport_policy_id: policy_id,
        transport_policy: policy,
        compat_version_id: version_id,
        compat_version: version,
        payload_bytes: vec![0u8; 100],
        received_at_tick: 0,
        decode_status: PacketDecodeStatus::Pending,
    };
    let verdict = packet_legality(&packet);
    assert!(matches!(verdict, sdk_compat::LegalityVerdict::Legal));
}

#[test]
fn test_packet_legality_illegal_large_payload() {
    let session = make_session_handle();
    let (policy_id, policy) = make_transport_policy();
    let (version_id, version) = make_compat_version();

    let packet = BridgePacket {
        packet_id: PacketId(1),
        session_handle: session,
        transport_policy_id: policy_id,
        transport_policy: policy,
        compat_version_id: version_id,
        compat_version: version,
        payload_bytes: vec![0u8; 17000], // Exceeds MAX_PACKET_BYTES (16384)
        received_at_tick: 0,
        decode_status: PacketDecodeStatus::Pending,
    };
    let verdict = packet_legality(&packet);
    assert!(matches!(verdict, sdk_compat::LegalityVerdict::Illegal));
}

#[test]
fn test_packet_legality_zero_payload() {
    let session = make_session_handle();
    let (policy_id, policy) = make_transport_policy();
    let (version_id, version) = make_compat_version();

    let packet = BridgePacket {
        packet_id: PacketId(1),
        session_handle: session,
        transport_policy_id: policy_id,
        transport_policy: policy,
        compat_version_id: version_id,
        compat_version: version,
        payload_bytes: vec![],
        received_at_tick: 0,
        decode_status: PacketDecodeStatus::Pending,
    };
    let verdict = packet_legality(&packet);
    assert!(matches!(verdict, sdk_compat::LegalityVerdict::Legal));
}

// ============================================================================
// VerticalSlice Types Tests
// ============================================================================

#[test]
fn test_vertical_slice_command_bootstrap() {
    let cmd = VerticalSliceCommand::BootstrapScene;
    assert!(matches!(cmd, VerticalSliceCommand::BootstrapScene));
}

#[test]
fn test_vertical_slice_command_fire_test_shot() {
    let cmd = VerticalSliceCommand::FireTestShot {
        weapon_entity_id: 42,
    };
    assert!(matches!(cmd, VerticalSliceCommand::FireTestShot { .. }));
}

#[test]
fn test_vertical_slice_command_reset_scene() {
    let cmd = VerticalSliceCommand::ResetScene;
    assert!(matches!(cmd, VerticalSliceCommand::ResetScene));
}

#[test]
fn test_vertical_slice_command_select_entity() {
    let cmd = VerticalSliceCommand::SelectEntity { entity_id: 100 };
    assert!(matches!(cmd, VerticalSliceCommand::SelectEntity { .. }));
}

#[test]
fn test_vertical_slice_command_assign_material_stack() {
    let cmd = VerticalSliceCommand::AssignMaterialStack {
        entity_id: 50,
        stack_id: 3,
    };
    assert!(matches!(
        cmd,
        VerticalSliceCommand::AssignMaterialStack { .. }
    ));
}

#[test]
fn test_vertical_slice_command_equality() {
    let a = VerticalSliceCommand::BootstrapScene;
    let b = VerticalSliceCommand::BootstrapScene;
    assert_eq!(a, b);
}

#[test]
fn test_vertical_slice_command_inequality() {
    let a = VerticalSliceCommand::BootstrapScene;
    let b = VerticalSliceCommand::ResetScene;
    assert_ne!(a, b);
}

#[test]
fn test_vertical_slice_ingress_packet_bootstrap() {
    let packet = VerticalSliceIngressPacket::bootstrap_scene(100);
    assert!(matches!(
        packet.command,
        VerticalSliceCommand::BootstrapScene
    ));
    assert_eq!(packet.request_id, 100);
}

#[test]
fn test_vertical_slice_ingress_packet_fire_test_shot() {
    let packet = VerticalSliceIngressPacket::fire_test_shot(200, 99);
    assert!(matches!(
        packet.command,
        VerticalSliceCommand::FireTestShot {
            weapon_entity_id: 99
        }
    ));
    assert_eq!(packet.request_id, 200);
}

#[test]
fn test_vertical_slice_ingress_packet_reset_scene() {
    let packet = VerticalSliceIngressPacket::reset_scene(300);
    assert!(matches!(packet.command, VerticalSliceCommand::ResetScene));
    assert_eq!(packet.request_id, 300);
}

#[test]
fn test_vertical_slice_ingress_packet_equality() {
    let a = VerticalSliceIngressPacket::bootstrap_scene(1);
    let b = VerticalSliceIngressPacket::bootstrap_scene(1);
    assert_eq!(a, b);
}

#[test]
fn test_vertical_slice_ingress_packet_inequality() {
    let a = VerticalSliceIngressPacket::bootstrap_scene(1);
    let b = VerticalSliceIngressPacket::bootstrap_scene(2);
    assert_ne!(a, b);
}

#[test]
fn test_vertical_slice_ingress_packet_serialize_roundtrip() {
    let packet = VerticalSliceIngressPacket::fire_test_shot(42, 7);
    let json = serde_json::to_string(&packet).unwrap();
    let parsed: VerticalSliceIngressPacket = serde_json::from_str(&json).unwrap();
    assert_eq!(packet, parsed);
}

// ============================================================================
// PacketExecutor Tests
// ============================================================================

#[test]
fn test_packet_executor_new() {
    let executor = PacketExecutor::new();
    assert_eq!(executor.last_request_id(), 0);
    assert!(!executor.has_session());
}

#[test]
fn test_packet_executor_default() {
    let executor = PacketExecutor::default();
    assert_eq!(executor.last_request_id(), 0);
    assert!(!executor.has_session());
}

#[test]
fn test_packet_executor_execute_without_session() {
    let mut executor = PacketExecutor::new();
    let packet = VerticalSliceIngressPacket::bootstrap_scene(1);
    let result = executor.execute_packet(packet);
    assert!(result.is_err());
    assert_eq!(executor.last_request_id(), 1);
}

#[test]
fn test_packet_executor_tracks_last_request_id() {
    let mut executor = PacketExecutor::new();
    let packet = VerticalSliceIngressPacket::reset_scene(999);
    let _ = executor.execute_packet(packet);
    assert_eq!(executor.last_request_id(), 999);
}

// ============================================================================
// PacketExecutor Tests
// ============================================================================
