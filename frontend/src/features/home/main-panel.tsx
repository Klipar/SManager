import { EmptyState } from "./empty-state"
import { TaskWorkspace } from "./task-workspace"
import CreateTaskPanel from "./create-task-panel"
import type { Agent, Task, TaskLog } from "./types"

type MainPanelProps = {
  selectedAgent: Agent | null
  selectedTask: Task | null
  selectedLog: TaskLog | null
  onSelectLog: (logId: string | null) => void
  showCreateTask?: boolean
  createTaskAgent?: Agent | null
  onCloseCreateTask?: () => void
}

function MainPanel({ selectedAgent, selectedTask, selectedLog, onSelectLog, showCreateTask, createTaskAgent, onCloseCreateTask }: MainPanelProps) {
  return (
    <section className="relative flex min-h-[calc(100vh-4rem)] flex-1 items-stretch py-5 pl-0 pr-5 sm:pl-1 sm:pr-6 md:py-8 md:pl-2 md:pr-10">
      <div
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(34,211,238,0.08),transparent_30%),linear-gradient(135deg,rgba(255,255,255,0.02),transparent_40%)]"
      />

      <div className="relative flex w-full flex-col justify-center">
        {showCreateTask ? (
          <CreateTaskPanel agent={createTaskAgent ?? null} onClose={onCloseCreateTask ?? (() => {})} />
        ) : selectedAgent ? (
          <TaskWorkspace
            key={selectedAgent.id}
            agent={selectedAgent}
            selectedTask={selectedTask}
            selectedLog={selectedLog}
            onSelectLog={onSelectLog}
          />
        ) : (
          <EmptyState />
        )}
      </div>
    </section>
  )
}

export { MainPanel }
