import { useState } from "react"

import { agents, currentUser } from "./mock-data"
import { MainPanel } from "./main-panel"
import { Sidebar } from "./sidebar"

function HomePage() {
  const [selectedAgentId, setSelectedAgentId] = useState<string | null>(null)
  const [expandedAgentId, setExpandedAgentId] = useState<string | null>(null)
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false)
  const [sidebarWidth, setSidebarWidth] = useState(228)

  const selectedAgent = agents.find((agent) => agent.id === selectedAgentId) ?? null

  const handleSelectAgent = (agentId: string) => {
    setSelectedAgentId(agentId)
    setExpandedAgentId((currentExpandedAgentId) =>
      currentExpandedAgentId === agentId ? null : agentId,
    )
  }

  return (
    <main className="min-h-screen bg-[#070b10] text-white">
      <div className="flex min-h-screen flex-col md:flex-row">
        <Sidebar
          agents={agents}
          selectedAgentId={selectedAgentId}
          expandedAgentId={expandedAgentId}
          onSelectAgent={handleSelectAgent}
          isCollapsed={isSidebarCollapsed}
          onToggleCollapse={() => setIsSidebarCollapsed((currentValue) => !currentValue)}
          width={sidebarWidth}
          onResizeWidth={setSidebarWidth}
          user={currentUser}
        />
        <MainPanel selectedAgent={selectedAgent} />
      </div>
    </main>
  )
}

export { HomePage }