import * as React from "react"
import { cn } from "@vibetown/ui/lib/utils"
import { Button } from "@vibetown/ui/components/button"
import {
  LayoutDashboard,
  Bot,
  Mail,
  GitMerge,
  Menu,
  X,
  PanelLeftClose,
  PanelLeftOpen,
  Moon,
  Sun,
  Wifi,
  WifiOff,
  Settings,
  FolderGit2,
} from "lucide-react"
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@vibetown/ui/components/tooltip"

const NAV_ITEMS = [
  { path: "/orchestration", label: "Dashboard", icon: LayoutDashboard },
  { path: "/orchestration/agents", label: "Agents", icon: Bot },
  { path: "/orchestration/mail", label: "Agent Comms", icon: Mail },
  { path: "/orchestration/merge-queue", label: "Merge Queue", icon: GitMerge },
  { path: "/projects", label: "Projects", icon: FolderGit2 },
  { path: "/settings", label: "Settings", icon: Settings },
] as const

interface ShellProps {
  children: React.ReactNode
  currentPath: string
  onNavigate: (path: string) => void
}

function useDarkMode() {
  const [dark, setDark] = React.useState(() => {
    if (typeof window === "undefined") return false
    return document.documentElement.classList.contains("dark")
  })

  const toggle = React.useCallback(() => {
    setDark((prev) => {
      const next = !prev
      if (next) {
        document.documentElement.classList.add("dark")
      } else {
        document.documentElement.classList.remove("dark")
      }
      try {
        localStorage.setItem("vibetown-theme", next ? "dark" : "light")
      } catch {
        // ignore
      }
      return next
    })
  }, [])

  // Init from localStorage
  React.useEffect(() => {
    try {
      const saved = localStorage.getItem("vibetown-theme")
      if (saved === "dark") {
        document.documentElement.classList.add("dark")
        setDark(true)
      } else if (saved === "light") {
        document.documentElement.classList.remove("dark")
        setDark(false)
      } else if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
        document.documentElement.classList.add("dark")
        setDark(true)
      }
    } catch {
      // ignore
    }
  }, [])

  return { dark, toggle }
}

