import type { CSSProperties } from "react"

import { AddAgentButton } from "./add-agent-button"
import { AgentList } from "./agent-list"
import { SidebarHeader } from "./sidebar-header"
import { UserFooter } from "./user-footer"
import type { Agent, CurrentUser } from "./types"

type SidebarProps = {
  agents: Agent[]
  selectedAgentId: string | null
  expandedAgentId: string | null
  onSelectAgent: (agentId: string) => void
  isCollapsed: boolean
  onToggleCollapse: () => void
  width: number
  onResizeWidth: (width: number) => void
  user: CurrentUser
}

function Sidebar({
  agents,
  selectedAgentId,
  expandedAgentId,
  onSelectAgent,
  isCollapsed,
  onToggleCollapse,
  width,
  onResizeWidth,
  user,
}: SidebarProps) {
  return (
    <aside
      className="relative flex h-auto w-full shrink-0 flex-col border-b border-white/5 bg-white/[0.018] backdrop-blur-xl md:sticky md:top-0 md:h-screen md:border-b-0 md:border-r md:flex-none md:[width:var(--sidebar-width)]"
      style={{
        ["--sidebar-width" as string]: `${isCollapsed ? 68 : width}px`,
      } as CSSProperties}
    >
      <div
        className={
          isCollapsed
            ? "flex h-full min-h-0 flex-col items-center gap-4 px-2 py-4"
            : "flex h-full min-h-0 flex-col gap-4 px-3 py-4"
        }
      >
        <SidebarHeader isCollapsed={isCollapsed} onToggleCollapse={onToggleCollapse} />

        <AddAgentButton isCollapsed={isCollapsed} />

        <div className="flex min-h-0 flex-1 flex-col gap-2">
          {!isCollapsed ? (
            <p className="px-1 text-[11px] font-medium uppercase tracking-[0.16em] text-white/30">
              Agents
            </p>
          ) : null}
          <AgentList
            agents={agents}
            selectedAgentId={selectedAgentId}
            expandedAgentId={expandedAgentId}
            isCollapsed={isCollapsed}
            onSelectAgent={onSelectAgent}
          />
        </div>

        <UserFooter user={user} isCollapsed={isCollapsed} />
      </div>

      {!isCollapsed ? (
        <button
          type="button"
          onPointerDown={(event) => {
            const startX = event.clientX
            const startWidth = width

            const handlePointerMove = (moveEvent: PointerEvent) => {
              const nextWidth = Math.min(284, Math.max(220, startWidth + (moveEvent.clientX - startX)))
              onResizeWidth(nextWidth)
            }

            const handlePointerUp = () => {
              window.removeEventListener("pointermove", handlePointerMove)
              window.removeEventListener("pointerup", handlePointerUp)
            }

            window.addEventListener("pointermove", handlePointerMove)
            window.addEventListener("pointerup", handlePointerUp)
          }}
          className="absolute top-0 -right-2 h-full w-4 cursor-col-resize border-l border-transparent hover:border-cyan-400/20"
          aria-label="Resize sidebar"
        />
      ) : null}
    </aside>
  )
}

export { Sidebar }