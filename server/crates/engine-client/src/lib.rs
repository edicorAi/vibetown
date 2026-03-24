//! gRPC client for communicating with the Go orchestration engine.

pub mod proto {
    pub mod orchestration {
        tonic::include_proto!("vibetown.orchestration.v1");
    }
    pub mod feed {
        tonic::include_proto!("vibetown.feed.v1");
    }
    pub mod mail {
        tonic::include_proto!("vibetown.mail.v1");
    }
}

mod client;
pub use client::{EngineClient, EngineClientError};
