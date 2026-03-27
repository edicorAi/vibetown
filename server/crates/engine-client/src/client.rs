use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tracing::instrument;

use crate::proto::feed;
use crate::proto::feed::feed_service_client::FeedServiceClient;
use crate::proto::mail;
use crate::proto::mail::mail_service_client::MailServiceClient;
use crate::proto::orchestration;
use crate::proto::orchestration::health_service_client::HealthServiceClient;
use crate::proto::orchestration::orchestration_service_client::OrchestrationServiceClient;

/// Errors returned by the engine client.
#[derive(Debug, thiserror::Error)]
pub enum EngineClientError {
    /// gRPC transport error (connection failed, channel broken, etc.).
    #[error("transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    /// The server returned an error status.
    #[error("grpc status: {0}")]
    Status(#[from] tonic::Status),

    /// The provided URI was invalid.
    #[error("invalid uri: {0}")]
    InvalidUri(String),

    /// The server returned an unexpected empty response.
    #[error("unexpected empty response for {0}")]
    EmptyResponse(&'static str),
}

/// High-level client wrapping the generated gRPC service clients for the
/// orchestration engine.
///
/// Each method maps to a single RPC call and performs minimal transformation
/// on the request/response types.
#[derive(Debug, Clone)]
pub struct EngineClient {
    orchestration: OrchestrationServiceClient<Channel>,
    health: HealthServiceClient<Channel>,
    feed: FeedServiceClient<Channel>,
    mail: MailServiceClient<Channel>,
}

impl EngineClient {
    /// Connect to the engine gRPC server at the given address.
    ///
    /// `addr` should be a URI such as `"http://127.0.0.1:50051"`.
    #[instrument(skip_all, fields(addr = %addr))]
    pub async fn connect(addr: &str) -> Result<Self, EngineClientError> {
        let endpoint = Channel::from_shared(addr.to_string())
            .map_err(|e| EngineClientError::InvalidUri(e.to_string()))?;
        let channel = endpoint.connect().await?;

        Ok(Self {
            orchestration: OrchestrationServiceClient::new(channel.clone()),
            health: HealthServiceClient::new(channel.clone()),
            feed: FeedServiceClient::new(channel.clone()),
            mail: MailServiceClient::new(channel),
        })
    }

    /// Create an `EngineClient` from an already-established channel.
    pub fn from_channel(channel: Channel) -> Self {
        Self {
            orchestration: OrchestrationServiceClient::new(channel.clone()),
            health: HealthServiceClient::new(channel.clone()),
            feed: FeedServiceClient::new(channel.clone()),
            mail: MailServiceClient::new(channel),
        }
    }

    /// Create a tonic request with user context metadata attached.
    ///
    /// If `user_id` or `email` are provided, they are injected as
    /// `x-user-id` and `x-user-email` gRPC metadata headers, which the
    /// Go engine extracts via its `UserContextInterceptor`.
    pub fn request_with_user<T>(
        payload: T,
        user_id: Option<&str>,
        email: Option<&str>,
    ) -> tonic::Request<T> {
        let mut request = tonic::Request::new(payload);
        if let Some(id) = user_id {
            if let Ok(val) = id.parse::<MetadataValue<tonic::metadata::Ascii>>() {
                request.metadata_mut().insert("x-user-id", val);
            }
        }
        if let Some(e) = email {
            if let Ok(val) = e.parse::<MetadataValue<tonic::metadata::Ascii>>() {
                request.metadata_mut().insert("x-user-email", val);
            }
        }
        request
    }

    // ─── Health ──────────────────────────────────────────────────────────

    /// Perform a health check against the engine.
    ///
    /// Returns `true` if the service reports `SERVING`.
    #[instrument(skip(self))]
    pub async fn health_check(&mut self) -> Result<bool, EngineClientError> {
        let resp = self
            .health
            .check(orchestration::HealthCheckRequest {
                service: String::new(),
            })
            .await?
            .into_inner();

        Ok(resp.status == orchestration::health_check_response::ServingStatus::Serving as i32)
    }

    // ─── Town ────────────────────────────────────────────────────────────

    /// Create a new town.
    #[instrument(skip(self))]
    pub async fn create_town(
        &mut self,
        name: &str,
        owner: &str,
    ) -> Result<orchestration::Town, EngineClientError> {
        let resp = self
            .orchestration
            .create_town(orchestration::CreateTownRequest {
                name: name.to_string(),
                owner: owner.to_string(),
            })
            .await?
            .into_inner();

        Ok(resp)
    }

    /// Get a town by ID.
    #[instrument(skip(self))]
    pub async fn get_town(
        &mut self,
        id: &str,
    ) -> Result<orchestration::Town, EngineClientError> {
        let resp = self
            .orchestration
            .get_town(orchestration::GetTownRequest {
                id: id.to_string(),
            })
            .await?
            .into_inner();

        Ok(resp)
    }

    // ─── Rig ─────────────────────────────────────────────────────────────

    /// Create a new rig within a town.
    #[instrument(skip(self))]
    pub async fn create_rig(
        &mut self,
        town_id: &str,
        name: &str,
        repo_url: &str,
        beads_prefix: &str,
    ) -> Result<orchestration::Rig, EngineClientError> {
        let resp = self
            .orchestration
            .create_rig(orchestration::CreateRigRequest {
                town_id: town_id.to_string(),
                name: name.to_string(),
                repo_url: repo_url.to_string(),
                beads_prefix: beads_prefix.to_string(),
            })
            .await?
            .into_inner();

        Ok(resp)
    }

    /// List rigs in a town.
    #[instrument(skip(self))]
    pub async fn list_rigs(
        &mut self,
        town_id: &str,
    ) -> Result<Vec<orchestration::Rig>, EngineClientError> {
        let resp = self
            .orchestration
            .list_rigs(orchestration::ListRigsRequest {
                town_id: town_id.to_string(),
            })
            .await?
            .into_inner();

        Ok(resp.rigs)
    }

    // ─── Agent ───────────────────────────────────────────────────────────

    /// Spawn a new agent.
    #[instrument(skip(self))]
    pub async fn spawn_agent(
        &mut self,
        req: orchestration::SpawnAgentRequest,
    ) -> Result<orchestration::Agent, EngineClientError> {
        let resp = self
            .orchestration
            .spawn_agent(req)
            .await?
            .into_inner();

        Ok(resp)
    }

    /// Kill an agent by ID.
    #[instrument(skip(self))]
    pub async fn kill_agent(&mut self, id: &str) -> Result<(), EngineClientError> {
        self.orchestration
            .kill_agent(orchestration::KillAgentRequest {
                id: id.to_string(),
            })
            .await?;

        Ok(())
    }

    /// Get the status of a specific agent.
    #[instrument(skip(self))]
    pub async fn get_agent_status(
        &mut self,
        id: &str,
    ) -> Result<orchestration::Agent, EngineClientError> {
        let resp = self
            .orchestration
            .get_agent_status(orchestration::GetAgentStatusRequest {
                id: id.to_string(),
            })
            .await?
            .into_inner();

        Ok(resp)
    }

    /// List agents, optionally filtered by rig ID, role, or status.
    #[instrument(skip(self))]
    pub async fn list_agents(
        &mut self,
        rig_id: Option<&str>,
        role: Option<&str>,
        status: Option<&str>,
    ) -> Result<Vec<orchestration::Agent>, EngineClientError> {
        let resp = self
            .orchestration
            .list_agents(orchestration::ListAgentsRequest {
                rig_id: rig_id.unwrap_or_default().to_string(),
                role: role.unwrap_or_default().to_string(),
                status: status.unwrap_or_default().to_string(),
            })
            .await?
            .into_inner();

        Ok(resp.agents)
    }

    // ─── Convoy ──────────────────────────────────────────────────────────

    /// Start a new convoy.
    #[instrument(skip(self))]
    pub async fn start_convoy(
        &mut self,
        name: &str,
        formula: &str,
        config_json: &str,
    ) -> Result<orchestration::Convoy, EngineClientError> {
        let resp = self
            .orchestration
            .start_convoy(orchestration::StartConvoyRequest {
                name: name.to_string(),
                formula: formula.to_string(),
                config_json: config_json.to_string(),
            })
            .await?
            .into_inner();

        Ok(resp)
    }

    /// Get the status of a convoy.
    #[instrument(skip(self))]
    pub async fn get_convoy_status(
        &mut self,
        id: &str,
    ) -> Result<orchestration::Convoy, EngineClientError> {
        let resp = self
            .orchestration
            .get_convoy_status(orchestration::GetConvoyStatusRequest {
                id: id.to_string(),
            })
            .await?
            .into_inner();

        Ok(resp)
    }

    /// List convoys, optionally filtered by status.
    #[instrument(skip(self))]
    pub async fn list_convoys(
        &mut self,
        status: Option<&str>,
    ) -> Result<Vec<orchestration::Convoy>, EngineClientError> {
        let resp = self
            .orchestration
            .list_convoys(orchestration::ListConvoysRequest {
                status: status.unwrap_or_default().to_string(),
            })
            .await?
            .into_inner();

        Ok(resp.convoys)
    }

    // ─── Work Dispatch ───────────────────────────────────────────────────

    /// Dispatch a work item to an agent.
    #[instrument(skip(self))]
    pub async fn dispatch_work(
        &mut self,
        agent_id: &str,
        title: &str,
        description: &str,
        rig_id: &str,
    ) -> Result<orchestration::DispatchWorkResponse, EngineClientError> {
        let resp = self
            .orchestration
            .dispatch_work(orchestration::DispatchWorkRequest {
                agent_id: agent_id.to_string(),
                title: title.to_string(),
                description: description.to_string(),
                rig_id: rig_id.to_string(),
            })
            .await?
            .into_inner();

        Ok(resp)
    }

    // ─── Merge Queue ─────────────────────────────────────────────────────

    /// Queue a merge request.
    #[instrument(skip(self))]
    pub async fn queue_merge(
        &mut self,
        req: orchestration::QueueMergeRequest,
    ) -> Result<orchestration::MergeRequest, EngineClientError> {
        let resp = self
            .orchestration
            .queue_merge(req)
            .await?
            .into_inner();

        Ok(resp)
    }

    /// Get the merge queue, optionally filtered by rig ID.
    #[instrument(skip(self))]
    pub async fn get_merge_queue(
        &mut self,
        rig_id: Option<&str>,
    ) -> Result<Vec<orchestration::MergeRequest>, EngineClientError> {
        let resp = self
            .orchestration
            .get_merge_queue(orchestration::GetMergeQueueRequest {
                rig_id: rig_id.unwrap_or_default().to_string(),
            })
            .await?
            .into_inner();

        Ok(resp.requests)
    }

    // ─── Mail ────────────────────────────────────────────────────────────

    /// Send a mail message.
    #[instrument(skip(self))]
    pub async fn send_mail(
        &mut self,
        req: mail::SendMailRequest,
    ) -> Result<mail::MailMessage, EngineClientError> {
        let resp = self
            .mail
            .send_mail(req)
            .await?
            .into_inner();

        Ok(resp)
    }

    /// Get the inbox for a given address.
    #[instrument(skip(self))]
    pub async fn get_inbox(
        &mut self,
        to_addr: &str,
        limit: i32,
    ) -> Result<Vec<mail::MailMessage>, EngineClientError> {
        let resp = self
            .mail
            .get_inbox(mail::GetInboxRequest {
                to_addr: to_addr.to_string(),
                queue: String::new(),
                channel: String::new(),
                unread_only: false,
                limit,
                offset: 0,
            })
            .await?
            .into_inner();

        Ok(resp.messages)
    }

    /// Mark a mail message as read.
    #[instrument(skip(self))]
    pub async fn mark_read(&mut self, id: &str) -> Result<(), EngineClientError> {
        self.mail
            .mark_read(mail::MarkReadRequest {
                id: id.to_string(),
            })
            .await?;

        Ok(())
    }

    /// Claim a message from a queue.
    #[instrument(skip(self))]
    pub async fn claim_queue_message(
        &mut self,
        queue: &str,
        claimed_by: &str,
    ) -> Result<mail::MailMessage, EngineClientError> {
        let resp = self
            .mail
            .claim_queue_message(mail::ClaimRequest {
                queue: queue.to_string(),
                claimed_by: claimed_by.to_string(),
            })
            .await?
            .into_inner();

        Ok(resp)
    }

    // ─── Feed ────────────────────────────────────────────────────────────

    /// Get recent feed events.
    #[instrument(skip(self))]
    pub async fn get_recent_events(
        &mut self,
        limit: i32,
    ) -> Result<Vec<feed::FeedEvent>, EngineClientError> {
        let resp = self
            .feed
            .get_recent_events(feed::GetRecentEventsRequest {
                rig_id: String::new(),
                limit,
                offset: 0,
            })
            .await?
            .into_inner();

        Ok(resp.events)
    }

    /// Stream feed events. Returns a tonic streaming response.
    #[instrument(skip(self))]
    pub async fn stream_events(
        &mut self,
        rig_id: Option<&str>,
        severity: Option<&str>,
    ) -> Result<tonic::Streaming<feed::FeedEvent>, EngineClientError> {
        let resp = self
            .feed
            .stream_events(feed::StreamEventsRequest {
                rig_id: rig_id.unwrap_or_default().to_string(),
                severity: severity.unwrap_or_default().to_string(),
            })
            .await?
            .into_inner();

        Ok(resp)
    }
}
