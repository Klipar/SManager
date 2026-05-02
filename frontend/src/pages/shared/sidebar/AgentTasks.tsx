import { Circle, FileText, Plus } from "lucide-react"
import { Button } from "@/components/ui/Button"
import { Separator } from "@/components/ui/Separator"
import { cn } from "@/lib/utils"
import type { Task } from "@/types"

type AgentTasksProps = {
  tasks: Task[]
  selectedTaskId: string | null
  onSelectTask: (taskId: string) => void
  onAddTask?: () => void
}

const taskStatusStyles: Record<Task["status"], string> = {
  ok: "text-emerald-400",
  starting: "text-cyan-300",
  failed: "text-red-400",
  stopped: "text-white/30",
  executed: "text-white/60",
}

function AgentTasks({ tasks, selectedTaskId, onSelectTask, onAddTask }: AgentTasksProps) {
  return (
    <div className="space-y-2 border-t border-white/[0.04] px-3 pb-3 pt-2 text-sm text-white/65">
      <p className="text-[11px] uppercase tracking-[0.16em] text-white/35">
        Tasks
      </p>
      <Separator className="bg-white/[0.045]" />

      <div className="space-y-1.5">
        {tasks.length > 0 ? (
          tasks.map((task) => (
            <button
              key={task.id}
              type="button"
              onClick={() => onSelectTask(task.id)}
              className={cn(
                "flex w-full items-center gap-2 rounded-lg px-2 py-1.5 text-left text-sm transition-colors",
                selectedTaskId === task.id
                  ? "bg-white/[0.08] text-white"
                  : "text-white/72 hover:bg-white/[0.05] hover:text-white",
              )}
            >
              <FileText className="size-3.5 text-white/50" aria-hidden="true" />
              <span className="min-w-0 flex-1 truncate">{task.name}</span>
              <Circle
                className={cn("size-2.5 shrink-0 fill-current", taskStatusStyles[task.status])}
                aria-hidden="true"
              />
            </button>
          ))
        ) : (
          <p className="rounded-lg px-2 py-1.5 text-xs text-white/45">
            No tasks yet.
          </p>
        )}
      </div>

      <Button
        type="button"
        variant="secondary"
        className="h-8 w-full justify-start rounded-lg border border-white/[0.05] bg-white/[0.035] px-2 text-xs font-medium text-white/82 hover:bg-white/[0.055]"
      onClick={() => onAddTask?.()}
      >
        <Plus className="mr-1.5 size-3.5" aria-hidden="true" />
        Add new task
      </Button>
    </div>
  )
}

export { AgentTasks }
