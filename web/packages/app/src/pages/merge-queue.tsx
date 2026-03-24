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
  DialogClose,
} from "@vibetown/ui/components/dialog"
import { Input } from "@vibetown/ui/components/input"
import {
  useMergeRequests,
  useQueueMerge,
} from "@vibetown/web-core/hooks/use-merge-queue"
import { useAgents } from "@vibetown/web-core/hooks/use-orchestration"
import type { MergeRequest } from "@vibetown/web-core/lib/api"
import { GitMerge, Plus, ExternalLink, Clock, CheckCircle2, Loader2, CircleDot } from "lucide-react"
import { cn } from "@vibetown/ui/lib/utils"

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function mergeStatusColor(status: string) {
  switch (status) {
    case "pending":
      return "bg-gray-500/15 text-gray-700 dark:text-gray-300 border-gray-300 dark:border-gray-600"
    case "testing":
      return "bg-blue-500/15 text-blue-700 dark:text-blue-400 border-blue-400"
    case "passed":
      return "bg-green-500/15 text-green-700 dark:text-green-400 border-green-400"
    case "failed":
      return "bg-red-500/15 text-red-700 dark:text-red-400 border-red-400"
    case "merged":
      return "bg-purple-500/15 text-purple-700 dark:text-purple-400 border-purple-400"
    default:
      return "bg-gray-500/15 text-gray-600 border-gray-300"
  }
}

function mergeStatusIcon(status: string) {
  switch (status) {
    case "pending":
      return <Clock className="size-4" />
    case "testing":
      return <Loader2 className="size-4 animate-spin" />
    case "passed":
      return <CheckCircle2 className="size-4" />
    case "merged":
      return <GitMerge className="size-4" />
    default:
      return <CircleDot className="size-4" />
  }
}

