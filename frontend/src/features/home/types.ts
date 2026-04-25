type AgentStatus = "online" | "offline" | "warning"

type Agent = {
  id: string
  name: string
  status: AgentStatus
}

type CurrentUser = {
  username: string
}

export type { Agent, AgentStatus, CurrentUser }