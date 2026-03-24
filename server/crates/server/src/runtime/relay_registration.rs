//! Relay host connection — currently disabled (relay crates removed).

use crate::DeploymentImpl;

/// No-op: relay functionality has been removed.
pub async fn spawn_relay(_deployment: &DeploymentImpl) {}

/// No-op: relay functionality has been removed.
pub async fn stop_relay(_deployment: &DeploymentImpl) {}
