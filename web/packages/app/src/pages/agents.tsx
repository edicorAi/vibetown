import * as React from "react"
import { Badge } from "@vibetown/ui/components/badge"
import { Button } from "@vibetown/ui/components/button"
import {
  Card,
  CardContent,
} from "@vibetown/ui/components/card"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@vibetown/ui/components/dialog"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@vibetown/ui/components/alert-dialog"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@vibetown/ui/components/select"
import { Input } from "@vibetown/ui/components/input"
import {
  useAgents,
  useRigs,
  useSpawnAgent,
  useKillAgent,
} from "@vibetown/web-core/hooks/use-orchestration"
import type { Agent } from "@vibetown/web-core/lib/api"
import { Plus, Skull, Check, X } from "lucide-react"
import { cn } from "@vibetown/ui/lib/utils"

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function statusDotColor(status: string) {
  switch (status) {
    case "working":
      return "bg-green-500 status-dot-working"
    case "idle":
      return "bg-gray-400"
    case "stuck":
      return "bg-yellow-500"
    case "zombie":
      return "bg-red-500"
    case "done":
      return "bg-blue-500"
    default:
      return "bg-gray-400"
  }
}

function roleBadgeColor(role: string) {
  switch (role) {
    case "mayor":
      return "bg-primary/15 text-primary"
    case "deacon":
      return "bg-purple-500/15 text-purple-700 dark:text-purple-400"
    case "polecat":
      return "bg-blue-500/15 text-blue-700 dark:text-blue-400"
    case "witness":
      return "bg-yellow-500/15 text-yellow-700 dark:text-yellow-400"
    case "refinery":
      return "bg-green-500/15 text-green-700 dark:text-green-400"
    case "crew":
      return "bg-gray-500/15 text-gray-700 dark:text-gray-400"
    default:
      return "bg-muted text-muted-foreground"
  }
}

function runtimeBadgeColor(runtime: string) {
  switch (runtime) {
    case "claude":
      return "bg-orange-500/10 text-orange-700 dark:text-orange-400 ring-orange-500/20"
    case "codex":
      return "bg-green-500/10 text-green-700 dark:text-green-400 ring-green-500/20"
    case "gemini":
      return "bg-blue-500/10 text-blue-700 dark:text-blue-400 ring-blue-500/20"
    case "cursor":
      return "bg-purple-500/10 text-purple-700 dark:text-purple-400 ring-purple-500/20"
    case "amp":
      return "bg-pink-500/10 text-pink-700 dark:text-pink-400 ring-pink-500/20"
    default:
      return "bg-muted text-muted-foreground ring-border"
  }
}

function formatTime(iso: string) {
  if (!iso) return ""
  try {
    return new Date(iso).toLocaleString(undefined, {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    })
  } catch {
    return iso
  }
}

const AVAILABLE_ROLES = ["mayor", "deacon", "polecat", "witness", "refinery", "crew"]
const AVAILABLE_RUNTIMES = ["claude", "codex", "gemini", "cursor", "amp", "droid", "ollama", "opencode", "qwen"]

// ---------------------------------------------------------------------------
// Runtime-to-provider mapping for auth status
// ---------------------------------------------------------------------------

const RUNTIME_PROVIDER_MAP: Record<string, { providerId: string; keyField: string }> = {
  claude: { providerId: "anthropic", keyField: "ANTHROPIC_API_KEY" },
  codex: { providerId: "openai", keyField: "OPENAI_API_KEY" },
  gemini: { providerId: "google", keyField: "GOOGLE_API_KEY" },
  cursor: { providerId: "cursor", keyField: "CURSOR_API_KEY" },
  amp: { providerId: "amp", keyField: "AMP_ACCESS_TOKEN" },
  droid: { providerId: "droid", keyField: "DROID_API_KEY" },
  ollama: { providerId: "ollama", keyField: "OLLAMA_BASE_URL" },
  qwen: { providerId: "custom", keyField: "CUSTOM_API_KEY" },
}

