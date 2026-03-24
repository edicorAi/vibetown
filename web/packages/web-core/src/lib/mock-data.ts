/**
 * Mock data for demo mode. Used when the backend is unavailable.
 */
import type {
  Town,
  Rig,
  Agent,
  FeedEvent,
  MailMessage,
  MergeRequest,
} from "./api.ts";

function minutesAgo(n: number): string {
  return new Date(Date.now() - n * 60_000).toISOString();
}

function hoursAgo(n: number): string {
  return new Date(Date.now() - n * 3_600_000).toISOString();
}

// ---------------------------------------------------------------------------
// Town
// ---------------------------------------------------------------------------
export const mockTown: Town = {
  id: "town-1",
  name: "Vibetown HQ",
  owner: "admin",
  config_json: "{}",
};

// ---------------------------------------------------------------------------
// Rigs (3 projects)
// ---------------------------------------------------------------------------
export const mockRigs: Rig[] = [
  {
    id: "rig-1",
    town_id: "town-1",
    name: "vibetown",
    repo_url: "github.com/edicorai/vibetown",
    beads_prefix: "vt-",
  },
  {
    id: "rig-2",
    town_id: "town-1",
    name: "frontend-app",
    repo_url: "github.com/edicorai/app",
    beads_prefix: "fa-",
  },
  {
    id: "rig-3",
    town_id: "town-1",
    name: "api-service",
    repo_url: "github.com/edicorai/api",
    beads_prefix: "as-",
  },
];

// ---------------------------------------------------------------------------
// Agents (8 active agents with different roles/statuses)
// ---------------------------------------------------------------------------
export const mockAgents: Agent[] = [
  {
    id: "agent-1",
    name: "mayor-alpha",
    role: "mayor",
    rig_id: "rig-1",
    status: "working",
    runtime: "claude",
    last_activity_at: minutesAgo(2),
  },
  {
    id: "agent-2",
    name: "deacon-bravo",
    role: "deacon",
    rig_id: "rig-1",
    status: "idle",
    runtime: "claude",
    last_activity_at: minutesAgo(15),
  },
  {
    id: "agent-3",
    name: "polecat-charlie",
    role: "polecat",
    rig_id: "rig-1",
    status: "working",
    runtime: "codex",
    last_activity_at: minutesAgo(1),
  },
  {
    id: "agent-4",
    name: "polecat-delta",
    role: "polecat",
    rig_id: "rig-2",
    status: "working",
    runtime: "gemini",
    last_activity_at: minutesAgo(3),
  },
  {
    id: "agent-5",
    name: "witness-echo",
    role: "witness",
    rig_id: "rig-1",
    status: "idle",
    runtime: "claude",
    last_activity_at: minutesAgo(30),
  },
  {
    id: "agent-6",
    name: "refinery-foxtrot",
    role: "refinery",
    rig_id: "rig-1",
    status: "working",
    runtime: "claude",
    last_activity_at: minutesAgo(5),
  },
  {
    id: "agent-7",
    name: "polecat-golf",
    role: "polecat",
    rig_id: "rig-3",
    status: "stuck",
    runtime: "cursor",
    last_activity_at: minutesAgo(45),
  },
  {
    id: "agent-8",
    name: "crew-hotel",
    role: "crew",
    rig_id: "rig-2",
    status: "done",
    runtime: "amp",
    last_activity_at: hoursAgo(1),
  },
];

// ---------------------------------------------------------------------------
// Feed events (15 recent events)
// ---------------------------------------------------------------------------
export const mockFeedEvents: FeedEvent[] = [
  {
    id: "ev-1",
    event_type: "agent_spawned",
    source: "system",
    summary: "Spawned polecat-charlie on vibetown",
    severity: "info",
    created_at: minutesAgo(1),
  },
  {
    id: "ev-2",
    event_type: "work_dispatched",
    source: "mayor-alpha",
    summary: "Dispatched vt-142: Fix auth middleware",
    severity: "info",
    created_at: minutesAgo(3),
  },
  {
    id: "ev-3",
    event_type: "merge_queued",
    source: "refinery-foxtrot",
    summary: "Queued PR #47 for merge",
    severity: "info",
    created_at: minutesAgo(5),
  },
  {
    id: "ev-4",
    event_type: "agent_stuck",
    source: "polecat-golf",
    summary: "Agent stuck on as-89: timeout waiting for test",
    severity: "warn",
    created_at: minutesAgo(8),
  },
  {
    id: "ev-5",
    event_type: "tests_passed",
    source: "refinery-foxtrot",
    summary: "All 142 tests passed for feat/auth-middleware",
    severity: "info",
    created_at: minutesAgo(10),
  },
  {
    id: "ev-6",
    event_type: "pr_merged",
    source: "system",
    summary: "Merged PR #45: fix/memory-leak into main",
    severity: "info",
    created_at: minutesAgo(15),
  },
  {
    id: "ev-7",
    event_type: "work_dispatched",
    source: "mayor-alpha",
    summary: "Dispatched vt-143: Add rate limiting to API",
    severity: "info",
    created_at: minutesAgo(18),
  },
  {
    id: "ev-8",
    event_type: "code_review",
    source: "witness-echo",
    summary: "Review complete for vt-140: Refactor DB layer",
    severity: "info",
    created_at: minutesAgo(22),
  },
  {
    id: "ev-9",
    event_type: "agent_spawned",
    source: "system",
    summary: "Spawned polecat-delta on frontend-app",
    severity: "info",
    created_at: minutesAgo(30),
  },
  {
    id: "ev-10",
    event_type: "build_failed",
    source: "polecat-golf",
    summary: "Build failed for as-89: missing dependency",
    severity: "critical",
    created_at: minutesAgo(35),
  },
  {
    id: "ev-11",
    event_type: "work_completed",
    source: "crew-hotel",
    summary: "Completed fa-67: Update dashboard layout",
    severity: "info",
    created_at: minutesAgo(40),
  },
  {
    id: "ev-12",
    event_type: "mail_sent",
    source: "mayor-alpha",
    summary: "Task assignment sent to polecat-charlie",
    severity: "info",
    created_at: minutesAgo(42),
  },
  {
    id: "ev-13",
    event_type: "merge_conflict",
    source: "system",
    summary: "Merge conflict detected on fix/db-migration",
    severity: "warn",
    created_at: minutesAgo(50),
  },
  {
    id: "ev-14",
    event_type: "agent_spawned",
    source: "system",
    summary: "Spawned crew-hotel on frontend-app",
    severity: "info",
    created_at: hoursAgo(1),
  },
  {
    id: "ev-15",
    event_type: "config_updated",
    source: "admin",
    summary: "Town configuration updated: enabled auto-merge",
    severity: "info",
    created_at: hoursAgo(2),
  },
];

