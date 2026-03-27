import { Button } from "@vibetown/ui/components/button"
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@vibetown/ui/components/tooltip"
import { LogOut } from "lucide-react"
import { useLogout, useCurrentUser } from "@vibetown/web-core/hooks/use-auth"

interface UserMenuProps {
  collapsed?: boolean
}

export function UserMenu({ collapsed = false }: UserMenuProps) {
  const { data } = useCurrentUser()
  const logoutMutation = useLogout()

  const user = data?.user
  if (!user) return null

  const displayName = user.display_name || user.email.split("@")[0]
  const initials = displayName
    .split(" ")
    .map((s) => s[0])
    .join("")
    .toUpperCase()
    .slice(0, 2)

  if (collapsed) {
    return (
      <Tooltip>
        <TooltipTrigger
          render={
            <button
              type="button"
              onClick={() => logoutMutation.mutate()}
              className="flex w-full items-center justify-center rounded-lg px-0 py-2 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
            >
              <div className="flex size-6 items-center justify-center rounded-full bg-primary text-[10px] font-medium text-primary-foreground">
                {initials}
              </div>
            </button>
          }
        />
        <TooltipContent side="right">
          {displayName} - Sign out
        </TooltipContent>
      </Tooltip>
    )
  }

  return (
    <div className="flex items-center gap-2 rounded-lg px-3 py-2">
      <div className="flex size-6 shrink-0 items-center justify-center rounded-full bg-primary text-[10px] font-medium text-primary-foreground">
        {initials}
      </div>
      <div className="min-w-0 flex-1">
        <p className="truncate text-xs font-medium text-foreground">
          {displayName}
        </p>
        <p className="truncate text-[10px] text-muted-foreground">
          {user.email}
        </p>
      </div>
      <Button
        variant="ghost"
        size="icon-xs"
        onClick={() => logoutMutation.mutate()}
        className="shrink-0"
        title="Sign out"
      >
        <LogOut className="size-3" />
      </Button>
    </div>
  )
}
