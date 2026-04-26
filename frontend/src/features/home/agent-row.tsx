import { ChevronDown, Circle } from "lucide-react"

import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { cn } from "@/lib/utils"

import { AgentTasks } from "./agent-tasks"
import type { Agent } from "./types"

type AgentRowProps = {
  agent: Agent
  isSelected: boolean
  isExpanded: boolean
  isCollapsed: boolean
  onSelect: (agentId: string) => void
}

const statusStyles: Record<Agent["status"], string> = {
  online: "text-emerald-400",
  offline: "text-white/25",
  warning: "text-amber-400",
}

function AgentRow({ agent, isSelected, isExpanded, isCollapsed, onSelect }: AgentRowProps) {
  return (
    <div>
      <button
        type="button"
        aria-pressed={isSelected}
        aria-expanded={isExpanded}
        onClick={() => onSelect(agent.id)}
        className={cn(
          "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-left transition-colors duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40",
          isCollapsed && "justify-center px-2",
          isSelected
            ? "bg-white/[0.065] text-white"
            : "text-white/76 hover:bg-white/[0.035] hover:text-white",
        )}
      >
        <Avatar className="size-9 border border-white/5 bg-white/[0.04]">
          <AvatarFallback className="bg-transparent text-xs font-medium text-white/80">
            {agent.name
              .split(" ")
              .map((part) => part[0])
              .slice(0, 2)
              .join("")}
          </AvatarFallback>
        </Avatar>

        {!isCollapsed ? (
          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-2">
              <span className="truncate text-sm font-medium">{agent.name}</span>
              <Circle
                className={cn("size-2.5 shrink-0 fill-current", statusStyles[agent.status])}
                aria-hidden="true"
              />
            </div>
          </div>
        ) : null}

        {!isCollapsed ? (
          <ChevronDown
            className={cn(
              "size-4 text-white/28 transition-transform duration-200",
              isExpanded && "rotate-180 text-white/45",
            )}
            aria-hidden="true"
          />
        ) : null}
      </button>

      {!isCollapsed && isExpanded ? <AgentTasks agentName={agent.name} /> : null}
    </div>
  )
}

export { AgentRow }