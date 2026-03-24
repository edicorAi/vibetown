//! Tests for the engine-client crate.
//!
//! These tests verify that the generated proto types compile, can be constructed,
//! and that the client wrapper types are usable. They do NOT require a running
//! gRPC server.

use engine_client::proto::{feed, mail, orchestration};
use engine_client::{EngineClient, EngineClientError};

// ─── Proto type construction ────────────────────────────────────────────────

#[test]
fn town_can_be_constructed() {
    let town = orchestration::Town {
        id: "town-1".into(),
        name: "Vibetown".into(),
        owner: "owner-1".into(),
        config_json: "{}".into(),
        created_at: None,
        updated_at: None,
    };
    assert_eq!(town.id, "town-1");
    assert_eq!(town.name, "Vibetown");
    assert_eq!(town.owner, "owner-1");
}

#[test]
fn rig_can_be_constructed() {
    let rig = orchestration::Rig {
        id: "rig-1".into(),
        town_id: "town-1".into(),
        name: "test-rig".into(),
        repo_url: "https://github.com/test/repo".into(),
        beads_prefix: "beads/".into(),
        config_json: "{}".into(),
        created_at: None,
        updated_at: None,
    };
    assert_eq!(rig.id, "rig-1");
    assert_eq!(rig.town_id, "town-1");
    assert_eq!(rig.repo_url, "https://github.com/test/repo");
}

#[test]
fn agent_can_be_constructed() {
    let agent = orchestration::Agent {
        id: "agent-1".into(),
        name: "test-agent".into(),
        role: "crew".into(),
        rig_id: "rig-1".into(),
        status: "idle".into(),
        runtime: "claude".into(),
        config_json: "{}".into(),
        last_activity_at: None,
        created_at: None,
    };
    assert_eq!(agent.id, "agent-1");
    assert_eq!(agent.role, "crew");
    assert_eq!(agent.runtime, "claude");
}

#[test]
fn spawn_agent_request_can_be_constructed() {
    let req = orchestration::SpawnAgentRequest {
        name: "worker-1".into(),
        role: "crew".into(),
        rig_id: "rig-1".into(),
        runtime: "claude".into(),
        config_json: "{}".into(),
    };
    assert_eq!(req.name, "worker-1");
    assert_eq!(req.role, "crew");
}

#[test]
fn convoy_can_be_constructed() {
    let convoy = orchestration::Convoy {
        id: "convoy-1".into(),
        name: "deploy-v2".into(),
        status: "active".into(),
        formula: "standard".into(),
        config_json: "{}".into(),
        created_at: None,
        updated_at: None,
    };
    assert_eq!(convoy.id, "convoy-1");
    assert_eq!(convoy.status, "active");
}

#[test]
fn dispatch_work_request_can_be_constructed() {
    let req = orchestration::DispatchWorkRequest {
        agent_id: "agent-1".into(),
        title: "Fix bug #42".into(),
        description: "The widget is broken".into(),
        rig_id: "rig-1".into(),
    };
    assert_eq!(req.agent_id, "agent-1");
    assert_eq!(req.title, "Fix bug #42");
}

#[test]
fn merge_request_can_be_constructed() {
    let mr = orchestration::MergeRequest {
        id: "mr-1".into(),
        work_item_id: "wi-1".into(),
        rig_id: "rig-1".into(),
        branch: "feature/fix".into(),
        target_branch: "main".into(),
        status: "pending".into(),
        agent_id: "agent-1".into(),
        pr_url: "https://github.com/test/repo/pull/1".into(),
        queued_at: None,
        merged_at: None,
    };
    assert_eq!(mr.id, "mr-1");
    assert_eq!(mr.status, "pending");
}

// ─── Feed types ─────────────────────────────────────────────────────────────

#[test]
fn feed_event_can_be_constructed() {
    let event = feed::FeedEvent {
        id: "evt-1".into(),
        event_type: "agent_spawned".into(),
        source: "system".into(),
        rig_id: "rig-1".into(),
        agent_id: "agent-1".into(),
        work_item_id: String::new(),
        summary: "Agent spawned".into(),
        details_json: "{}".into(),
        severity: "info".into(),
        created_at: None,
    };
    assert_eq!(event.id, "evt-1");
    assert_eq!(event.event_type, "agent_spawned");
    assert_eq!(event.severity, "info");
}

#[test]
fn get_recent_events_request_can_be_constructed() {
    let req = feed::GetRecentEventsRequest {
        rig_id: "rig-1".into(),
        limit: 50,
        offset: 0,
    };
    assert_eq!(req.limit, 50);
}

// ─── Mail types ─────────────────────────────────────────────────────────────

#[test]
fn mail_message_can_be_constructed() {
    let msg = mail::MailMessage {
        id: "mail-1".into(),
        from_addr: "mayor@town".into(),
        to_addr: "crew-1@town".into(),
        subject: "New task".into(),
        body: "Please implement feature X".into(),
        priority: "normal".into(),
        message_type: "task".into(),
        delivery: "queue".into(),
        thread_id: "thread-1".into(),
        reply_to: String::new(),
        queue: "default".into(),
        channel: String::new(),
        claimed_by: String::new(),
        pinned: false,
        read: false,
        claimed_at: None,
        created_at: None,
    };
    assert_eq!(msg.id, "mail-1");
    assert_eq!(msg.from_addr, "mayor@town");
    assert_eq!(msg.priority, "normal");
}

