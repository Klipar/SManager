import { Shield, Sparkles } from "lucide-react"

import { Badge } from "@/components/ui/badge"
import { Card, CardContent } from "@/components/ui/card"

function EmptyState() {
  return (
    <section
      aria-label="Agent details empty state"
      className="flex min-h-[32rem] items-center justify-center p-6"
    >
      <Card className="w-full max-w-xl border-white/5 bg-white/[0.01] text-center shadow-none">
        <CardContent className="p-0">
          <div className="mx-auto mb-6 flex size-20 items-center justify-center rounded-3xl border border-white/8 bg-white/[0.03] text-white/72 shadow-[0_12px_30px_rgba(0,0,0,0.18)]">
            <Sparkles className="size-8" aria-hidden="true" />
          </div>
          <h2 className="text-3xl font-semibold tracking-tight text-white sm:text-4xl">
            Select Agent
          </h2>
          <p className="mx-auto mt-4 max-w-lg text-sm leading-6 text-white/55 sm:text-base">
            Choose an agent from the sidebar to inspect server metrics, logs, and
            collected data.
          </p>
          <Badge variant="outline" className="mt-8 gap-2 rounded-full border-white/8 bg-white/[0.03] px-4 py-2 text-xs text-white/58">
            <Shield className="size-4" aria-hidden="true" />
            <span>Secure workspace overview</span>
          </Badge>
        </CardContent>
      </Card>
    </section>
  )
}

export { EmptyState }