export function Shell({ children, currentPath, onNavigate }: ShellProps) {
  const [mobileMenuOpen, setMobileMenuOpen] = React.useState(false)
  const [collapsed, setCollapsed] = React.useState(false)
  const { dark, toggle: toggleDark } = useDarkMode()

  // Demo mode indicator
  const [connected] = React.useState(false)

  return (
    <TooltipProvider delay={200}>
      <div className="flex min-h-svh">
        {/* -------- Desktop sidebar -------- */}
        <aside
          className={cn(
            "hidden md:flex flex-col border-r border-border bg-card transition-all duration-200 ease-in-out",
            collapsed ? "w-14" : "w-60"
          )}
        >
          {/* Logo area */}
          <div
            className={cn(
              "flex h-14 items-center border-b border-border",
              collapsed ? "justify-center px-0" : "gap-2 px-3"
            )}
          >
            {collapsed ? (
              <Button
                variant="ghost"
                size="icon"
                onClick={() => setCollapsed(false)}
                className="shrink-0"
              >
                <PanelLeftOpen className="size-4" />
              </Button>
            ) : (
              <>
                <div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-primary text-primary-foreground">
                  <span className="text-sm font-bold">V</span>
                </div>
                <span className="font-heading text-sm font-semibold tracking-tight">
                  Vibetown
                </span>
                <Button
                  variant="ghost"
                  size="icon-xs"
                  onClick={() => setCollapsed(true)}
                  className="ml-auto shrink-0"
                >
                  <PanelLeftClose className="size-3.5" />
                </Button>
              </>
            )}
          </div>

          {/* Navigation */}
          <nav className="flex-1 space-y-1 p-2">
            {NAV_ITEMS.map((item) => {
              const active =
                currentPath === item.path ||
                (item.path !== "/orchestration" &&
                  currentPath.startsWith(item.path))

              const button = (
                <button
                  key={item.path}
                  type="button"
                  onClick={() => onNavigate(item.path)}
                  className={cn(
                    "nav-item-hover flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                    active
                      ? "bg-primary text-primary-foreground"
                      : "text-muted-foreground hover:bg-muted hover:text-foreground",
                    collapsed && "justify-center px-0"
                  )}
                >
                  <item.icon className="size-4 shrink-0" />
                  {!collapsed && <span>{item.label}</span>}
                </button>
              )

              if (collapsed) {
                return (
                  <Tooltip key={item.path}>
                    <TooltipTrigger render={button} />
                    <TooltipContent side="right">
                      {item.label}
                    </TooltipContent>
                  </Tooltip>
                )
              }

              return button
            })}
          </nav>

          {/* Sidebar footer */}
          <div className="border-t border-border p-2 space-y-1">
            {/* Connection status */}
            <div
              className={cn(
                "flex items-center gap-2 rounded-lg px-3 py-2 text-xs",
                collapsed && "justify-center px-0"
              )}
            >
              {connected ? (
                <Wifi className="size-3.5 text-green-500" />
              ) : (
                <WifiOff className="size-3.5 text-muted-foreground" />
              )}
              {!collapsed && (
                <span className="text-muted-foreground">
                  {connected ? "Connected" : "Demo Mode"}
                </span>
              )}
            </div>

            {/* Dark mode toggle */}
            {collapsed ? (
              <Tooltip>
                <TooltipTrigger
                  render={
                    <button
                      type="button"
                      onClick={toggleDark}
                      className="flex w-full items-center justify-center rounded-lg px-0 py-2 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
                    >
                      {dark ? (
                        <Sun className="size-4" />
                      ) : (
                        <Moon className="size-4" />
                      )}
                    </button>
                  }
                />
                <TooltipContent side="right">
                  {dark ? "Light mode" : "Dark mode"}
                </TooltipContent>
              </Tooltip>
            ) : (
              <button
                type="button"
                onClick={toggleDark}
                className="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-xs text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
              >
                {dark ? (
                  <Sun className="size-3.5" />
                ) : (
                  <Moon className="size-3.5" />
                )}
                <span>{dark ? "Light mode" : "Dark mode"}</span>
              </button>
            )}
          </div>
        </aside>

        {/* -------- Mobile overlay -------- */}
        {mobileMenuOpen && (
          <div
            className="fixed inset-0 z-40 bg-black/50 md:hidden"
            onClick={() => setMobileMenuOpen(false)}
          />
        )}
        <aside
          className={cn(
            "fixed inset-y-0 left-0 z-50 flex w-60 flex-col border-r border-border bg-card transition-transform duration-200 md:hidden",
            mobileMenuOpen ? "translate-x-0" : "-translate-x-full"
          )}
        >
          <div className="flex h-14 items-center gap-2 border-b border-border px-3">
            <div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-primary text-primary-foreground">
              <span className="text-sm font-bold">V</span>
            </div>
            <span className="font-heading text-sm font-semibold tracking-tight">
              Vibetown
            </span>
            <Button
              variant="ghost"
              size="icon-xs"
              onClick={() => setMobileMenuOpen(false)}
              className="ml-auto"
            >
              <X className="size-4" />
            </Button>
          </div>

          <nav className="flex-1 space-y-1 p-2">
            {NAV_ITEMS.map((item) => {
              const active =
                currentPath === item.path ||
                (item.path !== "/orchestration" &&
                  currentPath.startsWith(item.path))
              return (
                <button
                  key={item.path}
                  type="button"
                  onClick={() => {
                    onNavigate(item.path)
                    setMobileMenuOpen(false)
                  }}
                  className={cn(
                    "nav-item-hover flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                    active
                      ? "bg-primary text-primary-foreground"
                      : "text-muted-foreground hover:bg-muted hover:text-foreground"
                  )}
                >
                  <item.icon className="size-4" />
                  <span>{item.label}</span>
                </button>
              )
            })}
          </nav>

          <div className="border-t border-border p-2 space-y-1">
            <div className="flex items-center gap-2 rounded-lg px-3 py-2 text-xs">
              {connected ? (
                <Wifi className="size-3.5 text-green-500" />
              ) : (
                <WifiOff className="size-3.5 text-muted-foreground" />
              )}
              <span className="text-muted-foreground">
                {connected ? "Connected" : "Demo Mode"}
              </span>
            </div>
            <button
              type="button"
              onClick={toggleDark}
              className="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-xs text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
            >
              {dark ? (
                <Sun className="size-3.5" />
              ) : (
                <Moon className="size-3.5" />
              )}
              <span>{dark ? "Light mode" : "Dark mode"}</span>
            </button>
          </div>
        </aside>

        {/* -------- Main content -------- */}
        <div className="flex min-w-0 flex-1 flex-col">
          {/* Mobile top bar */}
          <header className="sticky top-0 z-30 flex h-14 items-center gap-3 border-b border-border bg-background/80 px-4 backdrop-blur-sm md:hidden">
            <button
              type="button"
              onClick={() => setMobileMenuOpen(true)}
            >
              <Menu className="size-5" />
            </button>
            <div className="flex size-7 items-center justify-center rounded-md bg-primary text-primary-foreground">
              <span className="text-xs font-bold">V</span>
            </div>
            <span className="font-heading text-sm font-semibold tracking-tight">
              Vibetown
            </span>
          </header>

          <main className="flex-1 p-4 md:p-6">{children}</main>
        </div>
      </div>
    </TooltipProvider>
  )
}
