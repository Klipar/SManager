import { Button } from "@/components/ui/button"
import { AlertTriangle } from "lucide-react"
import type { AdminUser } from "./types"

interface DeleteUserModalProps {
  open: boolean
  user: AdminUser | null
  onClose: () => void
  onConfirm: () => void
}

export function DeleteUserModal({ open, user, onClose, onConfirm }: DeleteUserModalProps) {
  if (!open || !user) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
      <div className="absolute inset-0 bg-black/75" onClick={onClose} />
      <div className="relative z-10 w-[480px] rounded-xl border border-red-500/20 bg-[#0b0f13] p-8 text-white shadow-lg">
        <div className="mb-6 flex items-start gap-4">
          <div className="flex size-12 items-center justify-center rounded-xl bg-red-500/15">
            <AlertTriangle className="size-6 text-red-500" />
          </div>
          <div>
            <h2 className="text-2xl font-semibold">Delete User</h2>
            <p className="mt-1 text-sm text-white/70">This action cannot be undone</p>
          </div>
        </div>

        <div className="mb-6 space-y-3 rounded-xl border border-white/[0.04] bg-white/[0.03] p-4">
          <p className="text-sm text-white/80">
            Are you sure you want to delete <span className="font-medium text-white">{user.name}</span>?
          </p>
          <p className="text-sm text-white/60">
            Email: <span className="font-medium text-white/80">{user.email}</span>
          </p>
          <p className="mt-4 text-xs text-red-400/80">
            ⚠️ All associated data will be deleted. This includes tasks, settings, and history.
          </p>
        </div>

        <div className="flex items-center justify-between">
          <Button
            variant="outline"
            onClick={onClose}
            className="border-white/[0.06] text-white/70 hover:text-white"
          >
            Cancel
          </Button>
          <Button
            onClick={onConfirm}
            className="bg-rose-600 shadow-md transition-all hover:scale-105 hover:bg-rose-700 hover:shadow-md"
          >
            Delete User
          </Button>
        </div>
      </div>
    </div>
  )
}
