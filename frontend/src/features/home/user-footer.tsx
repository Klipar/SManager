import { LogOut } from "lucide-react"

import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Button } from "@/components/ui/button"

import type { CurrentUser } from "./types"

type UserFooterProps = {
  user: CurrentUser
  isCollapsed: boolean
}

function UserFooter({ user, isCollapsed }: UserFooterProps) {
  return (
    <div className={isCollapsed ? "flex w-full items-center justify-center border-t border-white/5 px-1 pt-4" : "flex items-center gap-3 border-t border-white/5 px-1 pt-4"}>
      <button
        type="button"
        className={isCollapsed ? "flex items-center justify-center rounded-xl px-2 py-1.5 text-left transition-colors hover:bg-white/[0.03] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40" : "flex min-w-0 flex-1 items-center gap-3 rounded-xl px-2 py-1.5 text-left transition-colors hover:bg-white/[0.03] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40"}
        aria-label="Open profile settings"
      >
        <Avatar className="size-8 border border-white/5 bg-cyan-400/12">
          <AvatarFallback className="bg-transparent text-[11px] font-medium text-cyan-100">
            {user.username
              .split("_")
              .map((part) => part[0])
              .slice(0, 2)
              .join("")}
          </AvatarFallback>
        </Avatar>

        {!isCollapsed ? <span className="truncate text-sm text-white/84">{user.username}</span> : null}
      </button>

      {!isCollapsed ? (
        <Button
          type="button"
          variant="ghost"
          size="icon-sm"
          className="rounded-xl text-white/45 hover:bg-white/[0.04] hover:text-white"
          aria-label="Log out"
        >
          <LogOut className="size-4" aria-hidden="true" />
        </Button>
      ) : null}
    </div>
  )
}

export { UserFooter }