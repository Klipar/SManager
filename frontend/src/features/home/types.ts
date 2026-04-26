type AgentStatus = "online" | "offline" | "warning"

type Agent = {
  id: string
  name: string
  status: AgentStatus
}

type CurrentUser = {
  username: string
  role: "admin" | "user"
}

export type { Agent, AgentStatus, CurrentUser }