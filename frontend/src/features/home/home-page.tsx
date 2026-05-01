import { useEffect, useState } from "react"
import { sendCoreRequest } from "@/lib/ws"
import { useUser } from "@/contexts/UserContext"
import { currentUser, tasksByAgentId } from "./mock-data"
import { MainPanel } from "./main-panel"
import { Sidebar } from "./sidebar"
import { AdminPanel } from "../admin/admin-panel"
import { AddAgentModal } from "./add-agent-modal"

type HomeViewState = {
  selectedAgentId: string | null
  expandedAgentId: string | null
  selectedTaskId: string | null
  selectedLogId: string | null
  showCreateTask: boolean
  createTaskAgentId: string | null
  isSidebarCollapsed: boolean
  sidebarWidth: number
  showAccount: boolean
  showAdminPanel: boolean
}

const HOME_VIEW_STATE_KEY = "sm_homeViewState"

const defaultHomeViewState: HomeViewState = {
  selectedAgentId: null,
  expandedAgentId: null,
  selectedTaskId: null,
  selectedLogId: null,
  showCreateTask: false,
  createTaskAgentId: null,
  isSidebarCollapsed: false,
  sidebarWidth: 228,
  showAccount: false,
  showAdminPanel: false,
}

function readHomeViewState(): HomeViewState {
  try {
    const rawState = localStorage.getItem(HOME_VIEW_STATE_KEY)
    if (!rawState) return defaultHomeViewState

    const parsed = JSON.parse(rawState) as Partial<HomeViewState>
    return {
      ...defaultHomeViewState,
      ...parsed,
      showAccount: (parsed as any).showAccount ?? (parsed as any).showSettings ?? defaultHomeViewState.showAccount,
    }
  } catch {
    return defaultHomeViewState
  }
}