// ---------------------------------------------------------------------------
// Mail messages (5 messages)
// ---------------------------------------------------------------------------
export const mockMail: MailMessage[] = [
  {
    id: "mail-1",
    from_addr: "mayor@vibetown",
    to_addr: "polecat-charlie@vibetown",
    subject: "New task: Implement user auth",
    body: "Please implement the user authentication module for the vibetown API.\n\nRequirements:\n- JWT-based token auth\n- Refresh token rotation\n- Rate limiting on login endpoint\n\nSee vt-142 for the full spec.",
    priority: "high",
    message_type: "task",
    read: false,
    created_at: minutesAgo(3),
  },
  {
    id: "mail-2",
    from_addr: "witness-echo@vibetown",
    to_addr: "polecat-delta@vibetown",
    subject: "Review feedback: Dashboard component",
    body: "Your dashboard component looks good overall. A few suggestions:\n\n1. Consider memoizing the chart data computation\n2. The loading skeleton could be more descriptive\n3. Add error boundary for the feed widget\n\nPlease address these before we merge.",
    priority: "normal",
    message_type: "review",
    read: true,
    created_at: minutesAgo(20),
  },
  {
    id: "mail-3",
    from_addr: "system@vibetown",
    to_addr: "polecat-golf@vibetown",
    subject: "ALERT: Build failure on api-service",
    body: "Your latest commit on branch fix/memory-leak caused a build failure.\n\nError: Cannot find module '@api/utils/cache'\n\nPlease check the import paths and ensure all dependencies are correctly specified.",
    priority: "urgent",
    message_type: "alert",
    read: false,
    created_at: minutesAgo(35),
  },
  {
    id: "mail-4",
    from_addr: "refinery-foxtrot@vibetown",
    to_addr: "mayor@vibetown",
    subject: "Merge queue status update",
    body: "Current merge queue status:\n\n- feat/auth-middleware: tests running (12/142 passed)\n- fix/db-migration: waiting for conflict resolution\n- feat/api-endpoints: all tests passed, ready to merge\n\nRecommend merging feat/api-endpoints first to unblock other PRs.",
    priority: "normal",
    message_type: "status",
    read: true,
    created_at: minutesAgo(45),
  },
  {
    id: "mail-5",
    from_addr: "deacon-bravo@vibetown",
    to_addr: "all@vibetown",
    subject: "Weekly sync: Sprint planning",
    body: "Reminder: Weekly sprint planning sync is scheduled for tomorrow.\n\nAgenda:\n1. Review completed work items\n2. Triage new issues from upstream\n3. Assign next sprint tasks\n4. Discuss merge queue improvements\n\nAll agents should prepare status updates.",
    priority: "low",
    message_type: "announcement",
    read: false,
    created_at: hoursAgo(2),
  },
];

// ---------------------------------------------------------------------------
// Merge requests (4 in queue)
// ---------------------------------------------------------------------------
export const mockMergeRequests: MergeRequest[] = [
  {
    id: "mr-1",
    branch: "feat/auth-middleware",
    target_branch: "main",
    status: "testing",
    pr_url: "#",
    agent_id: "agent-3",
    queued_at: minutesAgo(10),
  },
  {
    id: "mr-2",
    branch: "fix/db-migration",
    target_branch: "main",
    status: "pending",
    pr_url: "#",
    agent_id: "agent-4",
    queued_at: minutesAgo(25),
  },
  {
    id: "mr-3",
    branch: "feat/api-endpoints",
    target_branch: "main",
    status: "passed",
    pr_url: "#",
    agent_id: "agent-6",
    queued_at: minutesAgo(40),
  },
  {
    id: "mr-4",
    branch: "fix/memory-leak",
    target_branch: "main",
    status: "merged",
    pr_url: "#",
    agent_id: "agent-8",
    queued_at: hoursAgo(1),
  },
];
