import { ScrollArea } from "@/components/ui/scroll-area"

import { AgentRow } from "./agent-row"
import type { Agent, Task } from "./types"

type AgentListProps = {
  agents: Agent[]
  selectedAgentId: string | null
  expandedAgentId: string | null
  isCollapsed: boolean
  tasksByAgentId: Record<string, Task[]>
  selectedTaskId: string | null
  onSelectAgent: (agentId: string) => void
  onSelectTask: (taskId: string) => void
  onAddTask?: (agentId: string) => void
}

function AgentList({
  agents,
  selectedAgentId,
  expandedAgentId,
  isCollapsed,
  tasksByAgentId,
  selectedTaskId,
  onSelectAgent,
  onSelectTask,
  onAddTask,
}: AgentListProps) {
  return (
    <ScrollArea className="min-h-0 flex-1 pr-1">
      <div className={isCollapsed ? "space-y-1" : "space-y-1.5"}>
        {agents.map((agent) => (
          <AgentRow
            key={agent.id}
            agent={agent}
            isSelected={agent.id === selectedAgentId}
            isExpanded={agent.id === expandedAgentId}
            isCollapsed={isCollapsed}
            tasks={tasksByAgentId[agent.id] ?? []}
            selectedTaskId={selectedTaskId}
            onSelect={onSelectAgent}
            onSelectTask={onSelectTask}
            onAddTask={onAddTask}
          />
        ))}
      </div>
    </ScrollArea>
  )
}

export { AgentList }
