type AgentStatus = "online" | "offline" | "error"

type Agent = {
  id: string
  name: string
  status: AgentStatus
}

type RestartPolicy = "no" | "always" | "on-failure"

type TaskStatus = "ok" | "starting" | "failed" | "stopped" | "executed"

type ScriptType = "install" | "run" | "delete"

type TaskLog = {
  id: string
  startedAt: string
  status: "ok" | "warning" | "error"
  summary: string
  output: string[]
}

type Task = {
  id: string
  name: string
  scriptType: ScriptType
  status: TaskStatus
  description: string
  createdByCore: string
  restartPolicy: RestartPolicy
  logs: TaskLog[]
}

type CurrentUser = {
  username: string
  role: "admin" | "user"
}

export type { Agent, AgentStatus, CurrentUser }
export type { RestartPolicy, ScriptType, Task, TaskLog, TaskStatus }
