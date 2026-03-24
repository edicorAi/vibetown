import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@vibetown/ui/components/card"
import { Badge } from "@vibetown/ui/components/badge"
import { Separator } from "@vibetown/ui/components/separator"
import { useRigs } from "@vibetown/web-core/hooks/use-rigs"
import { useAgents } from "@vibetown/web-core/hooks/use-orchestration"
import { useMergeRequests } from "@vibetown/web-core/hooks/use-merge-queue"
import type { Agent, MergeRequest } from "@vibetown/web-core/lib/api"
import { cn } from "@vibetown/ui/lib/utils"
import {
  GitBranch,
  Users,
  GitMerge,
  FolderGit2,
  Plus,
} from "lucide-react"

function statusDotColor(status: string) {
  switch (status) {
    case "working":
      return "bg-green-500 status-dot-working"
    case "idle":
      return "bg-gray-400"
    case "stuck":
      return "bg-yellow-500"
    default:
      return "bg-gray-400"
  }
}

function roleBadgeColor(role: string) {
  switch (role) {
    case "mayor":
      return "bg-orange-500/15 text-orange-700 dark:text-orange-400"
    case "witness":
      return "bg-purple-500/15 text-purple-700 dark:text-purple-400"
    case "refinery":
      return "bg-cyan-500/15 text-cyan-700 dark:text-cyan-400"
    case "polecat":
      return "bg-green-500/15 text-green-700 dark:text-green-400"
    case "crew":
      return "bg-blue-500/15 text-blue-700 dark:text-blue-400"
    case "deacon":
      return "bg-red-500/15 text-red-700 dark:text-red-400"
    default:
      return "bg-gray-500/15 text-gray-600"
  }
}

export function ProjectsPage() {
  const rigsQuery = useRigs()
  const agentsQuery = useAgents()
  const mergeQuery = useMergeRequests()

  const rigs = rigsQuery.data ?? []
  const agents = agentsQuery.data ?? []
  const merges = mergeQuery.data ?? []

  return (
    <div className="mx-auto max-w-7xl space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="font-heading text-xl font-semibold">Projects</h1>
          <p className="text-sm text-muted-foreground">
            Rigs are project registrations — each one maps to a repo with its own
            agent pool, merge queue, and work tracking.
          </p>
        </div>
        <button className="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90">
          <Plus className="size-4" />
          Add Rig
        </button>
      </div>

      <div className="grid gap-6">
        {rigs.map((rig) => {
          const rigAgents = agents.filter((a: Agent) => a.rig_id === rig.id)
          const rigMerges = merges.filter((m: MergeRequest) => m.agent_id && rigAgents.some((a: Agent) => a.id === m.agent_id))
          const workingCount = rigAgents.filter((a: Agent) => a.status === "working").length
          const idleCount = rigAgents.filter((a: Agent) => a.status === "idle").length

          return (
            <Card
              key={rig.id}
              className="animate-card-enter transition-shadow hover:shadow-md"
            >
              <CardHeader className="pb-3">
                <div className="flex items-start justify-between">
                  <div className="flex items-center gap-3">
                    <div className="flex size-10 items-center justify-center rounded-lg bg-primary/10">
                      <FolderGit2 className="size-5 text-primary" />
                    </div>
                    <div>
                      <CardTitle className="text-lg">{rig.name}</CardTitle>
                      <CardDescription className="font-mono text-xs">
                        {rig.repo_url || "No repo configured"}
                      </CardDescription>
                    </div>
                  </div>
                  <Badge variant="outline" className="font-mono text-xs">
                    {rig.beads_prefix}*
                  </Badge>
                </div>
              </CardHeader>

              <CardContent className="space-y-4">
                {/* Stats row */}
                <div className="grid grid-cols-3 gap-4">
                  <div className="flex items-center gap-2">
                    <Users className="size-4 text-muted-foreground" />
                    <div>
                      <p className="text-sm font-medium">{rigAgents.length} agents</p>
                      <p className="text-xs text-muted-foreground">
                        {workingCount} working, {idleCount} idle
                      </p>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <GitMerge className="size-4 text-muted-foreground" />
                    <div>
                      <p className="text-sm font-medium">{rigMerges.length} in queue</p>
                      <p className="text-xs text-muted-foreground">merge requests</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <GitBranch className="size-4 text-muted-foreground" />
                    <div>
                      <p className="text-sm font-medium">main</p>
                      <p className="text-xs text-muted-foreground">target branch</p>
                    </div>
                  </div>
                </div>

                <Separator />

                {/* Agent list */}
                {rigAgents.length > 0 ? (
                  <div>
                    <p className="mb-2 text-xs font-medium text-muted-foreground">
                      Active Agents
                    </p>
                    <div className="flex flex-wrap gap-2">
                      {rigAgents.map((agent: Agent) => (
                        <div
                          key={agent.id}
                          className="inline-flex items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-1.5 text-sm"
                        >
                          <span
                            className={cn(
                              "size-2 shrink-0 rounded-full",
                              statusDotColor(agent.status)
                            )}
                          />
                          <span className="font-medium">{agent.name}</span>
                          <Badge
                            variant="outline"
                            className={cn("text-[10px] px-1.5 py-0", roleBadgeColor(agent.role))}
                          >
                            {agent.role}
                          </Badge>
                        </div>
                      ))}
                    </div>
                  </div>
                ) : (
                  <p className="text-sm text-muted-foreground">
                    No agents assigned to this rig
                  </p>
                )}

                {/* Architecture hint */}
                <div className="rounded-lg border border-dashed border-border bg-muted/20 p-3">
                  <p className="text-xs text-muted-foreground">
                    <strong>Rig structure:</strong>{" "}
                    <code className="rounded bg-muted px-1 py-0.5 text-[10px]">
                      ~/gt/{rig.name}/
                    </code>{" "}
                    → .repo.git/ + witness/ + refinery/ + polecats/ + crew/
                  </p>
                </div>
              </CardContent>
            </Card>
          )
        })}
      </div>
    </div>
  )
}
