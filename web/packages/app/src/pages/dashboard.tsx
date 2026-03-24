import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@vibetown/ui/components/card"
import { Badge } from "@vibetown/ui/components/badge"
import { ScrollArea } from "@vibetown/ui/components/scroll-area"
import { Separator } from "@vibetown/ui/components/separator"
import { useTown, useAgents, useRigs } from "@vibetown/web-core/hooks/use-orchestration"
import { useFeedEvents } from "@vibetown/web-core/hooks/use-feed"
import { useMergeRequests } from "@vibetown/web-core/hooks/use-merge-queue"
import type { Agent, FeedEvent, MergeRequest } from "@vibetown/web-core/lib/api"
import { cn } from "@vibetown/ui/lib/utils"
import {
  Activity,
  GitMerge,
  Users,
  Building2,
} from "lucide-react"

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

function severityBorderColor(severity: string) {
  switch (severity) {
    case "critical":
      return "border-l-red-500"
    case "warn":
      return "border-l-yellow-500"
    case "info":
      return "border-l-blue-500"
    default:
      return "border-l-gray-300 dark:border-l-gray-600"
  }
}

function mergeStatusColor(status: string) {
  switch (status) {
    case "pending":
      return "bg-gray-500/15 text-gray-700 dark:text-gray-300"
    case "testing":
      return "bg-blue-500/15 text-blue-700 dark:text-blue-400"
    case "passed":
      return "bg-green-500/15 text-green-700 dark:text-green-400"
    case "merged":
      return "bg-purple-500/15 text-purple-700 dark:text-purple-400"
    case "failed":
      return "bg-red-500/15 text-red-700 dark:text-red-400"
    default:
      return "bg-gray-500/15 text-gray-600"
  }
}

function formatTime(iso: string) {
  if (!iso) return ""
  try {
    return new Date(iso).toLocaleTimeString(undefined, {
      hour: "2-digit",
      minute: "2-digit",
    })
  } catch {
    return iso
  }
}

function formatRelative(iso: string) {
  if (!iso) return ""
  try {
    const diff = Date.now() - new Date(iso).getTime()
    const mins = Math.floor(diff / 60_000)
    if (mins < 1) return "just now"
    if (mins < 60) return `${mins}m ago`
    const hrs = Math.floor(mins / 60)
    if (hrs < 24) return `${hrs}h ago`
    return `${Math.floor(hrs / 24)}d ago`
  } catch {
    return iso
  }
}

function agentSummary(agents: Agent[]) {
  const byRole: Record<string, number> = {}
  const byStatus: Record<string, number> = {}
  for (const a of agents) {
    byRole[a.role] = (byRole[a.role] ?? 0) + 1
    byStatus[a.status] = (byStatus[a.status] ?? 0) + 1
  }
  return { byRole, byStatus }
}

function mergeStatusCounts(merges: MergeRequest[]) {
  const counts = { pending: 0, testing: 0, merged: 0, passed: 0, failed: 0 }
  for (const mr of merges) {
    const s = mr.status as keyof typeof counts
    if (s in counts) counts[s]++
  }
  return counts
}

// ---------------------------------------------------------------------------
// Dashboard
// ---------------------------------------------------------------------------

