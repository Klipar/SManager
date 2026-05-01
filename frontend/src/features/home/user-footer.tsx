import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { ShieldCheck, LogOut, User } from "lucide-react"

import type { CurrentUser } from "./types"
import { logout } from "@/lib/ws"

type UserFooterProps = {
  user: CurrentUser
  isCollapsed: boolean
  onOpenAccount?: () => void
  onOpenAdminPanel?: () => void
}

function UserFooter({ user, isCollapsed, onOpenAccount, onOpenAdminPanel }: UserFooterProps) {
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
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <button
            type="button"
            aria-label="Open profile menu"
            className={
              isCollapsed
                ? "flex items-center justify-center rounded-xl px-2 py-1.5 transition-colors hover:bg-white/[0.03] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40 data-[state=open]:bg-white/[0.045]"
                : "flex min-w-0 flex-1 items-center gap-3 rounded-xl px-2 py-1.5 text-left transition-colors hover:bg-white/[0.03] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40 data-[state=open]:bg-white/[0.045]"
            }
          >
            <Avatar className="size-8 border border-white/5 bg-cyan-400/12">
              <AvatarFallback className="bg-transparent text-[11px] font-medium text-cyan-100">
                {initials}
              </AvatarFallback>
            </Avatar>

            {!isCollapsed ? (
              <span className="truncate text-sm text-white/84">{user.username}</span>
            ) : null}
          </button>
        </DropdownMenuTrigger>

        <DropdownMenuContent
          align="end"
          side="top"
          sideOffset={6}
          collisionPadding={12}
          className={
            isCollapsed
              ? "w-44 rounded-2xl border border-white/[0.04] bg-[#12161d]/95 p-1.5 shadow-[0_24px_70px_rgba(0,0,0,0.45)] backdrop-blur-xl"
              : "min-w-44 w-[var(--radix-dropdown-menu-trigger-width)] rounded-2xl border border-white/[0.04] bg-[#12161d]/95 p-1.5 shadow-[0_24px_70px_rgba(0,0,0,0.45)] backdrop-blur-xl"
          }
        >
          <DropdownMenuItem
            className="flex cursor-pointer items-center gap-3 rounded-xl px-3 py-2 text-sm text-white/76 focus:bg-white/[0.04] focus:text-white"
            onSelect={() => onOpenAccount?.()}
          >
            <User className="size-4 text-white/55" />
            <span>Account</span>
          </DropdownMenuItem>

          {user.role === "admin" ? (
            <>
              <DropdownMenuItem
                className="flex cursor-pointer items-center gap-3 rounded-xl px-3 py-2 text-sm text-white/76 focus:bg-white/[0.04] focus:text-white"
                onSelect={() => onOpenAdminPanel?.()}
              >
                <ShieldCheck className="size-4 text-white/55" />
                <span>Admin Panel</span>
              </DropdownMenuItem>

              <DropdownMenuSeparator className="my-1 bg-white/[0.06]" />
            </>
          ) : null}

          <DropdownMenuItem
            className="flex cursor-pointer items-center gap-3 rounded-xl px-3 py-2 text-sm text-white/76 focus:bg-white/[0.04] focus:text-white"
            onSelect={() => {
              logout()
            }}
          >
            <LogOut className="size-4 text-white/55" />
            <span>Log Out</span>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  )
}

export { UserFooter }
