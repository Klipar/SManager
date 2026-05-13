import { Button } from "@/components/ui/button"
import { AlertTriangle } from "lucide-react"

interface DeleteInactiveRunsModalProps {
  open: boolean
  count: number
  onClose: () => void
  onConfirm: () => void
  isDeleting?: boolean
  error?: string | null
}

export function DeleteInactiveRunsModal({
  open,
  count,
  onClose,
  onConfirm,
  isDeleting = false,
  error = null,
}: DeleteInactiveRunsModalProps) {
  if (!open) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
      <div className="absolute inset-0 bg-black/75" onClick={onClose} />
      <div className="relative z-10 w-[480px] max-h-[90vh] overflow-y-auto rounded-xl border border-red-500/20 bg-[#0b0f13] p-8 text-left text-white shadow-lg">
        <div className="mb-6 flex items-start gap-4">
          <div className="flex size-12 items-center justify-center rounded-xl bg-red-500/15">
            <AlertTriangle className="size-6 text-red-500" />
          </div>
          <div>
            <h2 className="text-2xl font-semibold">Delete Inactive Runs</h2>
            <p className="mt-1 text-sm text-white/70">This action cannot be undone</p>
          </div>
        </div>

        <div className="mb-6 space-y-3 rounded-xl border border-white/[0.04] bg-white/[0.03] p-4">
          <p className="text-sm text-white/80">
            Delete <span className="font-medium text-white">{count}</span> finished run{count === 1 ? "" : "s"}?
          </p>
          <p className="mt-4 text-xs text-white/80">Running or incomplete runs remain untouched.</p>
        </div>

        {error ? <p className="mt-2 text-sm text-rose-400">{error}</p> : null}

        <div className="mt-6 flex items-center justify-between">
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
            {isDeleting ? "Deleting..." : "Delete Runs"}
          </Button>
        </div>
      </div>
    </div>
  )
}

export default DeleteInactiveRunsModal
