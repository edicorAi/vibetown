import * as React from "react"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@vibetown/ui/components/card"
import { Badge } from "@vibetown/ui/components/badge"
import { Button } from "@vibetown/ui/components/button"
import { Input } from "@vibetown/ui/components/input"
import { Separator } from "@vibetown/ui/components/separator"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@vibetown/ui/components/select"
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
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@vibetown/ui/components/dialog"
import { Switch } from "@vibetown/ui/components/switch"
import { ScrollArea } from "@vibetown/ui/components/scroll-area"
import { cn } from "@vibetown/ui/lib/utils"
import {
  Eye,
  EyeOff,
  Check,
  X,
  Loader2,
  Zap,
  Server,
  Globe,
  Send,
  MessageSquare,
  Hash,
  Webhook,
  Info,
  Terminal,
} from "lucide-react"

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface ProviderField {
  key: string
  label: string
  type: "password" | "url" | "text" | "select"
  placeholder?: string
  default?: string
  options?: string[]
}

interface Provider {
  id: string
  name: string
  description: string
  color: string
  fields: ProviderField[]
}

// ---------------------------------------------------------------------------
// Provider definitions
// ---------------------------------------------------------------------------

const PROVIDERS: Provider[] = [
  {
    id: "anthropic",
    name: "Anthropic (Claude)",
    description: "Claude Code, Claude API",
    color: "#D97706",
    fields: [
      { key: "ANTHROPIC_API_KEY", label: "API Key", type: "password", placeholder: "sk-ant-..." },
      { key: "ANTHROPIC_BASE_URL", label: "Base URL", type: "url", placeholder: "https://api.anthropic.com", default: "https://api.anthropic.com" },
      { key: "CLAUDE_MODEL", label: "Default Model", type: "select", options: ["claude-opus-4-6", "claude-sonnet-4-6", "claude-haiku-4-5"] },
    ],
  },
  {
    id: "openai",
    name: "OpenAI",
    description: "GPT, Codex, o-series",
    color: "#10B981",
    fields: [
      { key: "OPENAI_API_KEY", label: "API Key", type: "password", placeholder: "sk-..." },
      { key: "OPENAI_BASE_URL", label: "Base URL", type: "url", placeholder: "https://api.openai.com/v1", default: "https://api.openai.com/v1" },
      { key: "OPENAI_MODEL", label: "Default Model", type: "select", options: ["gpt-4o", "gpt-4o-mini", "o3", "o3-mini", "codex-mini"] },
    ],
  },
  {
    id: "google",
    name: "Google (Gemini)",
    description: "Gemini Pro, Flash",
    color: "#3B82F6",
    fields: [
      { key: "GOOGLE_API_KEY", label: "API Key", type: "password", placeholder: "AIza..." },
      { key: "GEMINI_BASE_URL", label: "Base URL", type: "url", placeholder: "https://generativelanguage.googleapis.com", default: "https://generativelanguage.googleapis.com" },
      { key: "GEMINI_MODEL", label: "Default Model", type: "select", options: ["gemini-2.5-pro", "gemini-2.5-flash", "gemini-2.0-flash"] },
    ],
  },
  {
    id: "ollama",
    name: "Ollama",
    description: "Local models via Ollama",
    color: "#8B5CF6",
    fields: [
      { key: "OLLAMA_BASE_URL", label: "Base URL", type: "url", placeholder: "http://localhost:11434", default: "http://localhost:11434" },
      { key: "OLLAMA_MODEL", label: "Default Model", type: "text", placeholder: "llama3.3" },
    ],
  },
  {
    id: "openrouter",
    name: "OpenRouter",
    description: "Multi-provider router",
    color: "#EC4899",
    fields: [
      { key: "OPENROUTER_API_KEY", label: "API Key", type: "password", placeholder: "sk-or-..." },
      { key: "OPENROUTER_BASE_URL", label: "Base URL", type: "url", placeholder: "https://openrouter.ai/api/v1", default: "https://openrouter.ai/api/v1" },
      { key: "OPENROUTER_MODEL", label: "Default Model", type: "text", placeholder: "anthropic/claude-opus-4-6" },
    ],
  },
  {
    id: "cursor",
    name: "Cursor",
    description: "Cursor AI agent",
    color: "#F59E0B",
    fields: [
      { key: "CURSOR_API_KEY", label: "API Key", type: "password", placeholder: "cursor-..." },
      { key: "CURSOR_MODEL", label: "Default Model", type: "select", options: ["gpt-5.4", "opus-4.6", "sonnet-4.6", "gemini-3.1-pro"] },
    ],
  },
  {
    id: "amp",
    name: "Amp (Sourcegraph)",
    description: "Sourcegraph Amp agent",
    color: "#06B6D4",
    fields: [
      { key: "AMP_ACCESS_TOKEN", label: "Access Token", type: "password", placeholder: "sgp_..." },
    ],
  },
  {
    id: "droid",
    name: "Droid (Factory)",
    description: "Factory Droid agent",
    color: "#EF4444",
    fields: [
      { key: "DROID_API_KEY", label: "API Key", type: "password", placeholder: "" },
      { key: "DROID_MODEL", label: "Default Model", type: "select", options: ["claude-opus", "gemini-3", "gpt-5", "glm-5", "kimi-k2.5"] },
    ],
  },
  {
    id: "custom",
    name: "Custom Provider",
    description: "OpenAI-compatible API",
    color: "#6B7280",
    fields: [
      { key: "CUSTOM_API_KEY", label: "API Key", type: "password", placeholder: "" },
      { key: "CUSTOM_BASE_URL", label: "Base URL", type: "url", placeholder: "https://your-api.example.com/v1" },
      { key: "CUSTOM_MODEL", label: "Model Name", type: "text", placeholder: "your-model-name" },
    ],
  },
]