function HomePage() {
  const { user } = useUser()
  const displayUser = user?.name ? { ...currentUser, username: user.name } : currentUser

  const [selectedAgentId, setSelectedAgentId] = useState<string | null>(() => readHomeViewState().selectedAgentId)
  const [expandedAgentId, setExpandedAgentId] = useState<string | null>(() => readHomeViewState().expandedAgentId)
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(() => readHomeViewState().selectedTaskId)
  const [selectedLogId, setSelectedLogId] = useState<string | null>(() => readHomeViewState().selectedLogId)
  const [showCreateTask, setShowCreateTask] = useState(() => readHomeViewState().showCreateTask)
  const [createTaskAgentId, setCreateTaskAgentId] = useState<string | null>(() => readHomeViewState().createTaskAgentId)
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(() => readHomeViewState().isSidebarCollapsed)
  const [sidebarWidth, setSidebarWidth] = useState(() => readHomeViewState().sidebarWidth)
  const [showAccount, setShowAccount] = useState(() => readHomeViewState().showAccount)
  const [showAdminPanel, setShowAdminPanel] = useState(() => readHomeViewState().showAdminPanel)
  const [showAddAgent, setShowAddAgent] = useState(false)
  const [agentsState, setAgentsState] = useState<any[]>([])
  const [isLoading, setIsLoading] = useState(true)

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
    setShowAccount(false)
    setShowAdminPanel(false)
  }

  const handleAddTask = (agentId: string) => {
    setCreateTaskAgentId(agentId)
    setShowCreateTask(true)
    setShowAccount(false)
    setSelectedTaskId(null)
    setSelectedLogId(null)
  }

  const handleOpenAddAgent = () => {
    setShowAddAgent(true)
  }

  const handleSaveAgent = (payload: { name: string; ip?: string; description?: string; port?: number }) => {
    sendCoreRequest("new-agent", payload)
      .then((res) => {
        if (res.status === "ok") {
          const raw = res.data && res.data.agent ? res.data.agent : res.data
          const created = {
            id: raw && (raw.id ?? raw._id ?? raw.uuid) ? String(raw.id ?? raw._id ?? raw.uuid) : `agent-${Date.now()}`,
            name: raw && raw.name ? raw.name : payload.name,
            status: raw && raw.status ? raw.status : "offline",
            ip: raw && raw.ip ? raw.ip : payload.ip,
            description: raw && raw.description ? raw.description : payload.description,
            port: raw && (raw.port ?? raw.sin) ? (raw.port ?? raw.sin) : payload.port,
          }
          setAgentsState((prev) => [created, ...prev])
          setSelectedAgentId(created.id)
          sendCoreRequest("get-all-agents", null)
            .then((r) => {
              if (r && r.status === "ok") {
                const rawAgents = r.data && r.data.agents ? r.data.agents : []
                const normalized = rawAgents.map((a: any, idx: number) => ({
                  id: a && (a.id ?? a._id ?? a.uuid) ? String(a.id ?? a._id ?? a.uuid) : `agent-${idx}`,
                  name: a && a.name ? a.name : `Unnamed ${idx + 1}`,
                  status: a && a.status ? a.status : "offline",
                  ip: a && a.ip ? a.ip : undefined,
                  description: a && a.description ? a.description : undefined,
                  port: a && (a.port ?? a.sin) ? (a.port ?? a.sin) : undefined,
                }))
                setAgentsState(normalized)
              }
            })
            .catch(() => {})
        } else {
          console.error("Failed to create agent", res)
        }
      })
      .catch((e) => console.error("WS error", e))
      .finally(() => {
        setShowAddAgent(false)
      })
  }

  useEffect(() => {
    setIsLoading(true)
    sendCoreRequest("get-all-agents", null)
      .then((res) => {
        if (res.status === "ok") {
          const rawAgents = res.data && res.data.agents ? res.data.agents : []
          const agents = rawAgents.map((a: any, idx: number) => ({
            id: a && (a.id ?? a._id ?? a.uuid) ? String(a.id ?? a._id ?? a.uuid) : `agent-${idx}`,
            name: a && a.name ? a.name : `Unnamed ${idx + 1}`,
            status: a && a.status ? a.status : "offline",
            ip: a && a.ip ? a.ip : undefined,
            description: a && a.description ? a.description : undefined,
            port: a && (a.port ?? a.sin) ? (a.port ?? a.sin) : undefined,
          }))
          setAgentsState(agents)
        } else {
          console.error("Failed to fetch agents", res)
          setAgentsState([])
        }
      })
      .catch((e) => {
        console.error("WS error", e)
        setAgentsState([])
      })
      .finally(() => {
        setIsLoading(false)
      })
  }, [])

  useEffect(() => {
    const nextState: HomeViewState = {
      selectedAgentId,
      expandedAgentId,
      selectedTaskId,
      selectedLogId,
      showCreateTask,
      createTaskAgentId,
      isSidebarCollapsed,
      sidebarWidth,
      showAccount,
      showAdminPanel,
    }

    try {
      localStorage.setItem(HOME_VIEW_STATE_KEY, JSON.stringify(nextState))
    } catch {
      // ignore storage failures
    }
  }, [
    selectedAgentId,
    expandedAgentId,
    selectedTaskId,
    selectedLogId,
    showCreateTask,
    createTaskAgentId,
    isSidebarCollapsed,
    sidebarWidth,
    showAccount,
    showAdminPanel,
  ])

  useEffect(() => {
    if (!isLoading && selectedAgentId && !agentsState.some((agent) => agent.id === selectedAgentId)) {
      setSelectedAgentId(null)
      setExpandedAgentId(null)
      setSelectedTaskId(null)
      setSelectedLogId(null)
      setCreateTaskAgentId(null)
      setShowCreateTask(false)
    }
  }, [agentsState, isLoading, selectedAgentId])

  useEffect(() => {
    if (showCreateTask && !createTaskAgentId && selectedAgentId) {
      setCreateTaskAgentId(selectedAgentId)
    }
  }, [createTaskAgentId, selectedAgentId, showCreateTask])

  return (
    <main className="min-h-screen w-full bg-[#070b10] text-white">
      {isLoading ? (
        <div className="flex min-h-screen items-center justify-center">
          <div className="text-lg text-white/60">Loading agents...</div>
        </div>
      ) : showCreateTask || showAccount ? (
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
            user={displayUser}
            onOpenAccount={() => {
              setShowAccount(true)
              setShowCreateTask(false)
              setShowAdminPanel(false)
            }}
            onOpenAdminPanel={() => {
              setShowAdminPanel(true)
              setShowAccount(false)
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
            showAccount={showAccount}
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
            user={displayUser}
            onOpenAccount={() => {
              setShowAccount(true)
              setShowCreateTask(false)
              setShowAdminPanel(false)
            }}
            onOpenAdminPanel={() => {
              setShowAdminPanel(true)
              setShowAccount(false)
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
            user={displayUser}
            onOpenAccount={() => {
              setShowAccount(true)
              setShowCreateTask(false)
              setShowAdminPanel(false)
            }}
            onOpenAdminPanel={() => {
              setShowAdminPanel(true)
                setShowAccount(false)
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
            showAccount={showAccount}
          />
        </div>
      )}

      <AddAgentModal open={showAddAgent} onClose={() => setShowAddAgent(false)} onSave={handleSaveAgent} />
    </main>
  )
}

export { HomePage }
