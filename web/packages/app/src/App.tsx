import * as React from "react"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { Shell } from "@/components/shell.tsx"
import { DashboardPage } from "@/pages/dashboard.tsx"
import { AgentsPage } from "@/pages/agents.tsx"
import { MailPage } from "@/pages/mail.tsx"
import { MergeQueuePage } from "@/pages/merge-queue.tsx"
import { ConvoyDetailPage } from "@/pages/convoy-detail.tsx"
import { SettingsPage } from "@/pages/settings.tsx"
import { ProjectsPage } from "@/pages/projects.tsx"
import { LoginPage } from "@/pages/login.tsx"
import { useAuthStatus, useCurrentUser } from "@vibetown/web-core/hooks/use-auth"
// useAuthStatus and useCurrentUser are used inside AuthGuard below

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      staleTime: 5_000,
    },
  },
})

function useSimpleRouter() {
  const [path, setPath] = React.useState(() => {
    const hash = window.location.hash.slice(1)
    return hash || "/orchestration"
  })

  React.useEffect(() => {
    function onHashChange() {
      const hash = window.location.hash.slice(1)
      setPath(hash || "/orchestration")
    }
    window.addEventListener("hashchange", onHashChange)
    return () => window.removeEventListener("hashchange", onHashChange)
  }, [])

  const navigate = React.useCallback((to: string) => {
    window.location.hash = to
    setPath(to)
  }, [])

  return { path, navigate }
}

function Router({
  path,
  navigate,
}: {
  path: string
  navigate: (to: string) => void
}) {
  // Match convoy detail: /orchestration/convoys/:id
  const convoyMatch = path.match(/^\/orchestration\/convoys\/(.+)$/)

  if (convoyMatch) {
    return (
      <ConvoyDetailPage
        convoyId={convoyMatch[1]}
        onBack={() => navigate("/orchestration")}
      />
    )
  }

  switch (path) {
    case "/":
    case "/orchestration":
      return <DashboardPage />
    case "/orchestration/agents":
      return <AgentsPage />
    case "/orchestration/mail":
      return <MailPage />
    case "/orchestration/merge-queue":
      return <MergeQueuePage />
    case "/projects":
      return <ProjectsPage />
    case "/settings":
      return <SettingsPage />
    default:
      return (
        <div className="flex min-h-[50vh] items-center justify-center">
          <p className="text-sm text-muted-foreground">
            Page not found: {path}
          </p>
        </div>
      )
  }
}

function AuthGuard({
  children,
  path,
  navigate,
}: {
  children: React.ReactNode
  path: string
  navigate: (to: string) => void
}) {
  const { data: authStatus, isLoading: statusLoading, isError: statusError } = useAuthStatus()

  const authRequired = authStatus?.auth_required === true

  const {
    data: userData,
    isLoading: userLoading,
    isError: userError,
  } = useCurrentUser(authRequired)

  // Still loading auth status
  if (statusLoading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <div className="animate-pulse text-sm text-muted-foreground">
          Loading...
        </div>
      </div>
    )
  }

  // Auth not required or status check failed — render app directly
  if (!authRequired || statusError) {
    return <>{children}</>
  }

  // Auth required but on login page
  if (path === "/login") {
    return (
      <LoginPage
        onSuccess={() => {
          navigate("/orchestration")
        }}
      />
    )
  }

  // Auth required, still loading user
  if (userLoading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <div className="animate-pulse text-sm text-muted-foreground">
          Loading...
        </div>
      </div>
    )
  }

  // Auth required but no valid session — redirect to login
  if (userError || !userData?.user) {
    navigate("/login")
    return null
  }

  // Authenticated
  return <>{children}</>
}

export function App() {
  const { path, navigate } = useSimpleRouter()

  return (
    <QueryClientProvider client={queryClient}>
      <AuthGuard path={path} navigate={navigate}>
        <Shell currentPath={path} onNavigate={navigate}>
          <Router path={path} navigate={navigate} />
        </Shell>
      </AuthGuard>
    </QueryClientProvider>
  )
}
