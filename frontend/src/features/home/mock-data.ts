import type { Agent, CurrentUser } from "./types"

const agents: Agent[] = [
  { id: "agent-main", name: "Agent main", status: "online" },
  { id: "agent-67", name: "Agent 67", status: "online" },
  { id: "agent-aboba", name: "Agent aboba", status: "warning" },
  { id: "agent-stu-fein", name: "Agent STU FEIN", status: "offline" },
  { id: "agent-aquila", name: "Agent Aquila", status: "online" },
  { id: "agent-orbit", name: "Agent Orbit", status: "offline" },
]

const currentUser: CurrentUser = {
  username: "User_idiot_nickname",
}

export { agents, currentUser }