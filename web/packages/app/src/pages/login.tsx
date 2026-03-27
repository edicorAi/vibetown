import * as React from "react"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@vibetown/ui/components/card"
import { Button } from "@vibetown/ui/components/button"
import { Input } from "@vibetown/ui/components/input"
import { Separator } from "@vibetown/ui/components/separator"
import {
  useAuthStatus,
  useLogin,
  useLdapLogin,
} from "@vibetown/web-core/hooks/use-auth"

const API_BASE = import.meta.env.VITE_API_BASE ?? "/api"

export function LoginPage({ onSuccess }: { onSuccess: () => void }) {
  const { data: authStatus } = useAuthStatus()
  const loginMutation = useLogin()
  const ldapLoginMutation = useLdapLogin()

  const [mode, setMode] = React.useState<"local" | "ldap">("local")
  const [email, setEmail] = React.useState("")
  const [username, setUsername] = React.useState("")
  const [password, setPassword] = React.useState("")
  const [error, setError] = React.useState("")

  const providers = authStatus?.providers ?? []
  const hasLocal = providers.some((p) => p.provider_type === "local")
  const hasLdap = providers.some((p) => p.provider_type === "ldap")
  const oidcProviders = providers.filter((p) => p.provider_type === "oidc")

  const handleLocalSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError("")
    try {
      await loginMutation.mutateAsync({ email, password })
      onSuccess()
    } catch {
      setError("Invalid email or password")
    }
  }

  const handleLdapSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError("")
    try {
      await ldapLoginMutation.mutateAsync({ username, password })
      onSuccess()
    } catch {
      setError("Invalid username or password")
    }
  }

  const handleOidcLogin = (providerName: string) => {
    window.location.href = `${API_BASE}/auth/oidc/${encodeURIComponent(providerName)}/login`
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-background p-4">
      <Card className="w-full max-w-md">
        <CardHeader className="text-center">
          <CardTitle className="text-2xl">vibetown</CardTitle>
          <CardDescription>Sign in to continue</CardDescription>
          {!authStatus?.has_users && (
            <p className="text-sm text-amber-500 mt-2">
              No users configured. Contact your administrator.
            </p>
          )}
        </CardHeader>
        <CardContent className="space-y-4">
          {/* OIDC Providers */}
          {oidcProviders.map((provider) => (
            <Button
              key={provider.name}
              variant="outline"
              className="w-full"
              onClick={() => handleOidcLogin(provider.name)}
            >
              Sign in with {provider.name}
            </Button>
          ))}

          {oidcProviders.length > 0 && (hasLocal || hasLdap) && (
            <div className="relative">
              <div className="absolute inset-0 flex items-center">
                <Separator className="w-full" />
              </div>
              <div className="relative flex justify-center text-xs uppercase">
                <span className="bg-background px-2 text-muted-foreground">
                  or
                </span>
              </div>
            </div>
          )}

          {/* Local/LDAP toggle */}
          {hasLocal && hasLdap && (
            <div className="flex gap-2">
              <Button
                variant={mode === "local" ? "default" : "outline"}
                size="sm"
                className="flex-1"
                onClick={() => setMode("local")}
              >
                Email
              </Button>
              <Button
                variant={mode === "ldap" ? "default" : "outline"}
                size="sm"
                className="flex-1"
                onClick={() => setMode("ldap")}
              >
                LDAP
              </Button>
            </div>
          )}

          {/* Local login form */}
          {hasLocal && mode === "local" && (
            <form onSubmit={handleLocalSubmit} className="space-y-3">
              <Input
                type="email"
                placeholder="Email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                autoFocus
              />
              <Input
                type="password"
                placeholder="Password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
              />
              <Button
                type="submit"
                className="w-full"
                disabled={loginMutation.isPending}
              >
                {loginMutation.isPending ? "Signing in..." : "Sign in"}
              </Button>
            </form>
          )}

          {/* LDAP login form */}
          {hasLdap && (mode === "ldap" || !hasLocal) && (
            <form onSubmit={handleLdapSubmit} className="space-y-3">
              <Input
                type="text"
                placeholder="Username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                required
                autoFocus
              />
              <Input
                type="password"
                placeholder="Password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
              />
              <Button
                type="submit"
                className="w-full"
                disabled={ldapLoginMutation.isPending}
              >
                {ldapLoginMutation.isPending ? "Signing in..." : "Sign in with LDAP"}
              </Button>
            </form>
          )}

          {error && (
            <p className="text-sm text-destructive text-center">{error}</p>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
