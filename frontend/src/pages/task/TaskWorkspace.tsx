import { Download, Play, Square, Trash2, MoreHorizontal } from "lucide-react"
import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { ScrollArea } from "@/components/ui/scroll-area"
import { cn } from "@/lib/utils"

import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem } from "@/components/ui/dropdown-menu"
import { useEffect, useState, useRef } from "react"
import { useApp } from "@/contexts/AppContext"
import { sendCoreRequest } from "@/lib/ws"
import { AddAgentModal } from "../agent/AddAgentModal"
import DeleteAgentModal from "../agent/DeleteAgentModal"
import AddTaskModal from "./AddTaskModal"
import DeleteTaskModal from "./DeleteTaskModal"

import type { Agent, ScriptType, Task, TaskLog } from "@/types"

type TaskWorkspaceProps = {
  agent: Agent
  selectedTask: Task | null
  selectedLog: TaskLog | null
  onSelectLog: (logId: string | null) => void
  onRunTask: (taskId: string, scriptType: ScriptType) => Promise<boolean>
  onStopTask: (taskId: string) => Promise<boolean>
}

const statusLabel: Record<Task["status"], string> = {
  Ok: "Current status: ok",
  Starting: "Current status: starting",
  Failed: "Current status: failed",
  Stopped: "Current status: stopped",
  Executed: "Current status: executed",
}

const statusDotClass: Record<Task["status"], string> = {
  Ok: "bg-emerald-500",
  Starting: "bg-sky-400",
  Failed: "bg-red-500",
  Stopped: "bg-slate-500",
  Executed: "bg-violet-500",
}

const scriptStripClass: Record<ScriptType, string> = {
  install: "bg-violet-400/80",
  run: "bg-emerald-400/80",
  delete: "bg-red-400/80",
}

