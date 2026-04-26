import { Card, CardContent } from "@/components/ui/card"
import { Separator } from "@/components/ui/separator"

type AgentTasksProps = {
  agentName: string
}

function AgentTasks({ agentName }: AgentTasksProps) {
  return (
    <Card className="ml-12 mt-2 rounded-xl border-white/5 bg-white/[0.02] text-white/65 shadow-none">
      <CardContent className="space-y-2 px-3 py-3 text-sm">
        <p className="text-[11px] uppercase tracking-[0.16em] text-white/35">
          Tasks
        </p>
        <Separator className="bg-white/[0.06]" />
        <p className="text-sm text-white/70">
          Task list for {agentName} will appear here.
        </p>
      </CardContent>
    </Card>
  )
}

export { AgentTasks }