#[test]
fn send_mail_request_can_be_constructed() {
    let req = mail::SendMailRequest {
        from_addr: "agent-1@town".into(),
        to_addr: "mayor@town".into(),
        subject: "Status report".into(),
        body: "All clear".into(),
        priority: "low".into(),
        message_type: "notification".into(),
        delivery: "queue".into(),
        thread_id: String::new(),
        reply_to: String::new(),
        channel: String::new(),
    };
    assert_eq!(req.from_addr, "agent-1@town");
    assert_eq!(req.message_type, "notification");
}

#[test]
fn get_inbox_request_can_be_constructed() {
    let req = mail::GetInboxRequest {
        to_addr: "agent-1@town".into(),
        queue: "default".into(),
        channel: String::new(),
        unread_only: true,
        limit: 20,
        offset: 0,
    };
    assert_eq!(req.to_addr, "agent-1@town");
    assert!(req.unread_only);
}

// ─── Enum values ────────────────────────────────────────────────────────────

#[test]
fn serving_status_enum_values() {
    use orchestration::health_check_response::ServingStatus;
    assert_eq!(ServingStatus::Unspecified as i32, 0);
    assert_eq!(ServingStatus::Serving as i32, 1);
    assert_eq!(ServingStatus::NotServing as i32, 2);
}

// ─── Client construction ────────────────────────────────────────────────────

#[test]
fn client_from_channel_compiles() {
    // This test verifies that from_channel accepts a Channel and the types
    // line up. We cannot actually create a Channel without a real endpoint,
    // so we just verify the function signature exists and compiles.
    let _: fn(tonic::transport::Channel) -> EngineClient = EngineClient::from_channel;
}

#[tokio::test]
async fn connect_to_invalid_address_returns_error() {
    // Connecting to a non-existent server should return a transport error.
    let result = EngineClient::connect("http://127.0.0.1:1").await;
    assert!(result.is_err());
}

#[test]
fn connect_with_invalid_uri_returns_error() {
    // An invalid URI (no scheme) should be caught before even trying to connect.
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let result = rt.block_on(EngineClient::connect("not a valid uri"));
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, EngineClientError::InvalidUri(_)),
        "expected InvalidUri, got: {err:?}"
    );
}

// ─── prost encode/decode roundtrip ──────────────────────────────────────────

#[test]
fn town_prost_roundtrip() {
    use prost::Message;

    let town = orchestration::Town {
        id: "town-42".into(),
        name: "Roundtrip City".into(),
        owner: "test-owner".into(),
        config_json: r#"{"k":"v"}"#.into(),
        created_at: None,
        updated_at: None,
    };

    let encoded = town.encode_to_vec();
    let decoded = orchestration::Town::decode(encoded.as_slice()).unwrap();

    assert_eq!(town, decoded);
}

#[test]
fn feed_event_prost_roundtrip() {
    use prost::Message;

    let event = feed::FeedEvent {
        id: "evt-99".into(),
        event_type: "convoy_completed".into(),
        source: "deacon".into(),
        rig_id: "rig-5".into(),
        agent_id: "agent-7".into(),
        work_item_id: "wi-3".into(),
        summary: "Convoy finished successfully".into(),
        details_json: "{}".into(),
        severity: "info".into(),
        created_at: None,
    };

    let encoded = event.encode_to_vec();
    let decoded = feed::FeedEvent::decode(encoded.as_slice()).unwrap();

    assert_eq!(event, decoded);
}

#[test]
fn mail_message_prost_roundtrip() {
    use prost::Message;

    let msg = mail::MailMessage {
        id: "mail-99".into(),
        from_addr: "a@b".into(),
        to_addr: "c@d".into(),
        subject: "Test".into(),
        body: "Body".into(),
        priority: "high".into(),
        message_type: "escalation".into(),
        delivery: "interrupt".into(),
        thread_id: "t-1".into(),
        reply_to: "mail-98".into(),
        queue: "urgent".into(),
        channel: "ops".into(),
        claimed_by: String::new(),
        pinned: true,
        read: false,
        claimed_at: None,
        created_at: None,
    };

    let encoded = msg.encode_to_vec();
    let decoded = mail::MailMessage::decode(encoded.as_slice()).unwrap();

    assert_eq!(msg, decoded);
}

// ─── Timestamp handling ─────────────────────────────────────────────────────

#[test]
fn town_with_timestamp() {
    use prost::Message;

    let town = orchestration::Town {
        id: "town-ts".into(),
        name: "Timestamp Town".into(),
        owner: "owner".into(),
        config_json: "{}".into(),
        created_at: Some(prost_types::Timestamp {
            seconds: 1700000000,
            nanos: 500_000_000,
        }),
        updated_at: None,
    };

    let encoded = town.encode_to_vec();
    let decoded = orchestration::Town::decode(encoded.as_slice()).unwrap();

    assert_eq!(decoded.created_at.unwrap().seconds, 1700000000);
    assert_eq!(decoded.created_at.unwrap().nanos, 500_000_000);
}
