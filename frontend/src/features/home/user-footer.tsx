import { ShieldCheck, LogOut, Settings } from "lucide-react"
import { useState } from "react"

import { cn } from "@/lib/utils"

import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover"

import type { CurrentUser } from "./types"

type UserFooterProps = {
  user: CurrentUser
  isCollapsed: boolean
}

function UserFooter({ user, isCollapsed }: UserFooterProps) {
  const [open, setOpen] = useState(false)

  const initials = user.username
    .split("_")
    .map((part) => part[0])
    .slice(0, 2)
    .join("")

  return (
    <div
      className={
        isCollapsed
          ? "relative flex w-full justify-center px-1 pt-4"
          : "relative flex items-center gap-3 px-1 pt-4"
      }
    >
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <button
            type="button"
            aria-label="Open profile menu"
            className={cn(
              isCollapsed
                ? "flex items-center justify-center rounded-xl px-2 py-1.5 transition-colors hover:bg-white/[0.03] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40"
                : "flex min-w-0 flex-1 items-center gap-3 rounded-xl px-2 py-1.5 text-left transition-colors hover:bg-white/[0.03] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40",
              open && "bg-white/[0.045]"
            )}
          >
            <Avatar className="size-8 border border-white/5 bg-cyan-400/12">
              <AvatarFallback className="bg-transparent text-[11px] font-medium text-cyan-100">
                {initials}
              </AvatarFallback>
            </Avatar>

            {!isCollapsed && (
              <span className="truncate text-sm text-white/84">
                {user.username}
              </span>
            )}
          </button>
        </PopoverTrigger>

        <PopoverContent
          align="end"
          side="top"
          sideOffset={8}
          avoidCollisions
          collisionPadding={12}
          className="min-w-44 w-[var(--radix-popover-trigger-width)] rounded-2xl border border-white/[0.04] bg-[#12161d] p-1.5 shadow-[0_24px_70px_rgba(0,0,0,0.45)] backdrop-blur-xl"
        >
          <div className="flex flex-col gap-0.5">
            <button
              type="button"
              onClick={() => setOpen(false)}
              className="flex w-full items-center gap-3 rounded-xl px-3 py-2 text-left text-sm text-white/76 transition-colors hover:bg-white/[0.04] hover:text-white"
            >
              <Settings className="size-4 text-white/55" />
              <span>Settings</span>
            </button>

            {user.role === "admin" && (
              <button
                type="button"
                onClick={() => setOpen(false)}
                className="flex w-full items-center gap-3 rounded-xl px-3 py-2 text-left text-sm text-white/76 transition-colors hover:bg-white/[0.04] hover:text-white"
              >
                <ShieldCheck className="size-4 text-white/55" />
                <span>Admin Panel</span>
              </button>
            )}

            <button
              type="button"
              onClick={() => setOpen(false)}
              className="flex w-full items-center gap-3 rounded-xl px-3 py-2 text-left text-sm text-white/76 transition-colors hover:bg-white/[0.04] hover:text-white"
            >
              <LogOut className="size-4 text-white/55" />
              <span>Log Out</span>
            </button>
          </div>
        </PopoverContent>
      </Popover>
    </div>
  )
}

export { UserFooter }