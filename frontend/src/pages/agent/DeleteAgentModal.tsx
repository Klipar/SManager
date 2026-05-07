import { Button } from "@/components/ui/button"
import { AlertTriangle } from "lucide-react"
import type { Agent } from "@/types"

interface DeleteAgentModalProps {
  open: boolean
  agent: Agent | null
  onClose: () => void
  onConfirm: () => void
  isDeleting?: boolean
}

export function DeleteAgentModal({ open, agent, onClose, onConfirm, isDeleting = false }: DeleteAgentModalProps) {
  if (!open || !agent) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
      <div className="absolute inset-0 bg-black/75" onClick={onClose} />
      <div className="relative z-10 w-[480px] rounded-xl border border-red-500/20 bg-[#0b0f13] p-8 text-white shadow-lg text-left">
        <div className="mb-6 flex items-start gap-4">
          <div className="flex size-12 items-center justify-center rounded-xl bg-red-500/15">
            <AlertTriangle className="size-6 text-red-500" />
          </div>
          <div>
            <h2 className="text-2xl font-semibold text-left">Delete Agent</h2>
            <p className="mt-1 text-sm text-white/70">This action cannot be undone</p>
          </div>
        </div>

        <div className="mb-6 space-y-3 rounded-xl border border-white/[0.04] bg-white/[0.03] p-4">
          <p className="text-sm text-white/80">
            Are you sure you want to delete <span className="font-medium text-white">{agent.name}</span>?
          </p>
          <p className="text-sm text-white/60">IP: <span className="font-medium text-white/80">{agent.ip ?? "—"}</span></p>
          <p className="mt-4 text-xs text-red-400/80">⚠️ All associated data will be deleted. This includes tasks and history.</p>
        </div>

        <div className="flex items-center justify-between">
          <Button
            variant="outline"
            onClick={onClose}
            disabled={isDeleting}
            className="border-white/[0.06] text-white/70 hover:text-white disabled:opacity-50"
          >
            Cancel
          </Button>
          <Button
            onClick={onConfirm}
            disabled={isDeleting}
            className="bg-rose-600 shadow-md transition-all hover:scale-105 hover:bg-rose-700 hover:shadow-md disabled:opacity-50 disabled:hover:scale-100"
          >
            {isDeleting ? "Deleting..." : "Delete Agent"}
          </Button>
        </div>
      </div>
    </div>
  )
}

export default DeleteAgentModal