// Default model placeholders by runtime
function getDefaultModelPlaceholder(runtime: string): string {
  // Check localStorage for ollama medium tier
  if (runtime === "ollama") {
    try {
      const saved = localStorage.getItem("vibetown-settings")
      if (saved) {
        const parsed = JSON.parse(saved) as Record<string, unknown>
        const tiers = parsed.modelTiers as Record<string, { medium?: string }> | undefined
        if (tiers?.ollama?.medium) return tiers.ollama.medium
      }
    } catch {
      // ignore
    }
    return "llama3.3"
  }

  switch (runtime) {
    case "claude": return "claude-opus-4-6"
    case "codex": return "codex-mini"
    case "gemini": return "gemini-2.5-pro"
    case "cursor": return "gpt-5.4"
    case "amp": return "default"
    case "opencode": return "default"
    case "qwen": return "qwen2.5-coder"
    case "droid": return "claude-opus"
    default: return "default"
  }
}

function isRuntimeConfigured(runtime: string): boolean {
  const mapping = RUNTIME_PROVIDER_MAP[runtime]
  if (!mapping) return false
  try {
    const saved = localStorage.getItem("vibetown-settings")
    if (!saved) return false
    const parsed = JSON.parse(saved) as Record<string, Record<string, string>>
    const providerSettings = parsed[mapping.providerId]
    if (!providerSettings) return false
    return Boolean(providerSettings[mapping.keyField]?.trim())
  } catch {
    return false
  }
}

// ---------------------------------------------------------------------------
// Runtime Auth Status Bar
// ---------------------------------------------------------------------------

const STATUS_RUNTIMES = ["claude", "codex", "gemini", "ollama", "cursor"] as const

function RuntimeAuthStatusBar() {
  // Re-read settings to detect changes
  const [, setTick] = React.useState(0)
  React.useEffect(() => {
    // Refresh every 2 seconds to pick up settings changes
    const interval = setInterval(() => setTick((t) => t + 1), 2000)
    return () => clearInterval(interval)
  }, [])

  return (
    <div className="flex flex-wrap items-center gap-x-4 gap-y-1 rounded-lg border border-border bg-muted/30 px-4 py-2">
      <span className="text-xs font-medium text-muted-foreground">Available:</span>
      {STATUS_RUNTIMES.map((rt) => {
        const configured = isRuntimeConfigured(rt)
        return (
          <span key={rt} className="flex items-center gap-1 text-xs">
            <span className="capitalize font-medium">{rt === "claude" ? "Claude" : rt === "codex" ? "Codex" : rt === "gemini" ? "Gemini" : rt === "ollama" ? "Ollama" : "Cursor"}</span>
            {configured ? (
              <Check className="size-3 text-green-500" />
            ) : (
              <X className="size-3 text-red-400" />
            )}
          </span>
        )
      })}
    </div>
  )
}

// ---------------------------------------------------------------------------
// Agent card
// ---------------------------------------------------------------------------

