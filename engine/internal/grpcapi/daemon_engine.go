package grpcapi

import (
	"fmt"
	"log"
	"path/filepath"

	"github.com/edicorai/vibetown/engine/internal/config"
	"github.com/edicorai/vibetown/engine/internal/feed"
	"github.com/edicorai/vibetown/engine/internal/git"
	"github.com/edicorai/vibetown/engine/internal/polecat"
	"github.com/edicorai/vibetown/engine/internal/refinery"
	"github.com/edicorai/vibetown/engine/internal/rig"
	"github.com/edicorai/vibetown/engine/internal/tmux"
)

// DaemonEngine wraps the real Gastown packages to provide a daemon-backed
// Engine. For methods that are too tightly coupled to filesystem/tmux state,
// it falls back to an embedded InMemoryEngine and logs a warning.
type DaemonEngine struct {
	townRoot string
	fallback *InMemoryEngine

	// Gastown components (initialized lazily or at construction)
	curator *feed.Curator
}

// NewDaemonEngine creates a DaemonEngine for the given town root directory.
// It validates that the town root exists and loads basic configuration.
// Components that require a running daemon (tmux sessions, etc.) are
// initialized lazily on first use.
func NewDaemonEngine(townRoot string) (*DaemonEngine, error) {
	// Validate town root by loading the town config.
	townConfigPath := filepath.Join(townRoot, "mayor", "town.json")
	if _, err := config.LoadTownConfig(townConfigPath); err != nil {
		return nil, fmt.Errorf("daemon engine: loading town config: %w", err)
	}

	curator := feed.NewCurator(townRoot)

	return &DaemonEngine{
		townRoot: townRoot,
		fallback: NewInMemoryEngine(),
		curator:  curator,
	}, nil
}

func (d *DaemonEngine) Mode() string { return "daemon" }

// --- Town ---

func (d *DaemonEngine) GetTownConfig() (*TownInfo, error) {
	townConfigPath := filepath.Join(d.townRoot, "mayor", "town.json")
	tc, err := config.LoadTownConfig(townConfigPath)
	if err != nil {
		return nil, fmt.Errorf("loading town config: %w", err)
	}
	return &TownInfo{
		ID:        tc.Name, // Town name serves as the identifier.
		Name:      tc.Name,
		Owner:     tc.Owner,
		CreatedAt: tc.CreatedAt,
	}, nil
}

// --- Rigs ---

func (d *DaemonEngine) ListRigs() ([]*RigInfo, error) {
	rigsPath := filepath.Join(d.townRoot, "mayor", "rigs.json")
	rc, err := config.LoadRigsConfig(rigsPath)
	if err != nil {
		log.Printf("daemon engine: ListRigs: could not load rigs.json: %v (falling back to in-memory)", err)
		return d.fallback.ListRigs()
	}

	var result []*RigInfo
	for name, entry := range rc.Rigs {
		prefix := ""
		if entry.BeadsConfig != nil {
			prefix = entry.BeadsConfig.Prefix
		}
		result = append(result, &RigInfo{
			Name:        name,
			GitURL:      entry.GitURL,
			PushURL:     entry.PushURL,
			Prefix:      prefix,
			WorktreeDir: filepath.Join(d.townRoot, name),
		})
	}
	return result, nil
}

func (d *DaemonEngine) GetRig(name string) (*RigInfo, error) {
	rigs, err := d.ListRigs()
	if err != nil {
		return nil, err
	}
	for _, r := range rigs {
		if r.Name == name {
			return r, nil
		}
	}
	return nil, fmt.Errorf("rig %q not found", name)
}

// --- Agents ---

// loadRig creates a rig.Rig value for the given rig name.
func (d *DaemonEngine) loadRig(rigName string) *rig.Rig {
	rigPath := filepath.Join(d.townRoot, rigName)
	return &rig.Rig{
		Name: rigName,
		Path: rigPath,
	}
}

func (d *DaemonEngine) SpawnAgent(name, role, rigName, runtime, configJSON string) (*AgentInfo, error) {
	// Spawning agents requires tmux sessions and complex initialization.
	// For Phase 1, delegate to fallback and log.
	log.Printf("daemon engine: SpawnAgent: delegating to in-memory (real spawn requires tmux lifecycle)")
	return d.fallback.SpawnAgent(name, role, rigName, runtime, configJSON)
}

