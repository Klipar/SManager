import { EmptyState } from "./empty-state"
import { TaskWorkspace } from "./task-workspace"
import CreateTaskPanel from "./create-task-panel"
import SettingsPanel from "./settings-panel"
import type { Agent, Task, TaskLog } from "./types"

type MainPanelProps = {
  selectedAgent: Agent | null
  selectedTask: Task | null
  selectedLog: TaskLog | null
  onSelectLog: (logId: string | null) => void
  showCreateTask?: boolean
  createTaskAgent?: Agent | null
  onCloseCreateTask?: () => void
  showSettings?: boolean
  onCloseSettings?: () => void
}

function MainPanel({ selectedAgent, selectedTask, selectedLog, onSelectLog, showCreateTask, createTaskAgent, onCloseCreateTask, showSettings, onCloseSettings }: MainPanelProps) {
  return (
    <section className="relative flex min-h-[calc(100vh-4rem)] w-full flex-1 flex-col py-5 pl-0 pr-5 sm:pl-1 sm:pr-6 md:py-8 md:pl-2 md:pr-10">
      <div
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(34,211,238,0.08),transparent_30%),linear-gradient(135deg,rgba(255,255,255,0.02),transparent_40%)]"
      />

      <div className="relative w-full flex-1">
        <div className="mx-auto w-full max-w-7xl px-8">
          {showCreateTask ? (
            <div className="pt-2 md:pt-4">
              <CreateTaskPanel agent={createTaskAgent ?? null} onClose={onCloseCreateTask ?? (() => {})} />
            </div>
          ) : showSettings ? (
            <div className="pt-2 md:pt-4">
              <SettingsPanel onClose={onCloseSettings ?? (() => {})} />
            </div>
          ) : (
            <div className="flex h-full flex-col justify-center">
              {selectedAgent ? (
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
          )}
        </div>
      </div>
    </section>
  )
}

export { MainPanel }