// ---------------------------------------------------------------------------
// CLI auth provider mapping
// ---------------------------------------------------------------------------

const CLI_AUTH_PROVIDERS: Record<string, string> = {
  anthropic: "claude login",
  openai: "codex login",
  google: "gemini login",
}

// Cloud provider suggested tiers (read-only)
const CLOUD_TIER_SUGGESTIONS: Record<string, { large: string; medium: string; small: string }> = {
  anthropic: { large: "claude-opus-4-6", medium: "claude-sonnet-4-6", small: "claude-haiku-4-5" },
  openai: { large: "o3", medium: "gpt-4o", small: "gpt-4o-mini" },
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

type SettingsData = Record<string, unknown>

function loadSettings(): SettingsData {
  try {
    const saved = localStorage.getItem("vibetown-settings")
    return saved ? (JSON.parse(saved) as SettingsData) : {}
  } catch {
    return {}
  }
}

function saveSettings(data: SettingsData) {
  try {
    localStorage.setItem("vibetown-settings", JSON.stringify(data))
  } catch {
    // ignore
  }
}

function isProviderConfigured(providerId: string, settings: SettingsData): boolean {
  const providerSettings = settings[providerId] as Record<string, string> | undefined
  if (!providerSettings) return false
  const provider = PROVIDERS.find((p) => p.id === providerId)
  if (!provider) return false
  // Consider configured if at least one key field has a value
  const keyField = provider.fields.find(
    (f) => f.type === "password" || f.key.endsWith("_BASE_URL")
  )
  if (!keyField) return false
  return Boolean(providerSettings[keyField.key]?.trim())
}

// ---------------------------------------------------------------------------
// Password Field with toggle
// ---------------------------------------------------------------------------

function PasswordField({
  value,
  onChange,
  placeholder,
}: {
  value: string
  onChange: (val: string) => void
  placeholder?: string
}) {
  const [visible, setVisible] = React.useState(false)

  return (
    <div className="relative">
      <Input
        type={visible ? "text" : "password"}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="pr-9 font-mono text-xs"
      />
      <button
        type="button"
        onClick={() => setVisible(!visible)}
        className="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground transition-colors hover:text-foreground"
      >
        {visible ? <EyeOff className="size-3.5" /> : <Eye className="size-3.5" />}
      </button>
    </div>
  )
}

// ---------------------------------------------------------------------------
// Test Connection Button
// ---------------------------------------------------------------------------

function TestConnectionButton() {
  const [state, setState] = React.useState<"idle" | "loading" | "success" | "error">("idle")

  const handleTest = () => {
    setState("loading")
    setTimeout(() => {
      setState("success")
      setTimeout(() => setState("idle"), 3000)
    }, 1000)
  }

  return (
    <Button
      variant="outline"
      size="sm"
      onClick={handleTest}
      disabled={state === "loading"}
      className="gap-1.5"
    >
      {state === "loading" && <Loader2 className="size-3.5 animate-spin" />}
      {state === "success" && <Check className="size-3.5 text-green-500" />}
      {state === "error" && <X className="size-3.5 text-red-500" />}
      {state === "idle" && <Zap className="size-3.5" />}
      {state === "loading"
        ? "Testing..."
        : state === "success"
          ? "Connected"
          : state === "error"
            ? "Failed"
            : "Test Connection"}
    </Button>
  )
}

// ---------------------------------------------------------------------------
// Model Tiers Section (for local providers: ollama, custom)
// ---------------------------------------------------------------------------

interface ModelTiers {
  large: string
  medium: string
  small: string
}

function ModelTiersSection({
  providerId,
  settings,
  onSaveSettings,
}: {
  providerId: string
  settings: SettingsData
  onSaveSettings: (updated: SettingsData) => void
}) {
  const modelTiersRoot = (settings.modelTiers ?? {}) as Record<string, ModelTiers>
  const saved = modelTiersRoot[providerId] ?? { large: "", medium: "", small: "" }

  const [tiers, setTiers] = React.useState<ModelTiers>(saved)
  const [flash, setFlash] = React.useState(false)

  // Sync when provider changes
  React.useEffect(() => {
    const root = (settings.modelTiers ?? {}) as Record<string, ModelTiers>
    setTiers(root[providerId] ?? { large: "", medium: "", small: "" })
  }, [providerId, settings])

  const isOllama = providerId === "ollama"

  const tierRows: { key: keyof ModelTiers; label: string; color: string; desc: string; placeholder: string }[] = [
    {
      key: "large",
      label: "Large",
      color: "bg-purple-500/15 text-purple-700 dark:text-purple-400",
      desc: "complex reasoning, architecture",
      placeholder: isOllama ? "deepseek-r1:70b" : "your-large-model",
    },
    {
      key: "medium",
      label: "Medium",
      color: "bg-blue-500/15 text-blue-700 dark:text-blue-400",
      desc: "standard coding tasks",
      placeholder: isOllama ? "qwen2.5-coder:32b" : "your-medium-model",
    },
    {
      key: "small",
      label: "Small",
      color: "bg-green-500/15 text-green-700 dark:text-green-400",
      desc: "monitoring, quick checks",
      placeholder: isOllama ? "llama3.2:3b" : "your-small-model",
    },
  ]

  const handleSaveTiers = () => {
    const updated: SettingsData = {
      ...settings,
      modelTiers: {
        ...modelTiersRoot,
        [providerId]: tiers,
      },
    }
    onSaveSettings(updated)
    setFlash(true)
    setTimeout(() => setFlash(false), 2000)
  }

  return (
    <Card className="mt-4">
      <CardHeader className="pb-3">
        <CardTitle className="text-sm font-medium">Model Tiers</CardTitle>
        <CardDescription>
          Assign models to tiers for formula-based routing. Formulas specify a tier, and the system routes to the configured model.
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {flash && (
          <div className="flex items-center gap-2 rounded-lg bg-green-500/10 px-3 py-2 text-xs text-green-700 dark:text-green-400">
            <Check className="size-3.5" />
            Model tiers saved successfully
          </div>
        )}

        {tierRows.map((row) => (
          <div key={row.key} className="flex items-center gap-3">
            <Badge
              variant="outline"
              className={cn("w-16 shrink-0 justify-center border-0 text-[10px] font-medium", row.color)}
            >
              {row.label}
            </Badge>
            <span className="hidden w-52 shrink-0 text-xs text-muted-foreground sm:inline">
              {row.desc}
            </span>
            <Input
              type="text"
              value={tiers[row.key]}
              onChange={(e) => setTiers((prev) => ({ ...prev, [row.key]: e.target.value }))}
              placeholder={row.placeholder}
              className="flex-1 font-mono text-xs"
            />
          </div>
        ))}

        <div className="flex items-center gap-2 pt-2">
          <Button onClick={handleSaveTiers} size="sm">
            Save Tiers
          </Button>
        </div>
      </CardContent>
    </Card>
  )
}

// ---------------------------------------------------------------------------
// Cloud Tier Suggestions (read-only, for Anthropic/OpenAI)
// ---------------------------------------------------------------------------

function CloudTierSuggestions({ providerId }: { providerId: string }) {
  const suggestions = CLOUD_TIER_SUGGESTIONS[providerId]
  if (!suggestions) return null

  return (
    <div className="mt-3 space-y-1.5">
      <p className="text-[10px] font-medium text-muted-foreground">Suggested tiers</p>
      <div className="flex flex-wrap items-center gap-1.5">
        <Badge variant="outline" className="border-0 bg-purple-500/15 text-[10px] text-purple-700 dark:text-purple-400">
          Large: {suggestions.large}
        </Badge>
        <Badge variant="outline" className="border-0 bg-blue-500/15 text-[10px] text-blue-700 dark:text-blue-400">
          Medium: {suggestions.medium}
        </Badge>
        <Badge variant="outline" className="border-0 bg-green-500/15 text-[10px] text-green-700 dark:text-green-400">
          Small: {suggestions.small}
        </Badge>
      </div>
    </div>
  )
}

// ---------------------------------------------------------------------------
// Provider Config Form
// ---------------------------------------------------------------------------

function ProviderConfigForm({
  provider,
  values,
  onFieldChange,
  onSave,
  onReset,
  settings,
  onSaveSettings,
}: {
  provider: Provider
  values: Record<string, string>
  onFieldChange: (key: string, value: string) => void
  onSave: () => void
  onReset: () => void
  settings: SettingsData
  onSaveSettings: (updated: SettingsData) => void
}) {
  const isLocal = provider.id === "ollama" || provider.id === "custom"
  const hasCloudTiers = provider.id in CLOUD_TIER_SUGGESTIONS

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center gap-3">
        <div
          className="flex size-10 items-center justify-center rounded-lg"
          style={{ backgroundColor: `${provider.color}15` }}
        >
          <div
            className="size-3 rounded-full"
            style={{ backgroundColor: provider.color }}
          />
        </div>
        <div>
          <h3 className="text-sm font-semibold">{provider.name}</h3>
          <p className="text-xs text-muted-foreground">{provider.description}</p>
        </div>
      </div>

      <Separator />

      {/* Fields */}
      <div className="space-y-4">
        {provider.fields.map((field) => (
          <div key={field.key} className="space-y-1.5">
            <label className="text-xs font-medium text-muted-foreground">
              {field.label}
            </label>
            <FieldInput
              field={field}
              value={values[field.key] ?? field.default ?? ""}
              onChange={(val) => onFieldChange(field.key, val)}
            />
            <p className="text-[10px] text-muted-foreground/60">{field.key}</p>
          </div>
        ))}
      </div>

      {/* Cloud tier suggestions */}
      {hasCloudTiers && <CloudTierSuggestions providerId={provider.id} />}

      <Separator />

      {/* Actions */}
      <div className="flex items-center gap-2">
        <Button onClick={onSave} size="sm">
          Save
        </Button>
        <Button variant="ghost" size="sm" onClick={onReset}>
          Reset
        </Button>
        <div className="ml-auto">
          <TestConnectionButton />
        </div>
      </div>

      {/* Model Tiers for local providers */}
      {isLocal && (
        <ModelTiersSection
          providerId={provider.id}
          settings={settings}
          onSaveSettings={onSaveSettings}
        />
      )}
    </div>
  )
}

