package grpcapi

import (
	"fmt"
	"sort"
	"sync"
	"time"

	"github.com/google/uuid"
)

// InMemoryEngine is a standalone Engine implementation that stores everything
// in memory. It requires no tmux, filesystem, or daemon. This is the default
// mode and is also used for testing.
type InMemoryEngine struct {
	mu       sync.RWMutex
	agents   map[string]*AgentInfo
	convoys  map[string]*ConvoyInfo
	messages map[string]*MailInfo
	events   []*EventInfo // append-only ring; for simplicity we keep all
	rigs     []*RigInfo

	town *TownInfo

	// Feed subscribers
	subMu     sync.Mutex
	subs      map[uint64]chan *EventInfo
	nextSubID uint64
}

// NewInMemoryEngine returns an InMemoryEngine ready for use.
func NewInMemoryEngine() *InMemoryEngine {
	return &InMemoryEngine{
		agents:   make(map[string]*AgentInfo),
		convoys:  make(map[string]*ConvoyInfo),
		messages: make(map[string]*MailInfo),
		subs:     make(map[uint64]chan *EventInfo),
		town: &TownInfo{
			ID:        uuid.NewString(),
			Name:      "default-town",
			Owner:     "system",
			CreatedAt: time.Now(),
		},
		rigs: []*RigInfo{
			{
				Name:        "demo-rig",
				GitURL:      "https://github.com/example/demo",
				Prefix:      "dm",
				AgentCount:  0,
				WorktreeDir: "/tmp/vibetown/demo-rig",
			},
		},
	}
}

func (e *InMemoryEngine) Mode() string { return "standalone" }

// --- Town ---

func (e *InMemoryEngine) GetTownConfig() (*TownInfo, error) {
	return e.town, nil
}

// --- Rigs ---

func (e *InMemoryEngine) ListRigs() ([]*RigInfo, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()
	out := make([]*RigInfo, len(e.rigs))
	copy(out, e.rigs)
	return out, nil
}

func (e *InMemoryEngine) GetRig(name string) (*RigInfo, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()
	for _, r := range e.rigs {
		if r.Name == name {
			return r, nil
		}
	}
	return nil, fmt.Errorf("rig %q not found", name)
}

// --- Agents ---

func (e *InMemoryEngine) SpawnAgent(name, role, rigName, runtime, configJSON string) (*AgentInfo, error) {
	now := time.Now()
	agent := &AgentInfo{
		ID:             uuid.NewString(),
		Name:           name,
		Role:           role,
		RigID:          rigName,
		Status:         "idle",
		Runtime:        runtime,
		ConfigJSON:     configJSON,
		LastActivityAt: now,
		CreatedAt:      now,
	}

	e.mu.Lock()
	e.agents[agent.ID] = agent
	e.mu.Unlock()

	return agent, nil
}

func (e *InMemoryEngine) KillAgent(id string) error {
	e.mu.Lock()
	defer e.mu.Unlock()
	if _, ok := e.agents[id]; !ok {
		return fmt.Errorf("agent %q not found", id)
	}
	delete(e.agents, id)
	return nil
}

func (e *InMemoryEngine) ListAgents(rigFilter, roleFilter, statusFilter string) ([]*AgentInfo, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	var result []*AgentInfo
	for _, a := range e.agents {
		if rigFilter != "" && a.RigID != rigFilter {
			continue
		}
		if roleFilter != "" && a.Role != roleFilter {
			continue
		}
		if statusFilter != "" && a.Status != statusFilter {
			continue
		}
		result = append(result, a)
	}
	return result, nil
}

func (e *InMemoryEngine) GetAgent(id string) (*AgentInfo, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()
	agent, ok := e.agents[id]
	if !ok {
		return nil, fmt.Errorf("agent %q not found", id)
	}
	return agent, nil
}

// --- Convoys ---

func (e *InMemoryEngine) StartConvoy(name, formula, configJSON string) (*ConvoyInfo, error) {
	now := time.Now()
	convoy := &ConvoyInfo{
		ID:         uuid.NewString(),
		Name:       name,
		Status:     "active",
		Formula:    formula,
		ConfigJSON: configJSON,
		CreatedAt:  now,
		UpdatedAt:  now,
	}
	e.mu.Lock()
	e.convoys[convoy.ID] = convoy
	e.mu.Unlock()
	return convoy, nil
}

