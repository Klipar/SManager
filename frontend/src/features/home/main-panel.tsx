import { Server } from "lucide-react"

import { Badge } from "@/components/ui/badge"
import { Card, CardContent } from "@/components/ui/card"

import { EmptyState } from "./empty-state"
import type { Agent } from "./types"

type MainPanelProps = {
  selectedAgent: Agent | null
}

const statusLabel: Record<Agent["status"], string> = {
  online: "Online",
  offline: "Offline",
  warning: "Needs attention",
}

function MainPanel({ selectedAgent }: MainPanelProps) {
  return (
    <section className="relative flex min-h-[calc(100vh-4rem)] flex-1 items-stretch px-5 py-5 sm:px-6 md:px-10 md:py-8">
      <div
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(34,211,238,0.08),transparent_30%),linear-gradient(135deg,rgba(255,255,255,0.02),transparent_40%)]"
      />

      <div className="relative flex w-full flex-col justify-center">
        {selectedAgent ? (
          <div className="mx-auto w-full max-w-4xl space-y-6">
            <Badge variant="outline" className="gap-2 rounded-full border-white/8 bg-white/[0.03] px-3 py-2 text-xs text-white/60">
              <Server className="size-3.5 text-cyan-200" aria-hidden="true" />
              <span>{statusLabel[selectedAgent.status]}</span>
            </Badge>

            <div>
              <h1 className="text-4xl font-semibold tracking-tight text-white sm:text-5xl">
                {selectedAgent.name}
              </h1>
              <p className="mt-3 max-w-2xl text-sm leading-6 text-white/52 sm:text-base">
                This dashboard area is ready for logs, permissions, task queues,
                and operational controls tied to the selected agent.
              </p>
            </div>

            <div className="grid gap-3 sm:grid-cols-3">
              {[
                ["Logs", "Live stream ready"],
                ["Metrics", "Server health snapshot"],
                ["Actions", "Queued operations"],
              ].map(([label, value]) => (
                <Card key={label} className="rounded-2xl border-white/8 bg-white/[0.03]">
                  <CardContent className="px-4 py-4">
                    <p className="text-xs uppercase tracking-[0.14em] text-white/35">{label}</p>
                    <p className="mt-2 text-sm text-white/78">{value}</p>
                  </CardContent>
                </Card>
              ))}
            </div>
          </div>
        ) : (
          <EmptyState />
        )}
      </div>
    </section>
  )
}

export { MainPanel }