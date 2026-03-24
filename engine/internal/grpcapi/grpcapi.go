// Package grpcapi contains gRPC service implementations that bridge the Go
// orchestration engine to the Rust API server.
//
// The package supports two operating modes:
//
//   - Standalone (default): Uses InMemoryEngine. No tmux, filesystem, or
//     daemon required. Suitable for development and testing.
//
//   - Daemon: Uses DaemonEngine which wraps the real Gastown packages
//     (polecat, refinery, feed, config, etc.). Requires a running Gas Town
//     workspace with tmux and filesystem state.
//
// The Engine interface is the abstraction boundary. All gRPC handlers
// (OrchestrationHandler, FeedHandler, MailHandler) accept an Engine and
// delegate to it for all operations.
package grpcapi