func (d *DaemonEngine) KillAgent(id string) error {
	// Killing agents requires tmux session teardown.
	// For Phase 1, delegate to fallback.
	log.Printf("daemon engine: KillAgent: delegating to in-memory (real kill requires tmux lifecycle)")
	return d.fallback.KillAgent(id)
}

func (d *DaemonEngine) ListAgents(rigFilter, roleFilter, statusFilter string) ([]*AgentInfo, error) {
	// Try to list real polecats from the filesystem.
	if rigFilter != "" {
		agents, err := d.listRigAgents(rigFilter)
		if err == nil {
			// Apply additional filters.
			var filtered []*AgentInfo
			for _, a := range agents {
				if roleFilter != "" && a.Role != roleFilter {
					continue
				}
				if statusFilter != "" && a.Status != statusFilter {
					continue
				}
				filtered = append(filtered, a)
			}
			return filtered, nil
		}
		log.Printf("daemon engine: ListAgents: real listing failed for rig %s: %v (falling back)", rigFilter, err)
	}

	// If no rig filter, try all rigs.
	if rigFilter == "" {
		rigs, err := d.ListRigs()
		if err == nil {
			var allAgents []*AgentInfo
			for _, r := range rigs {
				agents, err := d.listRigAgents(r.Name)
				if err != nil {
					continue
				}
				for _, a := range agents {
					if roleFilter != "" && a.Role != roleFilter {
						continue
					}
					if statusFilter != "" && a.Status != statusFilter {
						continue
					}
					allAgents = append(allAgents, a)
				}
			}
			if len(allAgents) > 0 || len(rigs) > 0 {
				return allAgents, nil
			}
		}
	}

	return d.fallback.ListAgents(rigFilter, roleFilter, statusFilter)
}

// listRigAgents reads polecats from the filesystem for a specific rig.
func (d *DaemonEngine) listRigAgents(rigName string) ([]*AgentInfo, error) {
	r := d.loadRig(rigName)
	g := git.NewGit(filepath.Join(r.Path, "mayor", "rig"))
	t := tmux.NewTmux()
	mgr := polecat.NewManager(r, g, t)

	polecats, err := mgr.List()
	if err != nil {
		return nil, fmt.Errorf("listing polecats for rig %s: %w", rigName, err)
	}

	var agents []*AgentInfo
	for _, p := range polecats {
		agents = append(agents, &AgentInfo{
			ID:        fmt.Sprintf("%s/%s", rigName, p.Name),
			Name:      p.Name,
			Role:      "polecat",
			RigID:     rigName,
			Status:    string(p.State),
			Runtime:   "claude", // Default; real runtime not stored on polecat
			CreatedAt: p.CreatedAt,
		})
	}
	return agents, nil
}

func (d *DaemonEngine) GetAgent(id string) (*AgentInfo, error) {
	// Try in-memory first (for spawned agents).
	agent, err := d.fallback.GetAgent(id)
	if err == nil {
		return agent, nil
	}
	return nil, fmt.Errorf("agent %q not found", id)
}

// --- Convoys ---

func (d *DaemonEngine) StartConvoy(name, formula, configJSON string) (*ConvoyInfo, error) {
	// Convoy creation requires beads and gt subprocess calls.
	// For Phase 1, delegate to fallback.
	log.Printf("daemon engine: StartConvoy: delegating to in-memory (real convoy requires beads/gt)")
	return d.fallback.StartConvoy(name, formula, configJSON)
}

func (d *DaemonEngine) ListConvoys(statusFilter string) ([]*ConvoyInfo, error) {
	// Convoys are tracked in beads, which requires Dolt/bd access.
	// For Phase 1, delegate to fallback.
	log.Printf("daemon engine: ListConvoys: delegating to in-memory (real convoy listing requires beads)")
	return d.fallback.ListConvoys(statusFilter)
}

func (d *DaemonEngine) GetConvoy(id string) (*ConvoyInfo, error) {
	log.Printf("daemon engine: GetConvoy: delegating to in-memory")
	return d.fallback.GetConvoy(id)
}

// --- Mail ---

func (d *DaemonEngine) SendMail(msg *MailInfo) (*MailInfo, error) {
	// Real mail uses beads and bd commands.
	// For Phase 1, delegate to fallback.
	log.Printf("daemon engine: SendMail: delegating to in-memory (real mail requires beads)")
	return d.fallback.SendMail(msg)
}

