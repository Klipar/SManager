import { ArrowUpRight, Download, PencilLine, Play, Trash2 } from "lucide-react"
import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { ScrollArea } from "@/components/ui/scroll-area"
import { cn } from "@/lib/utils"

import type { Agent, Task, TaskLog } from "@/types"

type TaskWorkspaceProps = {
  agent: Agent
  selectedTask: Task | null
  selectedLog: TaskLog | null
  onSelectLog: (logId: string | null) => void
}

const statusLabel: Record<Agent["status"], string> = {
  online: "Current status: Online",
  offline: "Current status: Offline",
  error: "Current status: Error",
}

const logStripClass: Record<TaskLog["status"], string> = {
  ok: "bg-emerald-500/90",
  warning: "bg-amber-500/90",
  error: "bg-red-500/90",
}

function TaskWorkspace({ agent, selectedTask, selectedLog, onSelectLog }: TaskWorkspaceProps) {
  const actionRows = [
    { id: "install", title: "Install", icon: Download, tone: "bg-violet-400/80" },
    { id: "run", title: "Run", icon: PencilLine, tone: "bg-emerald-400/80" },
    { id: "delete", title: "Delete", icon: Trash2, tone: "bg-red-400/80" },
  ]

  if (!selectedTask) {
    return (
      <div className="flex min-h-[calc(100vh-16rem)] items-center justify-center px-8 pb-8 pt-40 text-center md:pt-56">
        <div>
          <h2 className="text-5xl font-semibold tracking-tight text-white/92">Select Task</h2>
          <p className="mt-4 text-base text-white/50">
            Choose a task from the sidebar to continue.
          </p>
        </div>
      </div>
    )
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
                className="group flex w-full items-center overflow-hidden rounded-xl border border-white/[0.05] bg-white/[0.03] text-left transition-colors hover:bg-white/[0.05] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40"
              >
                <div className={cn("flex h-11 w-8 items-center justify-center", row.tone)}>
                  <row.icon className="size-3.5 text-black/80" />
                </div>
                <div className="flex flex-1 items-center justify-between px-3">
                  <span className="text-sm text-white/82">{row.title}</span>
                  <Play className="size-4 text-white/60 group-hover:text-white/75" />
                </div>
              </button>
            ))}
          </div>
        </div>

        <div className="border-y border-white/[0.035] px-3 py-2 text-center">
          <h3 className="text-3xl font-medium tracking-tight text-white/90">Start log</h3>
        </div>

        <ScrollArea className="h-[calc(100vh-19rem)] p-3">
          <div className="space-y-2">
            {selectedTask.logs.map((log) => (
              <button
                key={log.id}
                type="button"
                onClick={() => onSelectLog(log.id)}
                className="group flex w-full items-center overflow-hidden rounded-2xl border border-white/[0.05] bg-white/[0.03] text-left transition-colors hover:bg-white/[0.05]"
              >
                <span className={cn("h-14 w-7 shrink-0", logStripClass[log.status])} />
                <span className="flex min-w-0 flex-1 items-center justify-between px-3">
                  <span className="truncate text-sm text-white/84">{log.startedAt}</span>
                  <ArrowUpRight className="size-4 text-white/45 group-hover:text-white/75" />
                </span>
              </button>
            ))}
          </div>
        </ScrollArea>
      </div>

      <div className="min-h-[34rem]">
        {!selectedLog ? (
          <div className="flex min-h-[calc(100vh-18rem)] items-center justify-center px-8 pb-8 pt-40 text-center md:pt-56">
            <div>
              <h2 className="text-5xl font-semibold tracking-tight text-white/92">Select Log</h2>
              <p className="mt-4 text-base text-white/50">
                Choose a log from the left panel to view task output.
              </p>
            </div>
          </div>
        ) : (
          <div className="grid h-full grid-rows-[auto_1fr]">
            <div className="p-4">
              <div className="rounded-3xl border border-white/[0.05] bg-white/[0.035] p-4">
                <div className="grid gap-4 lg:grid-cols-[1fr_20rem]">
                  <div className="space-y-3 text-white/82">
                    <p className="text-xl">
                      <span className="mr-2 inline-block size-2 rounded-full bg-emerald-500" />
                      {statusLabel[agent.status]}
                    </p>
                    <p className="text-sm text-white/70">Started: {selectedLog.startedAt}</p>
                    <p className="text-sm text-white/70">Working: 5 days 6 hours 7 minutes 52 seconds</p>
                    <div className="pt-3 text-sm text-white/72">
                      <p>Created by core: {selectedTask.createdByCore}</p>
                      <p className="mt-2">Restart policy: {selectedTask.restartPolicy}</p>
                    </div>
                  </div>

                  <Card className="rounded-3xl border-white/[0.05] bg-white/[0.04] shadow-none">
                    <CardHeader className="pb-2">
                      <h4 className="text-center text-base font-medium text-white/88">Task Name</h4>
                    </CardHeader>
                    <CardContent>
                      <div className="rounded-2xl border border-white/[0.05] bg-white/[0.03] p-3 text-sm text-white/74">
                        <p>{selectedTask.description}</p>
                      </div>
                    </CardContent>
                  </Card>
                </div>
              </div>
            </div>

            <div className="p-4">
              <h3 className="mb-3 text-4xl font-medium tracking-tight text-white/90">Output:</h3>
              <div className="h-full rounded-xl border border-white/[0.05] bg-white/[0.03] p-3">
                <pre className="overflow-auto whitespace-pre-wrap text-sm leading-6 text-white/76">
                  {selectedLog.output.join("\n")}
                </pre>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export { TaskWorkspace }