function AgentCard({
  agent,
  rigName,
  onKill,
  killPending,
  index,
}: {
  agent: Agent
  rigName: string
  onKill: () => void
  killPending: boolean
  index: number
}) {
  return (
    <Card
      className="group animate-card-enter transition-all hover:-translate-y-0.5 hover:shadow-md"
      style={{ animationDelay: `${index * 50}ms` }}
    >
      <CardContent className="p-4 space-y-3">
        {/* Header: name + status dot */}
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-2">
            <span
              className={cn(
                "size-2.5 shrink-0 rounded-full",
                statusDotColor(agent.status)
              )}
            />
            <h3 className="font-heading text-sm font-semibold">{agent.name}</h3>
          </div>
          <AlertDialog>
            <AlertDialogTrigger
              render={
                <Button
                  variant="ghost"
                  size="icon-xs"
                  className="opacity-0 transition-opacity group-hover:opacity-100"
                  disabled={killPending}
                >
                  <Skull className="size-3 text-muted-foreground" />
                </Button>
              }
            />
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Kill Agent</AlertDialogTitle>
                <AlertDialogDescription>
                  Are you sure you want to kill agent &ldquo;{agent.name}&rdquo;?
                  This action cannot be undone.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction variant="destructive" onClick={onKill}>
                  Kill
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>

        {/* Badges: role + runtime */}
        <div className="flex items-center gap-2">
          <Badge
            variant="outline"
            className={cn("text-[10px] border-0 font-medium", roleBadgeColor(agent.role))}
          >
            {agent.role}
          </Badge>
          <Badge
            variant="outline"
            className={cn(
              "text-[10px] font-medium ring-1",
              runtimeBadgeColor(agent.runtime)
            )}
          >
            {agent.runtime}
          </Badge>
        </div>

        {/* Meta */}
        <div className="space-y-1 text-xs text-muted-foreground">
          <div className="flex justify-between">
            <span>Rig</span>
            <span className="font-medium text-foreground">{rigName}</span>
          </div>
          <div className="flex justify-between">
            <span>Status</span>
            <span className="font-medium text-foreground capitalize">{agent.status}</span>
          </div>
          <div className="flex justify-between">
            <span>Last activity</span>
            <span>{formatTime(agent.last_activity_at)}</span>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

export function AgentsPage() {
  const agentsQuery = useAgents()
  const rigsQuery = useRigs()
  const spawnMutation = useSpawnAgent()
  const killMutation = useKillAgent()

  const agents = agentsQuery.data ?? []
  const rigs = rigsQuery.data ?? []

  // Filters
  const [roleFilter, setRoleFilter] = React.useState("all")
  const [statusFilter, setStatusFilter] = React.useState("all")

  // Spawn form state
  const [spawnOpen, setSpawnOpen] = React.useState(false)
  const [spawnName, setSpawnName] = React.useState("")
  const [spawnRole, setSpawnRole] = React.useState("")
  const [spawnRuntime, setSpawnRuntime] = React.useState("")
  const [spawnModel, setSpawnModel] = React.useState("")
  const [spawnRig, setSpawnRig] = React.useState("")

  const roles = Array.from(new Set(agents.map((a) => a.role))).sort()
  const statuses = Array.from(new Set(agents.map((a) => a.status))).sort()
  const rigMap = new Map(rigs.map((r) => [r.id, r.name]))

  const filteredAgents = agents.filter((a: Agent) => {
    if (roleFilter !== "all" && a.role !== roleFilter) return false
    if (statusFilter !== "all" && a.status !== statusFilter) return false
    return true
  })

  function handleSpawn() {
    if (!spawnName || !spawnRole || !spawnRuntime || !spawnRig) return
    spawnMutation.mutate(
      {
        name: spawnName,
        role: spawnRole,
        runtime: spawnRuntime,
        rig_id: spawnRig,
      },
      {
        onSuccess: () => {
          setSpawnOpen(false)
          setSpawnName("")
          setSpawnRole("")
          setSpawnRuntime("")
          setSpawnModel("")
          setSpawnRig("")
        },
      }
    )
  }

  return (
    <div className="mx-auto max-w-7xl space-y-6">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <h1 className="font-heading text-xl font-semibold">Agents</h1>

        <div className="flex flex-wrap items-center gap-2">
          {/* Role filter */}
          <Select value={roleFilter} onValueChange={(v) => setRoleFilter(v ?? "all")}>
            <SelectTrigger className="w-36">
              <SelectValue placeholder="Role" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Roles</SelectItem>
              {roles.map((r) => (
                <SelectItem key={r} value={r}>
                  {r}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          {/* Status filter */}
          <Select value={statusFilter} onValueChange={(v) => setStatusFilter(v ?? "all")}>
            <SelectTrigger className="w-36">
              <SelectValue placeholder="Status" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Statuses</SelectItem>
              {statuses.map((s) => (
                <SelectItem key={s} value={s}>
                  {s}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          {/* Spawn button */}
          <Dialog open={spawnOpen} onOpenChange={setSpawnOpen}>
            <DialogTrigger
              render={
                <Button size="sm" className="gap-1.5">
                  <Plus className="size-3.5" />
                  Spawn Agent
                </Button>
              }
            />
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Spawn Agent</DialogTitle>
                <DialogDescription>
                  Create a new agent with the specified parameters.
                </DialogDescription>
              </DialogHeader>
              <div className="grid gap-3">
                <div className="grid gap-1.5">
                  <label className="text-sm font-medium" htmlFor="spawn-name">
                    Name
                  </label>
                  <Input
                    id="spawn-name"
                    value={spawnName}
                    onChange={(e) => setSpawnName(e.target.value)}
                    placeholder="agent-name"
                  />
                </div>
                <div className="grid gap-1.5">
                  <label className="text-sm font-medium" htmlFor="spawn-role">
                    Role
                  </label>
                  <Select value={spawnRole} onValueChange={(v) => setSpawnRole(v ?? "")}>
                    <SelectTrigger className="w-full">
                      <SelectValue placeholder="Select a role" />
                    </SelectTrigger>
                    <SelectContent>
                      {AVAILABLE_ROLES.map((r) => (
                        <SelectItem key={r} value={r}>
                          {r}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <div className="grid gap-1.5">
                  <label className="text-sm font-medium" htmlFor="spawn-runtime">
                    Runtime
                  </label>
                  <Select value={spawnRuntime} onValueChange={(v) => setSpawnRuntime(v ?? "")}>
                    <SelectTrigger className="w-full">
                      <SelectValue placeholder="Select a runtime" />
                    </SelectTrigger>
                    <SelectContent>
                      {AVAILABLE_RUNTIMES.map((r) => (
                        <SelectItem key={r} value={r}>
                          {r}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <div className="grid gap-1.5">
                  <label className="text-sm font-medium" htmlFor="spawn-model">
                    Model
                  </label>
                  <Input
                    id="spawn-model"
                    value={spawnModel}
                    onChange={(e) => setSpawnModel(e.target.value)}
                    placeholder={spawnRuntime ? getDefaultModelPlaceholder(spawnRuntime) : "Select a runtime first"}
                    className="font-mono text-xs"
                  />
                  <p className="text-[10px] text-muted-foreground/60">
                    Optional. Leave empty to use the default model for the selected runtime.
                  </p>
                </div>
                <div className="grid gap-1.5">
                  <label className="text-sm font-medium">Rig</label>
                  <Select value={spawnRig} onValueChange={(v) => setSpawnRig(v ?? "")}>
                    <SelectTrigger className="w-full">
                      <SelectValue placeholder="Select a rig" />
                    </SelectTrigger>
                    <SelectContent>
                      {rigs.map((rig) => (
                        <SelectItem key={rig.id} value={rig.id}>
                          {rig.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>
              <DialogFooter>
                <Button
                  onClick={handleSpawn}
                  disabled={
                    spawnMutation.isPending ||
                    !spawnName ||
                    !spawnRole ||
                    !spawnRuntime ||
                    !spawnRig
                  }
                >
                  {spawnMutation.isPending ? "Spawning..." : "Spawn"}
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>
      </div>

      {/* Runtime Auth Status Bar */}
      <RuntimeAuthStatusBar />

      {agentsQuery.isLoading ? (
        <p className="text-sm text-muted-foreground">Loading agents...</p>
      ) : filteredAgents.length === 0 ? (
        <p className="text-sm text-muted-foreground">
          No agents found matching the current filters.
        </p>
      ) : (
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          {filteredAgents.map((agent: Agent, index: number) => (
            <AgentCard
              key={agent.id}
              agent={agent}
              rigName={rigMap.get(agent.rig_id) ?? agent.rig_id}
              onKill={() => killMutation.mutate(agent.id)}
              killPending={killMutation.isPending}
              index={index}
            />
          ))}
        </div>
      )}
    </div>
  )
}