function mergeStatusBorderTop(status: string) {
  switch (status) {
    case "pending":
      return "border-t-gray-400"
    case "testing":
      return "border-t-blue-500"
    case "passed":
      return "border-t-green-500"
    case "failed":
      return "border-t-red-500"
    case "merged":
      return "border-t-purple-500"
    default:
      return "border-t-gray-400"
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

// Pipeline stages order
const STAGE_ORDER = ["pending", "testing", "passed", "merged"] as const

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

export function MergeQueuePage() {
  const mrQuery = useMergeRequests()
  const agentsQuery = useAgents()
  const queueMutation = useQueueMerge()

  const merges = mrQuery.data ?? []
  const agents = agentsQuery.data ?? []
  const agentMap = new Map(agents.map((a) => [a.id, a.name]))

  const [dialogOpen, setDialogOpen] = React.useState(false)
  const [branch, setBranch] = React.useState("")
  const [targetBranch, setTargetBranch] = React.useState("main")
  const [prUrl, setPrUrl] = React.useState("")

  function handleQueue() {
    if (!branch || !targetBranch) return
    queueMutation.mutate(
      { branch, target_branch: targetBranch, pr_url: prUrl },
      {
        onSuccess: () => {
          setDialogOpen(false)
          setBranch("")
          setTargetBranch("main")
          setPrUrl("")
        },
      }
    )
  }

  // Group by stage
  const byStage = new Map<string, MergeRequest[]>()
  for (const stage of STAGE_ORDER) {
    byStage.set(stage, [])
  }
  for (const mr of merges) {
    const group = byStage.get(mr.status)
    if (group) {
      group.push(mr)
    } else {
      // Unknown status goes to pending
      byStage.get("pending")!.push(mr)
    }
  }

  return (
    <div className="mx-auto max-w-7xl space-y-6">
      <div className="flex items-center justify-between gap-3">
        <div className="flex items-center gap-3">
          <div className="flex size-8 items-center justify-center rounded-lg bg-purple-500/10">
            <GitMerge className="size-4 text-purple-600 dark:text-purple-400" />
          </div>
          <div>
            <h1 className="font-heading text-xl font-semibold">Merge Queue</h1>
            <p className="text-xs text-muted-foreground">
              {merges.length} merge request{merges.length !== 1 ? "s" : ""} in pipeline
            </p>
          </div>
        </div>
        <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
          <DialogTrigger
            render={
              <Button size="sm" className="gap-1.5">
                <Plus className="size-3.5" />
                Queue Merge
              </Button>
            }
          />
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Queue Merge Request</DialogTitle>
              <DialogDescription>
                Add a branch to the merge queue.
              </DialogDescription>
            </DialogHeader>
            <div className="grid gap-3">
              <div className="grid gap-1.5">
                <label className="text-sm font-medium" htmlFor="mr-branch">
                  Branch
                </label>
                <Input
                  id="mr-branch"
                  value={branch}
                  onChange={(e) => setBranch(e.target.value)}
                  placeholder="feature/my-branch"
                />
              </div>
              <div className="grid gap-1.5">
                <label className="text-sm font-medium" htmlFor="mr-target">
                  Target Branch
                </label>
                <Input
                  id="mr-target"
                  value={targetBranch}
                  onChange={(e) => setTargetBranch(e.target.value)}
                  placeholder="main"
                />
              </div>
              <div className="grid gap-1.5">
                <label className="text-sm font-medium" htmlFor="mr-pr-url">
                  PR URL (optional)
                </label>
                <Input
                  id="mr-pr-url"
                  value={prUrl}
                  onChange={(e) => setPrUrl(e.target.value)}
                  placeholder="https://github.com/..."
                />
              </div>
            </div>
            <DialogFooter>
              <DialogClose render={<Button variant="outline" />}>
                Cancel
              </DialogClose>
              <Button
                onClick={handleQueue}
                disabled={queueMutation.isPending || !branch || !targetBranch}
              >
                {queueMutation.isPending ? "Queueing..." : "Queue"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      {mrQuery.isLoading ? (
        <p className="text-sm text-muted-foreground">Loading merge queue...</p>
      ) : merges.length === 0 ? (
        <div className="flex flex-col items-center gap-3 py-12">
          <div className="flex size-12 items-center justify-center rounded-full bg-muted">
            <GitMerge className="size-6 text-muted-foreground/50" />
          </div>
          <p className="text-sm text-muted-foreground">
            No merge requests in the queue.
          </p>
        </div>
      ) : (
        <>
          {/* Pipeline visualization (horizontal) */}
          <div className="grid gap-4 md:grid-cols-4">
            {STAGE_ORDER.map((stage) => {
              const items = byStage.get(stage) ?? []
              return (
                <div key={stage} className="space-y-3">
                  {/* Stage header */}
                  <div className="flex items-center gap-2">
                    <span className={cn(
                      "flex size-6 items-center justify-center rounded-full",
                      mergeStatusColor(stage)
                    )}>
                      {mergeStatusIcon(stage)}
                    </span>
                    <span className="text-sm font-medium capitalize">{stage}</span>
                    <span className="ml-auto rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
                      {items.length}
                    </span>
                  </div>

                  {/* Items */}
                  <div className="space-y-2">
                    {items.length === 0 ? (
                      <div className="rounded-lg border border-dashed border-border p-4 text-center text-xs text-muted-foreground">
                        No items
                      </div>
                    ) : (
                      items.map((mr: MergeRequest, index: number) => (
                        <Card
                          key={mr.id}
                          className={cn(
                            "animate-card-enter border-t-2 transition-all hover:-translate-y-0.5 hover:shadow-md",
                            mergeStatusBorderTop(mr.status)
                          )}
                          style={{ animationDelay: `${index * 50}ms` }}
                        >
                          <CardContent className="p-3 space-y-2">
                            <code className="block truncate text-xs font-semibold">
                              {mr.branch}
                            </code>
                            <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
                              <span>&rarr; {mr.target_branch}</span>
                              {mr.pr_url && mr.pr_url !== "#" && (
                                <a
                                  href={mr.pr_url}
                                  target="_blank"
                                  rel="noopener noreferrer"
                                  className="inline-flex items-center gap-0.5 text-primary hover:underline"
                                >
                                  <ExternalLink className="size-2.5" />
                                  PR
                                </a>
                              )}
                            </div>
                            <div className="flex items-center justify-between text-xs text-muted-foreground">
                              <span>{agentMap.get(mr.agent_id) ?? mr.agent_id ?? ""}</span>
                              <span>{formatRelative(mr.queued_at)}</span>
                            </div>
                          </CardContent>
                        </Card>
                      ))
                    )}
                  </div>
                </div>
              )
            })}
          </div>

          {/* Summary bar */}
          <div className="flex flex-wrap items-center gap-3 rounded-lg border border-border bg-card p-3">
            <span className="text-xs font-medium text-muted-foreground">Summary:</span>
            {STAGE_ORDER.map((stage) => {
              const count = (byStage.get(stage) ?? []).length
              if (count === 0) return null
              return (
                <Badge
                  key={stage}
                  variant="outline"
                  className={cn("text-xs", mergeStatusColor(stage))}
                >
                  {stage}: {count}
                </Badge>
              )
            })}
          </div>
        </>
      )}
    </div>
  )
}