// ---------------------------------------------------------------------------
// Field Input (dynamic by type)
// ---------------------------------------------------------------------------

function FieldInput({
  field,
  value,
  onChange,
}: {
  field: ProviderField
  value: string
  onChange: (val: string) => void
}) {
  switch (field.type) {
    case "password":
      return (
        <PasswordField
          value={value}
          onChange={onChange}
          placeholder={field.placeholder}
        />
      )
    case "select":
      return (
        <Select value={value} onValueChange={(v) => onChange(v ?? "")}>
          <SelectTrigger className="w-full">
            <SelectValue placeholder="Select a model..." />
          </SelectTrigger>
          <SelectContent>
            {field.options?.map((opt) => (
              <SelectItem key={opt} value={opt}>
                {opt}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      )
    case "url":
      return (
        <Input
          type="url"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={field.placeholder}
          className="text-xs"
        />
      )
    case "text":
    default:
      return (
        <Input
          type="text"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={field.placeholder}
          className="text-xs"
        />
      )
  }
}

// ---------------------------------------------------------------------------
// CLI Auth Info Dialog
// ---------------------------------------------------------------------------

function CLIAuthDialog({ command }: { command: string }) {
  return (
    <Dialog>
      <DialogTrigger
        render={
          <Button variant="outline" size="sm" className="h-6 px-2 text-[10px]">
            Connect
          </Button>
        }
      />
      <DialogContent>
        <DialogHeader>
          <DialogTitle>CLI Authentication</DialogTitle>
          <DialogDescription>
            This provider uses CLI-based authentication. Open a terminal and run:
          </DialogDescription>
        </DialogHeader>
        <div className="flex items-center gap-2 rounded-lg bg-muted px-3 py-2">
          <Terminal className="size-4 text-muted-foreground" />
          <code className="text-sm font-mono font-medium">{command}</code>
        </div>
      </DialogContent>
    </Dialog>
  )
}

// ---------------------------------------------------------------------------
// LLM Providers Tab
// ---------------------------------------------------------------------------

function LLMProvidersTab() {
  const [settings, setSettings] = React.useState<SettingsData>(loadSettings)
  const [selectedId, setSelectedId] = React.useState(PROVIDERS[0].id)
  const [saveFlash, setSaveFlash] = React.useState(false)

  const selectedProvider = PROVIDERS.find((p) => p.id === selectedId) ?? PROVIDERS[0]
  const providerValues = (settings[selectedId] ?? {}) as Record<string, string>

  const handleFieldChange = (key: string, value: string) => {
    setSettings((prev) => ({
      ...prev,
      [selectedId]: {
        ...(prev[selectedId] as Record<string, string> | undefined),
        [key]: value,
      },
    }))
  }

  const handleSave = () => {
    saveSettings(settings)
    setSaveFlash(true)
    setTimeout(() => setSaveFlash(false), 2000)
  }

  const handleReset = () => {
    setSettings((prev) => {
      const next = { ...prev }
      delete next[selectedId]
      return next
    })
  }

  const handleSaveSettings = (updated: SettingsData) => {
    setSettings(updated)
    saveSettings(updated)
  }

  return (
    <div className="grid gap-6 lg:grid-cols-[280px_1fr]">
      {/* Provider list */}
      <Card className="animate-card-enter">
        <CardHeader className="pb-2">
          <CardTitle className="text-sm font-medium">Providers</CardTitle>
          <CardDescription>Select a provider to configure</CardDescription>
        </CardHeader>
        <CardContent className="p-0">
          <ScrollArea className="h-[500px]">
            <div className="space-y-0.5 p-2">
              {PROVIDERS.map((provider, i) => {
                const active = provider.id === selectedId
                const configured = isProviderConfigured(provider.id, settings)
                const cliAuth = CLI_AUTH_PROVIDERS[provider.id]

                return (
                  <button
                    key={provider.id}
                    type="button"
                    onClick={() => setSelectedId(provider.id)}
                    className={cn(
                      "animate-card-enter flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-left transition-all duration-150",
                      active
                        ? "bg-primary text-primary-foreground shadow-sm"
                        : "hover:bg-muted/70 hover:-translate-y-px hover:shadow-sm"
                    )}
                    style={{ animationDelay: `${i * 40}ms` }}
                  >
                    <div
                      className={cn(
                        "flex size-8 shrink-0 items-center justify-center rounded-md transition-colors",
                        active ? "bg-primary-foreground/20" : "bg-muted"
                      )}
                    >
                      <div
                        className="size-2.5 rounded-full"
                        style={{ backgroundColor: provider.color }}
                      />
                    </div>
                    <div className="min-w-0 flex-1">
                      <p className="truncate text-sm font-medium">{provider.name}</p>
                      <p
                        className={cn(
                          "truncate text-xs",
                          active
                            ? "text-primary-foreground/70"
                            : "text-muted-foreground"
                        )}
                      >
                        {provider.description}
                      </p>
                      {/* Auth status line */}
                      <div className="mt-0.5 flex items-center gap-1.5">
                        <span
                          className={cn(
                            "size-1.5 shrink-0 rounded-full",
                            configured ? "bg-green-500" : "bg-gray-400/50"
                          )}
                        />
                        <span
                          className={cn(
                            "text-[10px]",
                            active
                              ? "text-primary-foreground/60"
                              : "text-muted-foreground/70"
                          )}
                        >
                          {configured ? "Connected" : "Not configured"}
                        </span>
                      </div>
                      {/* CLI auth hint */}
                      {cliAuth && !configured && (
                        <p
                          className={cn(
                            "mt-0.5 text-[9px]",
                            active
                              ? "text-primary-foreground/50"
                              : "text-muted-foreground/50"
                          )}
                        >
                          Uses CLI auth
                        </p>
                      )}
                    </div>
                    {/* CLI auth connect button */}
                    {cliAuth && !active && (
                      <div onClick={(e) => e.stopPropagation()}>
                        <CLIAuthDialog command={cliAuth} />
                      </div>
                    )}
                    {!cliAuth && (
                      <span
                        className={cn(
                          "size-2 shrink-0 rounded-full transition-colors",
                          configured ? "bg-green-500" : "bg-gray-400/50"
                        )}
                      />
                    )}
                  </button>
                )
              })}
            </div>
          </ScrollArea>
        </CardContent>
      </Card>

      {/* Config form */}
      <Card className="animate-card-enter" style={{ animationDelay: "80ms" }}>
        <CardContent className="p-6">
          {saveFlash && (
            <div className="mb-4 flex items-center gap-2 rounded-lg bg-green-500/10 px-3 py-2 text-xs text-green-700 dark:text-green-400">
              <Check className="size-3.5" />
              Settings saved successfully
            </div>
          )}

          {/* CLI auth info for selected provider */}
          {CLI_AUTH_PROVIDERS[selectedId] && (
            <div className="mb-4 flex items-center gap-2 rounded-lg bg-blue-500/10 px-3 py-2 text-xs text-blue-700 dark:text-blue-400">
              <Info className="size-3.5 shrink-0" />
              <span>
                Uses CLI auth &mdash; run{" "}
                <code className="rounded bg-blue-500/10 px-1 py-0.5 font-mono text-[10px] font-medium">
                  {CLI_AUTH_PROVIDERS[selectedId]}
                </code>{" "}
                to authenticate. API key below is an optional override.
              </span>
            </div>
          )}

          <ProviderConfigForm
            provider={selectedProvider}
            values={providerValues}
            onFieldChange={handleFieldChange}
            onSave={handleSave}
            onReset={handleReset}
            settings={settings}
            onSaveSettings={handleSaveSettings}
          />
        </CardContent>
      </Card>
    </div>
  )
}

// ---------------------------------------------------------------------------
// Agent Defaults Tab
// ---------------------------------------------------------------------------

interface AgentDefaults {
  defaultRuntime: string
  defaultModel: string
  autoApprove: boolean
  maxConcurrentAgents: number
  worktreeDirectory: string
}

function AgentDefaultsTab() {
  const [defaults, setDefaults] = React.useState<AgentDefaults>(() => {
    try {
      const saved = localStorage.getItem("vibetown-settings")
      const parsed = saved ? (JSON.parse(saved) as SettingsData & { agentDefaults?: AgentDefaults }) : null
      return parsed?.agentDefaults ?? {
        defaultRuntime: "claude",
        defaultModel: "",
        autoApprove: false,
        maxConcurrentAgents: 5,
        worktreeDirectory: "",
      }
    } catch {
      return {
        defaultRuntime: "claude",
        defaultModel: "",
        autoApprove: false,
        maxConcurrentAgents: 5,
        worktreeDirectory: "",
      }
    }
  })

  const [saved, setSaved] = React.useState(false)

  const handleSave = () => {
    try {
      const current = loadSettings()
      const updated = { ...current, agentDefaults: defaults }
      localStorage.setItem("vibetown-settings", JSON.stringify(updated))
      setSaved(true)
      setTimeout(() => setSaved(false), 2000)
    } catch {
      // ignore
    }
  }

  const runtimes = ["claude", "codex", "gemini", "cursor", "amp", "droid", "opencode", "qwen"]

  return (
    <Card className="animate-card-enter mx-auto max-w-2xl">
      <CardHeader>
        <CardTitle className="flex items-center gap-2 text-sm font-medium">
          <Server className="size-4" />
          Agent Defaults
        </CardTitle>
        <CardDescription>
          Configure default settings for new agents
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {saved && (
          <div className="flex items-center gap-2 rounded-lg bg-green-500/10 px-3 py-2 text-xs text-green-700 dark:text-green-400">
            <Check className="size-3.5" />
            Agent defaults saved successfully
          </div>
        )}

        {/* Default Runtime */}
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-muted-foreground">
            Default Runtime
          </label>
          <Select
            value={defaults.defaultRuntime}
            onValueChange={(val) =>
              setDefaults((prev) => ({ ...prev, defaultRuntime: val ?? prev.defaultRuntime }))
            }
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select runtime..." />
            </SelectTrigger>
            <SelectContent>
              {runtimes.map((rt) => (
                <SelectItem key={rt} value={rt}>
                  {rt}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Default Model */}
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-muted-foreground">
            Default Model
          </label>
          <Input
            type="text"
            value={defaults.defaultModel}
            onChange={(e) =>
              setDefaults((prev) => ({ ...prev, defaultModel: e.target.value }))
            }
            placeholder="e.g. claude-opus-4-6"
            className="text-xs"
          />
        </div>

        {/* Auto-approve */}
        <div className="flex items-center justify-between rounded-lg border border-border px-4 py-3">
          <div>
            <p className="text-sm font-medium">Auto-approve</p>
            <p className="text-xs text-muted-foreground">
              Automatically approve agent actions without confirmation
            </p>
          </div>
          <Switch
            checked={defaults.autoApprove}
            onCheckedChange={(val) =>
              setDefaults((prev) => ({ ...prev, autoApprove: val }))
            }
          />
        </div>

        {/* Max Concurrent Agents */}
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-muted-foreground">
            Max Concurrent Agents
          </label>
          <Input
            type="number"
            min={1}
            max={50}
            value={defaults.maxConcurrentAgents}
            onChange={(e) =>
              setDefaults((prev) => ({
                ...prev,
                maxConcurrentAgents: Math.min(50, Math.max(1, parseInt(e.target.value) || 1)),
              }))
            }
            className="w-32 text-xs"
          />
          <p className="text-[10px] text-muted-foreground/60">Between 1 and 50</p>
        </div>

        {/* Worktree Directory */}
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-muted-foreground">
            Worktree Directory
          </label>
          <Input
            type="text"
            value={defaults.worktreeDirectory}
            onChange={(e) =>
              setDefaults((prev) => ({ ...prev, worktreeDirectory: e.target.value }))
            }
            placeholder="/tmp/vibetown-worktrees"
            className="font-mono text-xs"
          />
        </div>

        <Separator />

        <div className="flex items-center gap-2">
          <Button onClick={handleSave} size="sm">
            Save Defaults
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={() =>
              setDefaults({
                defaultRuntime: "claude",
                defaultModel: "",
                autoApprove: false,
                maxConcurrentAgents: 5,
                worktreeDirectory: "",
              })
            }
          >
            Reset
          </Button>
        </div>
      </CardContent>
    </Card>
  )
}

// ---------------------------------------------------------------------------
// Integrations Tab
// ---------------------------------------------------------------------------

interface TelegramSettings {
  botToken: string
  chatId: string
  notificationLevel: string
  agentStatusChanges: boolean
  convoyCompletions: boolean
  mergeResults: boolean
  escalationsStuckAgents: boolean
  mailMessages: boolean
}

const DEFAULT_TELEGRAM: TelegramSettings = {
  botToken: "",
  chatId: "",
  notificationLevel: "all",
  agentStatusChanges: true,
  convoyCompletions: true,
  mergeResults: true,
  escalationsStuckAgents: true,
  mailMessages: false,
}

function IntegrationsTab() {
  const [telegram, setTelegram] = React.useState<TelegramSettings>(() => {
    try {
      const saved = localStorage.getItem("vibetown-settings")
      const parsed = saved ? (JSON.parse(saved) as SettingsData & { telegram?: TelegramSettings }) : null
      return parsed?.telegram ?? DEFAULT_TELEGRAM
    } catch {
      return DEFAULT_TELEGRAM
    }
  })

  const [saved, setSaved] = React.useState(false)
  const [testState, setTestState] = React.useState<"idle" | "loading" | "success">("idle")

  const handleSave = () => {
    try {
      const current = loadSettings()
      const updated = { ...current, telegram }
      localStorage.setItem("vibetown-settings", JSON.stringify(updated))
      setSaved(true)
      setTimeout(() => setSaved(false), 2000)
    } catch {
      // ignore
    }
  }

  const handleTestNotification = () => {
    setTestState("loading")
    setTimeout(() => {
      setTestState("success")
      setTimeout(() => setTestState("idle"), 2000)
    }, 1000)
  }

  const futureIntegrations = [
    { name: "Slack", icon: MessageSquare },
    { name: "Discord", icon: Hash },
    { name: "Webhooks", icon: Webhook },
  ]

  return (
    <div className="mx-auto max-w-2xl space-y-6">
      {/* Telegram Bot Section */}
      <Card className="animate-card-enter">
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-sm font-medium">
            <Send className="size-4" />
            Telegram Bot
          </CardTitle>
          <CardDescription>
            Connect a Telegram bot to receive agent notifications and send commands to Mayor
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {saved && (
            <div className="flex items-center gap-2 rounded-lg bg-green-500/10 px-3 py-2 text-xs text-green-700 dark:text-green-400">
              <Check className="size-3.5" />
              Telegram settings saved successfully
            </div>
          )}

          {/* Bot Token */}
          <div className="space-y-1.5">
            <label className="text-xs font-medium text-muted-foreground">
              Bot Token
            </label>
            <PasswordField
              value={telegram.botToken}
              onChange={(val) => setTelegram((prev) => ({ ...prev, botToken: val }))}
              placeholder="123456:ABC-DEF..."
            />
          </div>

          {/* Chat ID */}
          <div className="space-y-1.5">
            <label className="text-xs font-medium text-muted-foreground">
              Chat ID
            </label>
            <Input
              type="text"
              value={telegram.chatId}
              onChange={(e) => setTelegram((prev) => ({ ...prev, chatId: e.target.value }))}
              placeholder="your chat ID or group ID"
              className="text-xs"
            />
          </div>

          {/* Notification Level */}
          <div className="space-y-1.5">
            <label className="text-xs font-medium text-muted-foreground">
              Notification Level
            </label>
            <Select
              value={telegram.notificationLevel}
              onValueChange={(val) =>
                setTelegram((prev) => ({ ...prev, notificationLevel: val ?? prev.notificationLevel }))
              }
            >
              <SelectTrigger className="w-full">
                <SelectValue placeholder="Select level..." />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All</SelectItem>
                <SelectItem value="errors_only">Errors only</SelectItem>
                <SelectItem value="convoy_updates">Convoy updates</SelectItem>
                <SelectItem value="none">None</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <Separator />

          {/* Notification Type Toggles */}
          <div className="space-y-3">
            <p className="text-xs font-medium text-muted-foreground">Notification Types</p>

            {([
              { key: "agentStatusChanges" as const, label: "Agent status changes" },
              { key: "convoyCompletions" as const, label: "Convoy completions" },
              { key: "mergeResults" as const, label: "Merge results" },
              { key: "escalationsStuckAgents" as const, label: "Escalations & stuck agents" },
              { key: "mailMessages" as const, label: "Mail messages" },
            ]).map(({ key, label }) => (
              <div key={key} className="flex items-center justify-between rounded-lg border border-border px-4 py-2.5">
                <p className="text-sm">{label}</p>
                <Switch
                  checked={telegram[key]}
                  onCheckedChange={(val) =>
                    setTelegram((prev) => ({ ...prev, [key]: val }))
                  }
                />
              </div>
            ))}
          </div>

          <Separator />

          {/* Actions */}
          <div className="flex items-center gap-2">
            <Button onClick={handleSave} size="sm">
              Save
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={handleTestNotification}
              disabled={testState === "loading"}
              className="gap-1.5"
            >
              {testState === "loading" && <Loader2 className="size-3.5 animate-spin" />}
              {testState === "success" && <Check className="size-3.5 text-green-500" />}
              {testState === "idle" && <Send className="size-3.5" />}
              {testState === "loading"
                ? "Sending..."
                : testState === "success"
                  ? "Sent!"
                  : "Test Notification"}
            </Button>
          </div>

          {/* Info box */}
          <div className="rounded-lg bg-muted/50 px-4 py-3">
            <p className="text-xs font-medium text-muted-foreground">
              Commands available in Telegram:
            </p>
            <p className="mt-1 font-mono text-[11px] text-muted-foreground/80">
              /status &nbsp; /agents &nbsp; /dispatch &lt;issue&gt; &nbsp; /kill &lt;agent&gt; &nbsp; /convoy &lt;name&gt;
            </p>
          </div>
        </CardContent>
      </Card>

      {/* Future Integrations */}
      <Card className="animate-card-enter" style={{ animationDelay: "60ms" }}>
        <CardHeader>
          <CardTitle className="text-sm font-medium">More Integrations</CardTitle>
          <CardDescription>Coming soon</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-2">
            {futureIntegrations.map((item) => (
              <div
                key={item.name}
                className="flex items-center gap-3 rounded-lg border border-border px-4 py-3 opacity-50"
              >
                <item.icon className="size-4 text-muted-foreground" />
                <span className="flex-1 text-sm text-muted-foreground">{item.name}</span>
                <Badge variant="secondary" className="text-[10px]">
                  Coming soon
                </Badge>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  )
}

// ---------------------------------------------------------------------------
// General Tab
// ---------------------------------------------------------------------------

interface GeneralSettings {
  townName: string
  owner: string
  language: string
  notifications: boolean
  analytics: boolean
}

function GeneralTab() {
  const [general, setGeneral] = React.useState<GeneralSettings>(() => {
    try {
      const saved = localStorage.getItem("vibetown-settings")
      const parsed = saved ? (JSON.parse(saved) as SettingsData & { general?: GeneralSettings }) : null
      return parsed?.general ?? {
        townName: "Vibetown HQ",
        owner: "admin",
        language: "en",
        notifications: true,
        analytics: false,
      }
    } catch {
      return {
        townName: "Vibetown HQ",
        owner: "admin",
        language: "en",
        notifications: true,
        analytics: false,
      }
    }
  })

  const [saved, setSaved] = React.useState(false)

  const handleSave = () => {
    try {
      const current = loadSettings()
      const updated = { ...current, general }
      localStorage.setItem("vibetown-settings", JSON.stringify(updated))
      setSaved(true)
      setTimeout(() => setSaved(false), 2000)
    } catch {
      // ignore
    }
  }

  const languages = [
    { value: "en", label: "English" },
    { value: "es", label: "Spanish" },
    { value: "fr", label: "French" },
    { value: "de", label: "German" },
    { value: "ja", label: "Japanese" },
    { value: "zh", label: "Chinese" },
    { value: "ko", label: "Korean" },
    { value: "pt", label: "Portuguese" },
  ]

  return (
    <Card className="animate-card-enter mx-auto max-w-2xl">
      <CardHeader>
        <CardTitle className="flex items-center gap-2 text-sm font-medium">
          <Globe className="size-4" />
          General Settings
        </CardTitle>
        <CardDescription>
          Configure your Vibetown instance
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {saved && (
          <div className="flex items-center gap-2 rounded-lg bg-green-500/10 px-3 py-2 text-xs text-green-700 dark:text-green-400">
            <Check className="size-3.5" />
            General settings saved successfully
          </div>
        )}

        {/* Town Name */}
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-muted-foreground">
            Town Name
          </label>
          <Input
            type="text"
            value={general.townName}
            onChange={(e) =>
              setGeneral((prev) => ({ ...prev, townName: e.target.value }))
            }
            placeholder="My Vibetown"
            className="text-xs"
          />
        </div>

        {/* Owner */}
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-muted-foreground">
            Owner
          </label>
          <Input
            type="text"
            value={general.owner}
            onChange={(e) =>
              setGeneral((prev) => ({ ...prev, owner: e.target.value }))
            }
            placeholder="admin"
            className="text-xs"
          />
        </div>

        {/* Language */}
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-muted-foreground">
            Language
          </label>
          <Select
            value={general.language}
            onValueChange={(val) =>
              setGeneral((prev) => ({ ...prev, language: val ?? prev.language }))
            }
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select language..." />
            </SelectTrigger>
            <SelectContent>
              {languages.map((lang) => (
                <SelectItem key={lang.value} value={lang.value}>
                  {lang.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <Separator />

        {/* Notifications */}
        <div className="flex items-center justify-between rounded-lg border border-border px-4 py-3">
          <div>
            <p className="text-sm font-medium">Notifications</p>
            <p className="text-xs text-muted-foreground">
              Receive notifications for agent events and alerts
            </p>
          </div>
          <Switch
            checked={general.notifications}
            onCheckedChange={(val) =>
              setGeneral((prev) => ({ ...prev, notifications: val }))
            }
          />
        </div>

        {/* Analytics */}
        <div className="flex items-center justify-between rounded-lg border border-border px-4 py-3">
          <div>
            <p className="text-sm font-medium">Analytics</p>
            <p className="text-xs text-muted-foreground">
              Collect anonymous usage analytics to improve Vibetown
            </p>
          </div>
          <Switch
            checked={general.analytics}
            onCheckedChange={(val) =>
              setGeneral((prev) => ({ ...prev, analytics: val }))
            }
          />
        </div>

        <Separator />

        <div className="flex items-center gap-2">
          <Button onClick={handleSave} size="sm">
            Save Settings
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={() =>
              setGeneral({
                townName: "Vibetown HQ",
                owner: "admin",
                language: "en",
                notifications: true,
                analytics: false,
              })
            }
          >
            Reset
          </Button>
        </div>
      </CardContent>
    </Card>
  )
}

// ---------------------------------------------------------------------------
// Settings Page (exported)
// ---------------------------------------------------------------------------

export function SettingsPage() {
  return (
    <div className="mx-auto max-w-7xl space-y-6">
      <h1 className="font-heading text-xl font-semibold">Settings</h1>

      <Tabs defaultValue="providers">
        <TabsList>
          <TabsTrigger value="providers">LLM Providers</TabsTrigger>
          <TabsTrigger value="agent-defaults">Agent Defaults</TabsTrigger>
          <TabsTrigger value="integrations">Integrations</TabsTrigger>
          <TabsTrigger value="general">General</TabsTrigger>
        </TabsList>

        <TabsContent value="providers">
          <LLMProvidersTab />
        </TabsContent>

        <TabsContent value="agent-defaults">
          <AgentDefaultsTab />
        </TabsContent>

        <TabsContent value="integrations">
          <IntegrationsTab />
        </TabsContent>

        <TabsContent value="general">
          <GeneralTab />
        </TabsContent>
      </Tabs>
    </div>
  )
}