function TaskWorkspace({ agent, selectedTask, selectedLog, onSelectLog, onRunTask, onStopTask }: TaskWorkspaceProps) {
  const [pendingRunStart, setPendingRunStart] = useState(false)
  const [now, setNow] = useState(new Date())
  const outputRef = useRef<HTMLDivElement | null>(null)

  const hasRunningRun = selectedTask?.logs.some((log) => log.scriptType === "run" && !log.endedAt) ?? false
  const isRunActive = hasRunningRun || pendingRunStart

  useEffect(() => {
    setPendingRunStart(false)
  }, [selectedTask?.id])

  useEffect(() => {
    if (!hasRunningRun) {
      setPendingRunStart(false)
    }
  }, [hasRunningRun])

  useEffect(() => {
    const interval = setInterval(() => {
      setNow(new Date())
    }, 1000)
    return () => clearInterval(interval)
  }, [])

  useEffect(() => {
    if (!outputRef.current) return
    outputRef.current.scrollTop = outputRef.current.scrollHeight
  }, [selectedLog?.id, selectedLog?.output.length])

  if (!selectedTask) {
    return (
      <div className="relative flex min-h-[calc(100vh-16rem)] items-center justify-center px-8 pb-8 pt-40 text-center md:pt-56">
        <AgentMenu agent={agent} />

        <div>
          <h2 className="text-5xl font-semibold tracking-tight text-white/92">Select Task</h2>
          <p className="mt-4 text-base text-white/50">Choose a task from the sidebar to continue.</p>
        </div>
      </div>
    )
  }

  const sortedLogs = [...selectedTask.logs].sort((a, b) => b.startedAt.localeCompare(a.startedAt))

  const actionRows = [
    { id: "install", title: "Install", icon: Download, tone: "bg-violet-400/80" },
    { id: "run", title: "Run", icon: isRunActive ? Square : Play, tone: "bg-emerald-400/80", active: isRunActive },
    { id: "delete", title: "Delete", icon: Trash2, tone: "bg-red-400/80" },
  ]

  const handleRunAction = async (scriptType: ScriptType) => {
    if (scriptType === "run" && isRunActive) {
      const ok = await handleStopAction()
      if (ok) {
        setPendingRunStart(false)
      }
      return
    }

    const script = scriptType === "install"
      ? selectedTask.installScript
      : scriptType === "run"
        ? selectedTask.runScript
        : selectedTask.deleteScript

    if (!script || !script.trim()) {
      alert(`${scriptType} script is empty for this task`)
      return
    }

    const ok = await onRunTask(selectedTask.id, scriptType)
    if (!ok) {
      alert(`Failed to start ${scriptType} script`)
      return
    }

    if (scriptType === "run") {
      setPendingRunStart(true)
    }
  }

  const handleStopAction = async () => {
    const ok = await onStopTask(selectedTask.id)
    if (!ok) {
      alert("Failed to stop task")
      return false
    }

    return true
  }

  return (
    <div className="grid h-full w-full gap-0 lg:grid-cols-[18rem_1fr]">
      <div className="border-r border-white/[0.035]">
        <div className="p-3">
          <div className="space-y-2">
            {actionRows.map((row) => (
              <button
                key={row.id}
                type="button"
                onClick={() => { void handleRunAction(row.id as ScriptType) }}
                className={cn(
                  "group flex w-full items-center overflow-hidden rounded-xl border border-white/[0.05] bg-white/[0.03] text-left transition-colors hover:bg-white/[0.05] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40",
                  row.active && "border-emerald-400/35 bg-emerald-400/10 hover:bg-emerald-400/14"
                )}
              >
                <div className={cn("flex h-11 w-8 items-center justify-center", row.tone)}>
                  <row.icon className="size-3.5 text-black/80" />
                </div>
                <div className="flex flex-1 items-center justify-between px-3">
                  <span className="text-sm text-white/82">{row.title}</span>
                  {row.id === "run" && isRunActive ? (
                    <Square className="size-4 text-white/70 group-hover:text-white/90" />
                  ) : (
                    <Play className="size-4 text-white/60 group-hover:text-white/75" />
                  )}
                </div>
              </button>
            ))}
          </div>
        </div>

        <div className="border-y border-white/[0.035] px-3 py-2 text-center">
          <h3 className="text-3xl font-medium tracking-tight text-white/90">Runs log</h3>
        </div>

        <ScrollArea className="h-[calc(100vh-19rem)] p-3">
          <div className="space-y-2">
            {sortedLogs.map((log) => (
              <button
                key={log.id}
                type="button"
                onClick={() => onSelectLog(log.id)}
                className="group flex w-full items-center overflow-hidden rounded-xl border border-white/[0.05] bg-white/[0.03] text-left transition-colors hover:bg-white/[0.05] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40"
              >
                <div className={cn("flex h-11 w-8 items-center justify-center", scriptStripClass[log.scriptType])}>
                  {log.scriptType === "install" ? (
                    <Download className="size-3.5 text-black/80" />
                  ) : log.scriptType === "run" ? (
                    <Play className="size-3.5 text-black/80" />
                  ) : (
                    <Trash2 className="size-3.5 text-black/80" />
                  )}
                </div>
                <span className="flex min-w-0 flex-1 items-center justify-between px-3">
                  <span className="truncate text-sm text-white/82">{formatLogLabel(log.startedAt)}</span>
                </span>
              </button>
            ))}
          </div>
        </ScrollArea>
      </div>

      <div className="relative min-h-[34rem]">
        <TaskMenu agent={agent} task={selectedTask} />

        <div className="grid h-full grid-rows-[auto_1fr]">
          <div className="p-4 pt-3">
            <div className="rounded-3xl border border-white/[0.05] bg-white/[0.035] p-4">
              <div className="grid gap-4 lg:grid-cols-[1fr_20rem]">
                <div className="space-y-3 text-white/82">
                  <p className="text-xl">
                    <span className={cn("mr-2 inline-block size-2 rounded-full -translate-y-0.5", statusDotClass[selectedTask.status])} />
                    {statusLabel[selectedTask.status]}
                  </p>
                  <p className="text-sm text-white/70">Started: {formatStartedTime(selectedLog?.startedAt) ?? "-"}</p>
                  <p className="text-sm text-white/70">Working: {formatUptime(selectedLog?.startedAt, selectedLog?.endedAt, now)}</p>
                  <div className="pt-3 text-sm text-white/72">
                    <p>Created by core: {selectedTask.createdByCore}</p>
                    <p className="mt-2">Restart policy: {selectedTask.restartPolicy}</p>
                  </div>
                </div>

                <Card className="flex max-h-60 flex-col overflow-hidden rounded-3xl border-white/[0.05] bg-white/[0.04] shadow-none">
                  <CardHeader className="pb-1">
                    <div className="px-1">
                      <h4 className="text-center text-base font-medium text-white/88">{selectedTask.name}</h4>
                    </div>
                  </CardHeader>
                  <CardContent className="min-h-0 flex-1 overflow-hidden p-3">
                    <div className="h-full overflow-auto rounded-2xl border border-white/[0.05] bg-white/[0.03] p-2 text-sm text-white/74">
                      <p className="whitespace-pre-wrap text-sm text-white/74">{selectedTask.description ?? ""}</p>
                    </div>
                  </CardContent>
                </Card>
              </div>
            </div>
          </div>

          <div className="flex min-h-0 flex-1 flex-col p-4 pb-0">
            <h3 className="mb-3 text-4xl font-medium tracking-tight text-white/90">Output:</h3>
            <div className="flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border border-white/[0.05] bg-white/[0.03] max-h-[calc(100vh-22rem)]">
              {selectedLog ? (
                <div ref={outputRef} className="min-h-0 flex-1 overflow-auto px-4 py-3">
                  <pre className="m-0 whitespace-pre-wrap text-sm leading-6 text-white/76">{selectedLog.output.join("\n")}</pre>
                </div>
              ) : (
                <div className="flex flex-1 items-center justify-center px-4 py-3 text-center text-sm text-white/50">
                  Select a log to view output.
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

function formatDatePart(value: number) {
  return String(value).padStart(2, "0")
}

function formatLogLabel(startedAt?: string | null) {
  if (!startedAt) return "-"
  const date = new Date(startedAt + (startedAt.includes('Z') ? '' : 'Z'))
  if (Number.isNaN(date.getTime())) return "-"
  return `${date.getFullYear()}-${formatDatePart(date.getMonth() + 1)}-${formatDatePart(date.getDate())} ${formatDatePart(date.getHours())}:${formatDatePart(date.getMinutes())}`
}

function formatStartedTime(startedAt?: string | null) {
  if (!startedAt) return undefined
  const date = new Date(startedAt + (startedAt.includes('Z') ? '' : 'Z'))
  if (Number.isNaN(date.getTime())) return undefined
  return `${date.getFullYear()}-${formatDatePart(date.getMonth() + 1)}-${formatDatePart(date.getDate())} ${formatDatePart(date.getHours())}:${formatDatePart(date.getMinutes())}:${formatDatePart(date.getSeconds())}`
}

function formatUptime(startedAt?: string | null, endedAt?: string | null, currentTime?: Date) {
  if (!startedAt) return "-"
  try {
    const startDate = new Date(startedAt + (startedAt.includes('Z') ? '' : 'Z'))
    if (Number.isNaN(startDate.getTime())) return "-"

    const endDate = endedAt
      ? new Date(endedAt + (endedAt.includes('Z') ? '' : 'Z'))
      : (currentTime || new Date())

    if (endedAt && Number.isNaN(endDate.getTime())) return "-"

    const diff = endDate.getTime() - startDate.getTime()
    if (diff < 0) return "-"

    const sec = Math.floor(diff / 1000)
    const days = Math.floor(sec / 86400)
    const hours = Math.floor((sec % 86400) / 3600)
    const minutes = Math.floor((sec % 3600) / 60)
    const parts: string[] = []
    if (days > 0) parts.push(`${days} day${days !== 1 ? "s" : ""}`)
    if (hours > 0) parts.push(`${hours} hour${hours !== 1 ? "s" : ""}`)
    if (minutes > 0) parts.push(`${minutes} minute${minutes !== 1 ? "s" : ""}`)
    if (parts.length === 0) return `${sec} sec${sec !== 1 ? "s" : ""}`
    return parts.join(" ")
  } catch {
    return "-"
  }
}

function TaskMenu({ agent, task }: { agent: Agent; task: Task }) {
  const { removeTaskRecord, setSelectedTaskId, setSelectedLogId, saveTaskRecord } = useApp()
  const [showEdit, setShowEdit] = useState(false)
  const [showDelete, setShowDelete] = useState(false)
  const [isDeleting, setIsDeleting] = useState(false)
  const [deleteError, setDeleteError] = useState<string | null>(null)

  const handleDeleteConfirm = async () => {
    setIsDeleting(true)
    setDeleteError(null)

    try {
      const res = await sendCoreRequest("remove-task", { id: Number(task.id), agent_id: Number(agent.id) })
      if (res?.status === "ok") {
        await removeTaskRecord(task.id)
        setSelectedTaskId(null)
        setSelectedLogId(null)
        setShowDelete(false)
      } else {
        setDeleteError(res?.message ?? "Failed to delete task")
      }
    } catch (error) {
      setDeleteError(String(error))
    } finally {
      setIsDeleting(false)
    }
  }

  const handleEditSave = async (payload: any) => {
    if (payload.id) {
      // Edit mode
      const ok = await saveTaskRecord(payload.id, payload.updates)
      if (!ok) {
        alert("Failed to update task")
        return
      }
    }
    setShowEdit(false)
  }

  return (
    <>
      <button className="fixed right-6 top-6 z-50 p-2 transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40" style={{
        backgroundImage: 'radial-gradient(circle, rgba(255, 255, 255, 0.08) 0%, rgba(255, 255, 255, 0.04) 100%)',
        borderRadius: '9999px',
      }}>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <MoreHorizontal className="size-5 text-white/90" />
          </DropdownMenuTrigger>

          <DropdownMenuContent align="end" side="bottom" sideOffset={8} className="w-44 rounded-2xl border border-white/[0.04] bg-[#12161d]/95 p-1.5 shadow-[0_24px_70px_rgba(0,0,0,0.45)] backdrop-blur-xl">
            <DropdownMenuItem className="flex cursor-pointer items-center gap-3 rounded-xl px-3 py-2 text-sm text-white/76 focus:bg-white/[0.04] focus:text-white" onSelect={() => setShowEdit(true)}>
              Edit
            </DropdownMenuItem>
            <DropdownMenuItem className="flex cursor-pointer items-center gap-3 rounded-xl px-3 py-2 text-sm text-white/76 focus:bg-white/[0.04] focus:text-white" onSelect={() => setShowDelete(true)}>
              Delete
            </DropdownMenuItem>
            <DropdownMenuItem className="flex cursor-pointer items-center gap-3 rounded-xl px-3 py-2 text-sm text-white/76 focus:bg-white/[0.04] focus:text-white" onSelect={() => undefined}>
              Export
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </button>

      <AddTaskModal
        open={showEdit}
        task={task}
        agent={agent}
        onClose={() => setShowEdit(false)}
        onSave={handleEditSave}
        title="Edit Task"
      />

      <DeleteTaskModal
        open={showDelete}
        task={task}
        agent={agent}
        onClose={() => {
          setShowDelete(false)
          setDeleteError(null)
        }}
        onConfirm={handleDeleteConfirm}
        isDeleting={isDeleting}
        error={deleteError}
      />
    </>
  )
}

function AgentMenu({ agent }: { agent: Agent | null }) {
  const { refreshAgents, refreshTasks, setSelectedAgentId } = useApp()
  const [showEdit, setShowEdit] = useState(false)
  const [showDelete, setShowDelete] = useState(false)
  const [isDeleting, setIsDeleting] = useState(false)

  if (!agent) return null

  const handleEditSave = async (payload: { name: string; ip: string; description?: string; port: number }) => {
    try {
      const res = await sendCoreRequest("update-agent", {
        id: Number(agent.id),
        name: payload.name,
        ip: payload.ip,
        description: payload.description,
        port: payload.port,
      })

      if (res?.status === "ok") {
        await refreshAgents()
      } else {
        alert(res?.message ?? "Failed to update agent")
      }
    } catch (e) {
      console.error(e)
      alert("Error updating agent")
    }
  }

  const handleDeleteConfirm = async () => {
    setIsDeleting(true)
    try {
      const res = await sendCoreRequest("remove-agent", { id: Number(agent.id) })
      if (res?.status === "ok") {
        await refreshAgents()
        await refreshTasks()
        setSelectedAgentId(null)
        setShowDelete(false)
      } else {
        alert(res?.message ?? "Failed to delete agent")
      }
    } catch (e) {
      console.error(e)
      alert("Error deleting agent")
    } finally {
      setIsDeleting(false)
    }
  }

  return (
    <>
      <button className="fixed right-6 top-6 z-50 p-2 transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40" style={{
        backgroundImage: 'radial-gradient(circle, rgba(255, 255, 255, 0.08) 0%, rgba(255, 255, 255, 0.04) 100%)',
        borderRadius: '9999px',
      }}>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <MoreHorizontal className="size-5 text-white/80" />
          </DropdownMenuTrigger>

          <DropdownMenuContent align="end" side="bottom" sideOffset={8} className="w-44 rounded-2xl border border-white/[0.04] bg-[#12161d]/95 p-1.5 shadow-[0_24px_70px_rgba(0,0,0,0.45)] backdrop-blur-xl">
            <DropdownMenuItem className="flex cursor-pointer items-center gap-3 rounded-xl px-3 py-2 text-sm text-white/76 focus:bg-white/[0.04] focus:text-white" onSelect={() => setShowEdit(true)}>
              Edit
            </DropdownMenuItem>
            <DropdownMenuItem className="flex cursor-pointer items-center gap-3 rounded-xl px-3 py-2 text-sm text-white/76 focus:bg-white/[0.04] focus:text-white" onSelect={() => setShowDelete(true)}>
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </button>

      <AddAgentModal
        open={showEdit}
        onClose={() => setShowEdit(false)}
        onSave={async (payload) => {
          await handleEditSave(payload)
          setShowEdit(false)
        }}
        initial={{ name: agent.name, ip: agent.ip ?? "", description: agent.description ?? "", port: agent.port ?? undefined }}
        title="Edit Agent"
      />

      <DeleteAgentModal open={showDelete} agent={agent} onClose={() => setShowDelete(false)} onConfirm={handleDeleteConfirm} isDeleting={isDeleting} />
    </>
  )
}

export { TaskWorkspace }
