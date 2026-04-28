import { useState } from "react"

import { agents, currentUser, tasksByAgentId } from "./mock-data"
import { MainPanel } from "./main-panel"
import { Sidebar } from "./sidebar"

function HomePage() {
  const [selectedAgentId, setSelectedAgentId] = useState<string | null>(null)
  const [expandedAgentId, setExpandedAgentId] = useState<string | null>(null)
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null)
  const [selectedLogId, setSelectedLogId] = useState<string | null>(null)
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false)
  const [sidebarWidth, setSidebarWidth] = useState(228)

  const selectedAgent = agents.find((agent) => agent.id === selectedAgentId) ?? null
  const selectedAgentTasks = selectedAgent ? (tasksByAgentId[selectedAgent.id] ?? []) : []
  const selectedTask = selectedAgentTasks.find((task) => task.id === selectedTaskId) ?? null
  const selectedLog = selectedTask?.logs.find((log) => log.id === selectedLogId) ?? null

  const handleSelectAgent = (agentId: string) => {
    setSelectedAgentId(agentId)
    setSelectedTaskId(null)
    setSelectedLogId(null)
    setExpandedAgentId((currentExpandedAgentId) =>
      currentExpandedAgentId === agentId ? null : agentId,
    )
  }

  const handleSelectTask = (taskId: string) => {
    setSelectedTaskId(taskId)
    setSelectedLogId(null)
  }

  return (
    <main className="min-h-screen bg-[#070b10] text-white">
      <div className="flex min-h-screen flex-col md:flex-row">
        <Sidebar
          agents={agents}
          selectedAgentId={selectedAgentId}
          expandedAgentId={expandedAgentId}
          selectedTaskId={selectedTaskId}
          tasksByAgentId={tasksByAgentId}
          onSelectAgent={handleSelectAgent}
          onSelectTask={handleSelectTask}
          isCollapsed={isSidebarCollapsed}
          onToggleCollapse={() => setIsSidebarCollapsed((currentValue) => !currentValue)}
          width={sidebarWidth}
          onResizeWidth={setSidebarWidth}
          user={currentUser}
        />
        <MainPanel
          selectedAgent={selectedAgent}
          selectedTask={selectedTask}
          selectedLog={selectedLog ?? null}
          onSelectLog={setSelectedLogId}
        />
      </div>
    </main>
  )
}

export { HomePage }