func (e *InMemoryEngine) ListConvoys(statusFilter string) ([]*ConvoyInfo, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	var result []*ConvoyInfo
	for _, c := range e.convoys {
		if statusFilter != "" && c.Status != statusFilter {
			continue
		}
		result = append(result, c)
	}
	return result, nil
}

func (e *InMemoryEngine) GetConvoy(id string) (*ConvoyInfo, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()
	convoy, ok := e.convoys[id]
	if !ok {
		return nil, fmt.Errorf("convoy %q not found", id)
	}
	return convoy, nil
}

// --- Mail ---

func (e *InMemoryEngine) SendMail(msg *MailInfo) (*MailInfo, error) {
	msg.ID = uuid.NewString()
	msg.CreatedAt = time.Now()

	e.mu.Lock()
	e.messages[msg.ID] = msg
	e.mu.Unlock()

	return msg, nil
}

func (e *InMemoryEngine) GetInbox(addr string, limit, offset int, unreadOnly bool) ([]*MailInfo, int, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	var matching []*MailInfo
	for _, msg := range e.messages {
		if addr != "" && msg.ToAddr != addr {
			continue
		}
		if unreadOnly && msg.Read {
			continue
		}
		matching = append(matching, msg)
	}

	// Sort by creation time for deterministic ordering.
	sort.Slice(matching, func(i, j int) bool {
		return matching[i].CreatedAt.Before(matching[j].CreatedAt)
	})

	total := len(matching)

	if limit <= 0 {
		limit = 50
	}
	if offset > len(matching) {
		offset = len(matching)
	}
	end := offset + limit
	if end > len(matching) {
		end = len(matching)
	}

	return matching[offset:end], total, nil
}

func (e *InMemoryEngine) MarkRead(id string) error {
	e.mu.Lock()
	defer e.mu.Unlock()
	msg, ok := e.messages[id]
	if !ok {
		return fmt.Errorf("message %q not found", id)
	}
	msg.Read = true
	return nil
}

// --- Feed ---

func (e *InMemoryEngine) GetRecentEvents(limit int) ([]*EventInfo, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()

	if limit <= 0 {
		limit = 50
	}

	total := len(e.events)
	start := total - limit
	if start < 0 {
		start = 0
	}

	result := make([]*EventInfo, total-start)
	copy(result, e.events[start:])
	return result, nil
}

func (e *InMemoryEngine) SubscribeEvents() (<-chan *EventInfo, func()) {
	ch := make(chan *EventInfo, 64)

	e.subMu.Lock()
	id := e.nextSubID
	e.nextSubID++
	e.subs[id] = ch
	e.subMu.Unlock()

	cancel := func() {
		e.subMu.Lock()
		delete(e.subs, id)
		e.subMu.Unlock()
	}

	return ch, cancel
}

// PublishEvent adds an event and fans it out to subscribers. This is used
// by the FeedHandler's Publish method to insert events into the engine.
func (e *InMemoryEngine) PublishEvent(evt *EventInfo) {
	if evt.ID == "" {
		evt.ID = uuid.NewString()
	}
	if evt.CreatedAt.IsZero() {
		evt.CreatedAt = time.Now()
	}

	e.mu.Lock()
	e.events = append(e.events, evt)
	e.mu.Unlock()

	e.subMu.Lock()
	for _, ch := range e.subs {
		select {
		case ch <- evt:
		default:
		}
	}
	e.subMu.Unlock()
}

// --- Merge Queue ---

func (e *InMemoryEngine) QueueMerge(rigID, branch, targetBranch, workItemID, agentID string) (*MergeInfo, error) {
	mr := &MergeInfo{
		ID:           uuid.NewString(),
		WorkItemID:   workItemID,
		RigID:        rigID,
		Branch:       branch,
		TargetBranch: targetBranch,
		Status:       "pending",
		AgentID:      agentID,
		QueuedAt:     time.Now(),
	}

	e.mu.Lock()
	e.events = append(e.events, &EventInfo{
		ID:        uuid.NewString(),
		EventType: "merge_queued",
		Source:    "engine",
		RigID:    rigID,
		Summary:  fmt.Sprintf("Merge queued: %s -> %s", branch, targetBranch),
		Severity: "info",
		CreatedAt: time.Now(),
	})
	e.mu.Unlock()

	return mr, nil
}

func (e *InMemoryEngine) GetMergeQueue(_ string) ([]*MergeInfo, error) {
	// In-memory engine doesn't persist merge queue entries beyond creation.
	return nil, nil
}
