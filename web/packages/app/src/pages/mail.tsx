import * as React from "react"
import { Badge } from "@vibetown/ui/components/badge"
import { Button } from "@vibetown/ui/components/button"
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@vibetown/ui/components/tabs"
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@vibetown/ui/components/select"
import { Input } from "@vibetown/ui/components/input"
import { Textarea } from "@vibetown/ui/components/textarea"
import { ScrollArea } from "@vibetown/ui/components/scroll-area"
import { Separator } from "@vibetown/ui/components/separator"
import {
  useInbox,
  useSent,
  useMailQueue,
  useSendMail,
  useMarkRead,
} from "@vibetown/web-core/hooks/use-mail"
import type { MailMessage } from "@vibetown/web-core/lib/api"
import { Mail as MailIcon, Send, Check } from "lucide-react"
import { cn } from "@vibetown/ui/lib/utils"

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function priorityDotColor(priority: string) {
  switch (priority) {
    case "urgent":
      return "bg-red-500"
    case "high":
      return "bg-orange-500"
    case "normal":
      return "bg-blue-500"
    case "low":
      return "bg-gray-400"
    default:
      return "bg-gray-400"
  }
}

function priorityVariant(
  priority: string
): "default" | "secondary" | "outline" | "destructive" {
  switch (priority) {
    case "urgent":
      return "destructive"
    case "high":
      return "default"
    case "normal":
      return "secondary"
    case "low":
      return "outline"
    default:
      return "secondary"
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

// ---------------------------------------------------------------------------
// Message list
// ---------------------------------------------------------------------------

function MessageList({
  messages,
  isLoading,
  onSelect,
  selectedId,
}: {
  messages: MailMessage[]
  isLoading: boolean
  onSelect: (msg: MailMessage) => void
  selectedId: string | null
}) {
  if (isLoading) {
    return <p className="p-4 text-sm text-muted-foreground">Loading...</p>
  }
  if (messages.length === 0) {
    return <p className="p-4 text-sm text-muted-foreground">No messages</p>
  }
  return (
    <ScrollArea className="h-[500px]">
      <ul className="divide-y divide-border pr-3">
        {messages.map((msg) => (
          <li key={msg.id}>
            <button
              type="button"
              onClick={() => onSelect(msg)}
              className={cn(
                "flex w-full flex-col gap-1.5 px-4 py-3 text-left text-sm transition-colors hover:bg-muted/50",
                selectedId === msg.id && "bg-muted",
              )}
            >
              <div className="flex items-center gap-2">
                {/* Unread indicator */}
                {!msg.read && (
                  <span className="size-1.5 shrink-0 rounded-full bg-primary" />
                )}
                {/* Priority dot */}
                <span
                  className={cn(
                    "size-2 shrink-0 rounded-full",
                    priorityDotColor(msg.priority)
                  )}
                />
                <span
                  className={cn(
                    "flex-1 truncate",
                    !msg.read && "font-semibold"
                  )}
                >
                  {msg.subject || "(no subject)"}
                </span>
                <Badge
                  variant={priorityVariant(msg.priority)}
                  className="shrink-0 text-[10px]"
                >
                  {msg.priority}
                </Badge>
              </div>
              <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <span className="truncate">From: {msg.from_addr}</span>
                <span className="ml-auto shrink-0">
                  {formatTime(msg.created_at)}
                </span>
              </div>
            </button>
          </li>
        ))}
      </ul>
    </ScrollArea>
  )
}

// ---------------------------------------------------------------------------
// Message detail
// ---------------------------------------------------------------------------

function MessageDetail({
  message,
  onMarkRead,
  markReadPending,
}: {
  message: MailMessage | null
  onMarkRead: (id: string) => void
  markReadPending: boolean
}) {
  if (!message) {
    return (
      <div className="flex h-[500px] items-center justify-center text-sm text-muted-foreground">
        <div className="flex flex-col items-center gap-3">
          <div className="flex size-12 items-center justify-center rounded-full bg-muted">
            <MailIcon className="size-6 text-muted-foreground/50" />
          </div>
          <p>Select a message to view</p>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-4 p-5">
      <div className="flex items-start justify-between gap-3">
        <div className="space-y-1.5">
          <h3 className="font-heading text-base font-semibold">
            {message.subject || "(no subject)"}
          </h3>
          <div className="flex flex-wrap gap-x-4 gap-y-1 text-xs text-muted-foreground">
            <span>From: {message.from_addr}</span>
            <span>To: {message.to_addr}</span>
            <span>{formatTime(message.created_at)}</span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span
            className={cn(
              "size-2.5 rounded-full",
              priorityDotColor(message.priority)
            )}
          />
          <Badge variant={priorityVariant(message.priority)}>
            {message.priority}
          </Badge>
          {!message.read && (
            <Button
              variant="outline"
              size="xs"
              onClick={() => onMarkRead(message.id)}
              disabled={markReadPending}
              className="gap-1"
            >
              <Check className="size-3" />
              Mark Read
            </Button>
          )}
        </div>
      </div>
      <Separator />
      <div className="whitespace-pre-wrap text-sm leading-relaxed text-foreground/90">
        {message.body}
      </div>
    </div>
  )
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

export function MailPage() {
  const inboxQuery = useInbox()
  const sentQuery = useSent()
  const queueQuery = useMailQueue()
  const sendMutation = useSendMail()
  const markReadMutation = useMarkRead()

  const [selected, setSelected] = React.useState<MailMessage | null>(null)
  const [composeOpen, setComposeOpen] = React.useState(false)
  const [composeFrom, setComposeFrom] = React.useState("")
  const [composeTo, setComposeTo] = React.useState("")
  const [composeSubject, setComposeSubject] = React.useState("")
  const [composeBody, setComposeBody] = React.useState("")
  const [composePriority, setComposePriority] = React.useState("normal")

  function handleSend() {
    if (!composeFrom || !composeTo || !composeSubject) return
    sendMutation.mutate(
      {
        from_addr: composeFrom,
        to_addr: composeTo,
        subject: composeSubject,
        body: composeBody,
        priority: composePriority,
      },
      {
        onSuccess: () => {
          setComposeOpen(false)
          setComposeFrom("")
          setComposeTo("")
          setComposeSubject("")
          setComposeBody("")
          setComposePriority("normal")
        },
      }
    )
  }

  // Count unread
  const inbox = inboxQuery.data ?? []
  const unreadCount = inbox.filter((m) => !m.read).length

  return (
    <div className="mx-auto max-w-7xl space-y-6">
      <div className="flex items-center justify-between gap-3">
        <div className="flex items-center gap-3">
          <h1 className="font-heading text-xl font-semibold">Agent Comms</h1>
          {unreadCount > 0 && (
            <Badge variant="default" className="text-xs">
              {unreadCount} unread
            </Badge>
          )}
        </div>
        <Dialog open={composeOpen} onOpenChange={setComposeOpen}>
          <DialogTrigger
            render={
              <Button size="sm" className="gap-1.5">
                <Send className="size-3.5" />
                Compose
              </Button>
            }
          />
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Send Agent Message</DialogTitle>
              <DialogDescription>
                Send a message between agents (task, escalation, or notification).
              </DialogDescription>
            </DialogHeader>
            <div className="grid gap-3">
              <div className="grid gap-1.5">
                <label className="text-sm font-medium" htmlFor="mail-from">
                  From
                </label>
                <Input
                  id="mail-from"
                  value={composeFrom}
                  onChange={(e) => setComposeFrom(e.target.value)}
                  placeholder="sender@agents"
                />
              </div>
              <div className="grid gap-1.5">
                <label className="text-sm font-medium" htmlFor="mail-to">
                  To
                </label>
                <Input
                  id="mail-to"
                  value={composeTo}
                  onChange={(e) => setComposeTo(e.target.value)}
                  placeholder="recipient@agents"
                />
              </div>
              <div className="grid gap-1.5">
                <label className="text-sm font-medium" htmlFor="mail-subject">
                  Subject
                </label>
                <Input
                  id="mail-subject"
                  value={composeSubject}
                  onChange={(e) => setComposeSubject(e.target.value)}
                  placeholder="Subject line"
                />
              </div>
              <div className="grid gap-1.5">
                <label className="text-sm font-medium" htmlFor="mail-body">
                  Body
                </label>
                <Textarea
                  id="mail-body"
                  value={composeBody}
                  onChange={(e) => setComposeBody(e.target.value)}
                  placeholder="Message body..."
                  rows={4}
                />
              </div>
              <div className="grid gap-1.5">
                <label className="text-sm font-medium">Priority</label>
                <Select value={composePriority} onValueChange={(v) => setComposePriority(v ?? "normal")}>
                  <SelectTrigger className="w-full">
                    <SelectValue placeholder="Priority" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="low">Low</SelectItem>
                    <SelectItem value="normal">Normal</SelectItem>
                    <SelectItem value="high">High</SelectItem>
                    <SelectItem value="urgent">Urgent</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
            <DialogFooter>
              <DialogClose render={<Button variant="outline" />}>
                Cancel
              </DialogClose>
              <Button
                onClick={handleSend}
                disabled={
                  sendMutation.isPending ||
                  !composeFrom ||
                  !composeTo ||
                  !composeSubject
                }
              >
                {sendMutation.isPending ? "Sending..." : "Send"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      <div className="grid gap-4 md:grid-cols-[1fr_1.5fr]">
        <div className="overflow-hidden rounded-xl border border-border bg-card">
          <Tabs defaultValue="inbox">
            <TabsList className="w-full">
              <TabsTrigger value="inbox">
                Inbox
                {unreadCount > 0 && (
                  <span className="ml-1.5 inline-flex size-4 items-center justify-center rounded-full bg-primary text-[10px] font-bold text-primary-foreground">
                    {unreadCount}
                  </span>
                )}
              </TabsTrigger>
              <TabsTrigger value="sent">Sent</TabsTrigger>
              <TabsTrigger value="queue">Queue</TabsTrigger>
            </TabsList>
            <TabsContent value="inbox">
              <MessageList
                messages={inboxQuery.data ?? []}
                isLoading={inboxQuery.isLoading}
                onSelect={setSelected}
                selectedId={selected?.id ?? null}
              />
            </TabsContent>
            <TabsContent value="sent">
              <MessageList
                messages={sentQuery.data ?? []}
                isLoading={sentQuery.isLoading}
                onSelect={setSelected}
                selectedId={selected?.id ?? null}
              />
            </TabsContent>
            <TabsContent value="queue">
              <MessageList
                messages={queueQuery.data ?? []}
                isLoading={queueQuery.isLoading}
                onSelect={setSelected}
                selectedId={selected?.id ?? null}
              />
            </TabsContent>
          </Tabs>
        </div>

        <div className="overflow-hidden rounded-xl border border-border bg-card">
          <MessageDetail
            message={selected}
            onMarkRead={(id) => markReadMutation.mutate(id)}
            markReadPending={markReadMutation.isPending}
          />
        </div>
      </div>
    </div>
  )
}
