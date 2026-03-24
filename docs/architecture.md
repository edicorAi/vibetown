# Vibetown Architecture

## Overview

Vibetown is an all-in-one workspace for AI-assisted software development. It combines multi-agent orchestration (from Gastown) with a visual kanban board and agent execution environment (from Vibe-Kanban).

## System Components

```
┌─────────────────────────────────────────────────────────┐
│  Browser                                                 │
│  ┌─────────────────────────────────────────────────────┐│
│  │  React Frontend (shadcn/ui, TanStack, Tailwind v4)  ││
│  │  - Orchestration Dashboard                          ││
│  │  - Agent Management                                 ││
│  │  - Mail Viewer                                      ││
│  │  - Merge Queue                                      ││
│  │  - Kanban Board (work_items)                        ││
│  └────────────────────┬────────────────────────────────┘│
└───────────────────────┼─────────────────────────────────┘
                        │ HTTP/WebSocket
┌───────────────────────┼─────────────────────────────────┐
│  Kubernetes Pod        │                                 │
│  ┌────────────────────┴────────────────────────────────┐│
│  │  Rust Server (Axum)                        :3000    ││
│  │  - REST API + WebSocket                             ││
│  │  - SQLx (SQLite/PostgreSQL)                         ││
│  │  - Agent executor adapters                          ││
│  │  - Embedded React frontend                          ││
│  └────────────────────┬────────────────────────────────┘│
│                       │ gRPC (localhost:50051)           │
│  ┌────────────────────┴────────────────────────────────┐│
│  │  Go Engine (Gastown)                       :50051   ││
│  │  - OrchestrationService (agents, convoys, rigs)     ││
│  │  - FeedService (real-time event streaming)          ││
│  │  - MailService (inter-agent messaging)              ││
│  │  - Daemon (watchdogs, health, scheduling)           ││
│  └─────────────────────────────────────────────────────┘│
│                                                          │
│  Shared: emptyDir/PVC for git worktrees                  │
└──────────────────────────────────────────────────────────┘
                        │
              ┌─────────┴─────────┐
              │  PostgreSQL (RDS)  │
              │  or SQLite (local) │
              └───────────────────┘
```

## Monorepo Structure

```
vibetown/
├── engine/          Go orchestration engine (from Gastown)
├── server/          Rust API server (from Vibe-Kanban)
├── web/             React frontend (shadcn/ui)
├── proto/           gRPC protobuf definitions
├── deploy/          Docker + Helm + CI
└── tests/           E2E tests
```

## Data Flow

1. **User action in browser** → HTTP request to Rust server
2. **Rust server** handles request:
   - Direct DB queries for CRUD operations
   - gRPC call to Go engine for orchestration operations
3. **Go engine** executes orchestration logic:
   - Manages agent lifecycle via tmux sessions
   - Tracks convoy progress
   - Routes inter-agent mail
4. **Real-time updates** flow back via:
   - gRPC `StreamEvents` → Rust WebSocket → Browser

## Key Design Decisions

- **Rust + Go sidecar**: Each language plays to its strengths. Rust handles HTTP/WS, DB, and executor adapters. Go handles the orchestration state machine that was already built in Gastown.
- **gRPC bridge**: Clean boundary between the two runtimes. In K8s, this is a standard sidecar pattern with pod-internal networking.
- **Unified work_items**: Single table replacing both Vibe-Kanban's `tasks` and Gastown's beads, supporting tasks, messages, merge-requests, and molecules.
- **shadcn/ui from scratch**: Fresh frontend with modern stack (React 19, Tailwind v4) rather than porting the existing Vibe-Kanban frontend.

## Agent Execution

| Mode | Handler | When |
|------|---------|------|
| Interactive | Rust executor system | User starts a workspace session in UI |
| Autonomous | Go engine via tmux | Witness, deacon, refinery, autonomous polecats |

Supported runtimes: Claude, Codex, Gemini, Cursor, AMP, OpenCode, Qwen, Droid.

## Database

- **Local development**: SQLite (zero config)
- **Production (K8s)**: PostgreSQL (managed, e.g., RDS/Cloud SQL)
- **Schema**: 48 existing migrations from Vibe-Kanban + 9 new orchestration tables
