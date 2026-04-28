import { Plus } from "lucide-react"

import { Button } from "@/components/ui/button"

type AddAgentButtonProps = {
  isCollapsed: boolean
}

function AddAgentButton({ isCollapsed }: AddAgentButtonProps) {
  return (
    <Button
      type="button"
      className={isCollapsed ? "h-10 w-10 justify-center rounded-xl border border-white/5 bg-white/[0.03] p-0 text-white shadow-none hover:bg-white/[0.05]" : "h-10 w-full justify-start gap-3 rounded-xl border border-white/5 bg-white/[0.03] px-3 text-sm font-medium text-white shadow-none hover:bg-white/[0.05]"}
      variant="secondary"
      aria-label="Add Agent"
    >
      <span className="flex size-4 items-center justify-center rounded-full text-white/80">
        <Plus className="size-3.5" aria-hidden="true" />
      </span>
      {!isCollapsed ? <span>Add Agent</span> : null}
    </Button>
  )
}

export { AddAgentButton }
