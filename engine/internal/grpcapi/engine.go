// Package grpcapi contains gRPC service implementations
// that bridge the Go orchestration engine to the Rust API server.
//
// The Engine interface abstracts the orchestration backend. Two implementations
// are provided:
//   - InMemoryEngine: standalone mode, no tmux/filesystem needed.
//   - DaemonEngine: wraps the real Gastown daemon packages.
package grpcapi

import "time"

// Engine abstracts the orchestration backend so gRPC handlers can work
// in both standalone (in-memory) and daemon-backed (real Gastown) modes.
type Engine interface {
	// Mode returns the engine mode identifier ("standalone" or "daemon").
	Mode() string

	// --- Town ---

	GetTownConfig() (*TownInfo, error)

	// --- Rigs ---

	ListRigs() ([]*RigInfo, error)
	GetRig(name string) (*RigInfo, error)

	// --- Agents ---

	SpawnAgent(name, role, rigName, runtime, configJSON string) (*AgentInfo, error)
	KillAgent(id string) error
	ListAgents(rigFilter, roleFilter, statusFilter string) ([]*AgentInfo, error)
	GetAgent(id string) (*AgentInfo, error)

	// --- Convoys ---

	StartConvoy(name, formula, configJSON string) (*ConvoyInfo, error)
	ListConvoys(statusFilter string) ([]*ConvoyInfo, error)
	GetConvoy(id string) (*ConvoyInfo, error)

	// --- Mail ---

	SendMail(msg *MailInfo) (*MailInfo, error)
	GetInbox(addr string, limit, offset int, unreadOnly bool) ([]*MailInfo, int, error)
	MarkRead(id string) error

	// --- Feed ---

	GetRecentEvents(limit int) ([]*EventInfo, error)
	SubscribeEvents() (<-chan *EventInfo, func())

	// --- Merge Queue ---

	QueueMerge(rigID, branch, targetBranch, workItemID, agentID string) (*MergeInfo, error)
	GetMergeQueue(rigID string) ([]*MergeInfo, error)
}

// TownInfo holds town-level metadata.
type TownInfo struct {
	ID        string
	Name      string
	Owner     string
	CreatedAt time.Time
}

// RigInfo represents a configured rig/workspace.
type RigInfo struct {
	Name        string
	GitURL      string
	PushURL     string
	Prefix      string
	AgentCount  int
	WorktreeDir string
}

// AgentInfo represents an agent (polecat, witness, refinery, crew, etc.).
type AgentInfo struct {
	ID             string
	Name           string
	Role           string
	RigID          string
	Status         string
	Runtime        string
	ConfigJSON     string
	LastActivityAt time.Time
	CreatedAt      time.Time
}

// ConvoyInfo represents a convoy (coordinated multi-issue dispatch).
type ConvoyInfo struct {
	ID         string
	Name       string
	Status     string
	Formula    string
	ConfigJSON string
	CreatedAt  time.Time
	UpdatedAt  time.Time
}

// MailInfo represents a mail message.
type MailInfo struct {
	ID          string
	FromAddr    string
	ToAddr      string
	Subject     string
	Body        string
	Priority    string
	MessageType string
	Delivery    string
	ThreadID    string
	ReplyTo     string
	Channel     string
	Queue       string
	Read        bool
	ClaimedBy   string
	ClaimedAt   time.Time
	CreatedAt   time.Time
}

// EventInfo represents a feed event.
type EventInfo struct {
	ID        string
	EventType string
	Source    string
	RigID    string
	AgentID  string
	Summary  string
	Severity string
	Payload  map[string]string
	CreatedAt time.Time
}

// MergeInfo represents a merge queue entry.
type MergeInfo struct {
	ID           string
	WorkItemID   string
	RigID        string
	Branch       string
	TargetBranch string
	Status       string
	AgentID      string
	QueuedAt     time.Time
}
