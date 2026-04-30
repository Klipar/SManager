import { useState } from "react"

import { agents, currentUser, tasksByAgentId } from "./mock-data"
import { MainPanel } from "./main-panel"
import { Sidebar } from "./sidebar"
import { AdminPanel } from "../admin/admin-panel"
import { AddAgentModal } from "./add-agent-modal"

function HomePage() {
  const [selectedAgentId, setSelectedAgentId] = useState<string | null>(null)
  const [expandedAgentId, setExpandedAgentId] = useState<string | null>(null)
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null)
  const [selectedLogId, setSelectedLogId] = useState<string | null>(null)
  const [showCreateTask, setShowCreateTask] = useState(false)
  const [createTaskAgentId, setCreateTaskAgentId] = useState<string | null>(null)
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false)
  const [sidebarWidth, setSidebarWidth] = useState(228)
  const [showSettings, setShowSettings] = useState(false)
  const [showAdminPanel, setShowAdminPanel] = useState(false)
  const [showAddAgent, setShowAddAgent] = useState(false)
  const [agentsState, setAgentsState] = useState(agents)

  const selectedAgent = agentsState.find((agent) => agent.id === selectedAgentId) ?? null
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
    setShowCreateTask(false)
    setShowSettings(false)
    setShowAdminPanel(false)
  }

  const handleAddTask = (agentId: string) => {
    setCreateTaskAgentId(agentId)
    setShowCreateTask(true)
    setShowSettings(false)
    setSelectedTaskId(null)
    setSelectedLogId(null)
  }

  const handleOpenAddAgent = () => {
    // Open Add Agent modal without changing the current main view
    setShowAddAgent(true)
  }

  const handleSaveAgent = (payload: { name: string; ip?: string; description?: string; sin?: string }) => {
    const newAgent = {
      id: `${Date.now()}`,
      name: payload.name,
      status: "offline" as const,
      ip: payload.ip,
      description: payload.description,
      sin: payload.sin,
    }
    setAgentsState((prev) => [newAgent, ...prev])
    setShowAddAgent(false)
    setSelectedAgentId(newAgent.id)
  }

  return (
    <main className="min-h-screen w-full bg-[#070b10] text-white">
      {showCreateTask || showSettings ? (
        <div className="flex min-h-screen w-full flex-col md:flex-row">
          <Sidebar
            agents={agentsState}
            selectedAgentId={selectedAgentId}
            expandedAgentId={expandedAgentId}
            selectedTaskId={selectedTaskId}
            tasksByAgentId={tasksByAgentId}
            onSelectAgent={handleSelectAgent}
            onSelectTask={handleSelectTask}
            onAddTask={handleAddTask}
            isCollapsed={isSidebarCollapsed}
            onToggleCollapse={() => setIsSidebarCollapsed((currentValue) => !currentValue)}
            width={sidebarWidth}
            onResizeWidth={setSidebarWidth}
            user={currentUser}
            onOpenSettings={() => {
              setShowSettings(true)
              setShowCreateTask(false)
              setShowAdminPanel(false)
            }}
            onOpenAdminPanel={() => {
              setShowAdminPanel(true)
              setShowSettings(false)
              setShowCreateTask(false)
            }}
            onOpenAddAgent={handleOpenAddAgent}
          />
            <MainPanel
            selectedAgent={selectedAgent}
            selectedTask={selectedTask}
            selectedLog={selectedLog ?? null}
            onSelectLog={setSelectedLogId}
            showCreateTask={showCreateTask}
            createTaskAgent={agentsState.find((a) => a.id === createTaskAgentId) ?? null}
            showSettings={showSettings}
            onCloseSettings={() => setShowSettings(false)}
          />
        </div>
      ) : showAdminPanel ? (
        <div className="flex min-h-screen w-full flex-col md:flex-row">
          <Sidebar
            agents={agentsState}
            selectedAgentId={selectedAgentId}
            expandedAgentId={expandedAgentId}
            selectedTaskId={selectedTaskId}
            tasksByAgentId={tasksByAgentId}
            onSelectAgent={handleSelectAgent}
            onSelectTask={handleSelectTask}
            onAddTask={handleAddTask}
            isCollapsed={isSidebarCollapsed}
            onToggleCollapse={() => setIsSidebarCollapsed((currentValue) => !currentValue)}
            width={sidebarWidth}
            onResizeWidth={setSidebarWidth}
            user={currentUser}
            onOpenSettings={() => {
              setShowSettings(true)
              setShowCreateTask(false)
              setShowAdminPanel(false)
            }}
            onOpenAdminPanel={() => {
              setShowAdminPanel(true)
              setShowSettings(false)
              setShowCreateTask(false)
            }}
            onOpenAddAgent={handleOpenAddAgent}
          />
          <div className="flex-1">
            <AdminPanel />
          </div>
        </div>
      ) : (
        <div className="flex min-h-screen w-full flex-col md:flex-row">
          <Sidebar
            agents={agentsState}
            selectedAgentId={selectedAgentId}
            expandedAgentId={expandedAgentId}
            selectedTaskId={selectedTaskId}
            tasksByAgentId={tasksByAgentId}
            onSelectAgent={handleSelectAgent}
            onSelectTask={handleSelectTask}
            onAddTask={handleAddTask}
            isCollapsed={isSidebarCollapsed}
            onToggleCollapse={() => setIsSidebarCollapsed((currentValue) => !currentValue)}
            width={sidebarWidth}
            onResizeWidth={setSidebarWidth}
            user={currentUser}
            onOpenSettings={() => {
              setShowSettings(true)
              setShowCreateTask(false)
              setShowAdminPanel(false)
            }}
            onOpenAdminPanel={() => {
              setShowAdminPanel(true)
              setShowSettings(false)
              setShowCreateTask(false)
            }}
            onOpenAddAgent={handleOpenAddAgent}
          />
          <MainPanel
            selectedAgent={selectedAgent}
            selectedTask={selectedTask}
            selectedLog={selectedLog ?? null}
            onSelectLog={setSelectedLogId}
            showCreateTask={showCreateTask}
            createTaskAgent={agentsState.find((a) => a.id === createTaskAgentId) ?? null}
            showSettings={showSettings}
            onCloseSettings={() => setShowSettings(false)}
          />
        </div>
      )}

      <AddAgentModal open={showAddAgent} onClose={() => setShowAddAgent(false)} onSave={handleSaveAgent} />
    </main>
  )
}

export { HomePage }