func (d *DaemonEngine) GetInbox(addr string, limit, offset int, unreadOnly bool) ([]*MailInfo, int, error) {
	log.Printf("daemon engine: GetInbox: delegating to in-memory (real inbox requires beads)")
	return d.fallback.GetInbox(addr, limit, offset, unreadOnly)
}

func (d *DaemonEngine) MarkRead(id string) error {
	log.Printf("daemon engine: MarkRead: delegating to in-memory")
	return d.fallback.MarkRead(id)
}

// --- Feed ---

func (d *DaemonEngine) GetRecentEvents(limit int) ([]*EventInfo, error) {
	// Try reading from the real feed file.
	feedEvents, err := d.readFeedFile(limit)
	if err == nil && len(feedEvents) > 0 {
		return feedEvents, nil
	}
	if err != nil {
		log.Printf("daemon engine: GetRecentEvents: reading feed file failed: %v (falling back)", err)
	}
	return d.fallback.GetRecentEvents(limit)
}

// readFeedFile reads recent events from the .feed.jsonl file.
func (d *DaemonEngine) readFeedFile(limit int) ([]*EventInfo, error) {
	feedPath := filepath.Join(d.townRoot, feed.FeedFile)

	// Use the feed package's exported FeedEvent type to read the file.
	// We parse JSONL manually since the Curator's read methods are private.
	events, err := readFeedJSONL(feedPath, limit)
	if err != nil {
		return nil, err
	}
	return events, nil
}

func (d *DaemonEngine) SubscribeEvents() (<-chan *EventInfo, func()) {
	// Real event subscription would tail .events.jsonl. For Phase 1,
	// use the in-memory subscription.
	log.Printf("daemon engine: SubscribeEvents: using in-memory subscription (real requires file tailing)")
	return d.fallback.SubscribeEvents()
}

// PublishEvent inserts an event into both the fallback store and fans out
// to subscribers. DaemonEngine does not write to .feed.jsonl (the Curator
// does that from .events.jsonl).
func (d *DaemonEngine) PublishEvent(evt *EventInfo) {
	d.fallback.PublishEvent(evt)
}

// --- Merge Queue ---

func (d *DaemonEngine) QueueMerge(rigID, branch, targetBranch, workItemID, agentID string) (*MergeInfo, error) {
	// Try real refinery queue.
	mr, err := d.queueMergeReal(rigID, branch, targetBranch, workItemID, agentID)
	if err == nil {
		return mr, nil
	}
	log.Printf("daemon engine: QueueMerge: real queue failed: %v (falling back)", err)
	return d.fallback.QueueMerge(rigID, branch, targetBranch, workItemID, agentID)
}

func (d *DaemonEngine) queueMergeReal(rigID, _, _, _, _ string) (*MergeInfo, error) {
	// Creating real MRs requires beads bead creation which needs bd.
	// For Phase 1, just read the queue.
	return nil, fmt.Errorf("real merge creation not yet wired (requires beads)")
}

func (d *DaemonEngine) GetMergeQueue(rigID string) ([]*MergeInfo, error) {
	if rigID == "" {
		return nil, nil
	}

	r := d.loadRig(rigID)
	mgr := refinery.NewManager(r)

	queue, err := mgr.Queue()
	if err != nil {
		log.Printf("daemon engine: GetMergeQueue: reading real queue for %s failed: %v (falling back)", rigID, err)
		return d.fallback.GetMergeQueue(rigID)
	}

	var result []*MergeInfo
	for _, item := range queue {
		if item.MR == nil {
			continue
		}
		result = append(result, &MergeInfo{
			ID:           item.MR.ID,
			WorkItemID:   item.MR.IssueID,
			RigID:        rigID,
			Branch:       item.MR.Branch,
			TargetBranch: item.MR.TargetBranch,
			Status:       string(item.MR.Status),
			AgentID:      item.MR.Worker,
			QueuedAt:     item.MR.CreatedAt,
		})
	}
	return result, nil
}

// StartCurator starts the feed curator goroutine.
// Call this after creating the DaemonEngine to enable feed curation.
func (d *DaemonEngine) StartCurator() error {
	return d.curator.Start()
}

// StopCurator stops the feed curator.
func (d *DaemonEngine) StopCurator() {
	d.curator.Stop()
}
