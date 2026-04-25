type AgentTasksProps = {
  agentName: string
}

function AgentTasks({ agentName }: AgentTasksProps) {
  return (
    <div className="ml-12 mt-2 space-y-2 rounded-xl border border-white/5 bg-white/[0.02] px-3 py-3 text-sm text-white/65">
      <p className="text-[11px] uppercase tracking-[0.16em] text-white/35">
        Tasks
      </p>
      <p className="text-sm text-white/70">
        Task list for {agentName} will appear here.
      </p>
    </div>
  )
}

export { AgentTasks }