export function DashboardPage() {
  const townQuery = useTown()
  const agentsQuery = useAgents()
  const rigsQuery = useRigs()
  const feedQuery = useFeedEvents()
  const mergeQuery = useMergeRequests()

  const town = townQuery.data
  const agents = agentsQuery.data ?? []
  const rigs = rigsQuery.data ?? []
  const feedEvents = (feedQuery.data ?? []).slice(0, 20)
  const merges = mergeQuery.data ?? []

  const { byRole, byStatus } = agentSummary(agents)
  const mergeCounts = mergeStatusCounts(merges)
  const rigMap = new Map(rigs.map((r) => [r.id, r.name]))

  const workingCount = byStatus["working"] ?? 0
  const stuckCount = byStatus["stuck"] ?? 0

  return (
    <div className="mx-auto max-w-7xl space-y-6">
      <h1 className="font-heading text-xl font-semibold">Dashboard</h1>

      {/* Summary stat cards */}
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Card className="animate-card-enter" style={{ animationDelay: "0ms" }}>
          <CardContent className="flex items-center gap-4 p-4">
            <div className="flex size-10 items-center justify-center rounded-lg bg-primary/10">
              <Building2 className="size-5 text-primary" />
            </div>
            <div>
              <p className="text-2xl font-bold">{town?.name ?? "Loading..."}</p>
              <p className="text-xs text-muted-foreground">
                {rigs.length} rig{rigs.length !== 1 ? "s" : ""} configured
              </p>
            </div>
          </CardContent>
        </Card>

        <Card className="animate-card-enter" style={{ animationDelay: "50ms" }}>
          <CardContent className="flex items-center gap-4 p-4">
            <div className="flex size-10 items-center justify-center rounded-lg bg-green-500/10">
              <Users className="size-5 text-green-600 dark:text-green-400" />
            </div>
            <div>
              <p className="text-2xl font-bold">{agents.length}</p>
              <p className="text-xs text-muted-foreground">
                {workingCount} working{stuckCount > 0 ? `, ${stuckCount} stuck` : ""}
              </p>
            </div>
          </CardContent>
        </Card>

        <Card className="animate-card-enter" style={{ animationDelay: "100ms" }}>
          <CardContent className="flex items-center gap-4 p-4">
            <div className="flex size-10 items-center justify-center rounded-lg bg-blue-500/10">
              <Activity className="size-5 text-blue-600 dark:text-blue-400" />
            </div>
            <div>
              <p className="text-2xl font-bold">{feedEvents.length}</p>
              <p className="text-xs text-muted-foreground">recent events</p>
            </div>
          </CardContent>
        </Card>

        <Card className="animate-card-enter" style={{ animationDelay: "150ms" }}>
          <CardContent className="flex items-center gap-4 p-4">
            <div className="flex size-10 items-center justify-center rounded-lg bg-purple-500/10">
              <GitMerge className="size-5 text-purple-600 dark:text-purple-400" />
            </div>
            <div>
              <p className="text-2xl font-bold">{merges.length}</p>
              <p className="text-xs text-muted-foreground">
                in merge queue
              </p>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* 3-column layout */}
      <div className="grid gap-6 lg:grid-cols-[1fr_1.2fr_1fr]">
        {/* Left column: Town + Agents */}
        <div className="space-y-6">
          {/* Agents by status */}
          <Card className="animate-card-enter" style={{ animationDelay: "200ms" }}>
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium">Active Agents</CardTitle>
              <CardDescription>
                {agents.length} total across {rigs.length} rigs
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-3">
              {agents.length === 0 ? (
                <p className="text-sm text-muted-foreground">No agents running</p>
              ) : (
                <div className="space-y-2">
                  {agents.map((agent) => (
                    <div
                      key={agent.id}
                      className="flex items-center gap-3 rounded-lg p-2 transition-colors hover:bg-muted/50"
                    >
                      <span
                        className={cn(
                          "size-2.5 shrink-0 rounded-full",
                          statusDotColor(agent.status)
                        )}
                      />
                      <div className="min-w-0 flex-1">
                        <p className="truncate text-sm font-medium">
                          {agent.name}
                        </p>
                        <p className="text-xs text-muted-foreground">
                          {agent.role} &middot; {rigMap.get(agent.rig_id) ?? agent.rig_id}
                        </p>
                      </div>
                      <span className="shrink-0 text-xs text-muted-foreground">
                        {formatRelative(agent.last_activity_at)}
                      </span>
                    </div>
                  ))}
                </div>
              )}

              <Separator />

              {/* Role + status badges */}
              <div>
                <p className="mb-1.5 text-xs font-medium text-muted-foreground">
                  By Role
                </p>
                <div className="flex flex-wrap gap-1.5">
                  {Object.entries(byRole).map(([role, count]) => (
                    <Badge key={role} variant="secondary" className="text-xs">
                      {role}: {count}
                    </Badge>
                  ))}
                </div>
              </div>

              <div>
                <p className="mb-1.5 text-xs font-medium text-muted-foreground">
                  By Status
                </p>
                <div className="flex flex-wrap gap-1.5">
                  {Object.entries(byStatus).map(([status, count]) => (
                    <span
                      key={status}
                      className="inline-flex items-center gap-1.5 rounded-md bg-muted px-2 py-0.5 text-xs font-medium"
                    >
                      <span
                        className={cn(
                          "size-1.5 rounded-full",
                          statusDotColor(status)
                        )}
                      />
                      {status}: {count}
                    </span>
                  ))}
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Center column: Feed */}
        <Card className="animate-card-enter" style={{ animationDelay: "250ms" }}>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium">Activity Feed</CardTitle>
            <CardDescription>Recent events across all rigs</CardDescription>
          </CardHeader>
          <CardContent>
            {feedEvents.length === 0 ? (
              <p className="text-sm text-muted-foreground">No events yet</p>
            ) : (
              <ScrollArea className="h-[500px]">
                <div className="space-y-1 pr-3">
                  {feedEvents.map((ev: FeedEvent) => (
                    <div
                      key={ev.id}
                      className={cn(
                        "rounded-lg border-l-2 p-3 transition-colors hover:bg-muted/30",
                        severityBorderColor(ev.severity)
                      )}
                    >
                      <div className="flex items-start justify-between gap-2">
                        <div className="min-w-0 flex-1">
                          <p className="text-sm leading-snug">{ev.summary}</p>
                          <div className="mt-1 flex items-center gap-2 text-xs text-muted-foreground">
                            <Badge variant="outline" className="text-[10px] px-1.5 py-0">
                              {ev.event_type.replace(/_/g, " ")}
                            </Badge>
                            <span>{ev.source}</span>
                          </div>
                        </div>
                        <span className="shrink-0 text-xs text-muted-foreground">
                          {formatTime(ev.created_at)}
                        </span>
                      </div>
                    </div>
                  ))}
                </div>
              </ScrollArea>
            )}
          </CardContent>
        </Card>

        {/* Right column: Merge Queue */}
        <Card className="animate-card-enter" style={{ animationDelay: "300ms" }}>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium">Merge Queue</CardTitle>
            <CardDescription>
              {merges.length} merge request{merges.length !== 1 ? "s" : ""}
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Status summary */}
            <div className="flex flex-wrap gap-2">
              {Object.entries(mergeCounts)
                .filter(([, count]) => count > 0)
                .map(([status, count]) => (
                  <Badge
                    key={status}
                    variant="outline"
                    className={cn("text-xs", mergeStatusColor(status))}
                  >
                    {status}: {count}
                  </Badge>
                ))}
            </div>

            <Separator />

            {/* Individual merge items */}
            <div className="space-y-3">
              {merges.map((mr: MergeRequest) => {
                const agent = agents.find((a) => a.id === mr.agent_id)
                return (
                  <div
                    key={mr.id}
                    className="rounded-lg border border-border bg-muted/30 p-3 space-y-2"
                  >
                    <div className="flex items-center justify-between gap-2">
                      <code className="truncate text-xs font-medium">
                        {mr.branch}
                      </code>
                      <Badge
                        variant="outline"
                        className={cn("shrink-0 text-[10px]", mergeStatusColor(mr.status))}
                      >
                        {mr.status}
                      </Badge>
                    </div>
                    <div className="flex items-center gap-2 text-xs text-muted-foreground">
                      <span>&rarr; {mr.target_branch}</span>
                      <span>&middot;</span>
                      <span>{agent?.name ?? mr.agent_id}</span>
                    </div>
                    <p className="text-xs text-muted-foreground">
                      {formatRelative(mr.queued_at)}
                    </p>
                  </div>
                )
              })}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
