import React from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { AlertTriangle } from "lucide-react"
import type { AdminUser } from "@/types"

interface DeleteUserModalProps {
  open: boolean
  user: AdminUser | null
  onClose: () => void
  onConfirm: (password?: string) => void
  isDeleting?: boolean
  requirePassword?: boolean
  error?: string | null
}

export function DeleteUserModal({
  open,
  user,
  onClose,
  onConfirm,
  isDeleting = false,
  requirePassword = false,
  error = null,
}: DeleteUserModalProps) {
  const [password, setPassword] = React.useState("")
  const [localError, setLocalError] = React.useState<string | null>(null)

  React.useEffect(() => {
    if (open) {
      setPassword("")
      setLocalError(null)
    }
  }, [open, user])

  if (!open || !user) return null

  const handleConfirm = () => {
    if (requirePassword && !password.trim()) {
      setLocalError("Password is required")
      return
    }

    setLocalError(null)
    onConfirm(requirePassword ? password : undefined)
  }

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
            ⚠️ All account data will be deleted.
          </p>
        </div>

        {requirePassword ? (
          <div className="mb-6 space-y-2">
            <Label htmlFor="delete-user-password" className="block text-sm font-medium text-white/85">
              Enter password to confirm
            </Label>
            <Input
              id="delete-user-password"
              type="password"
              value={password}
              onChange={(event) => {
                setPassword(event.target.value)
                if (localError) setLocalError(null)
              }}
              placeholder="Password"
              className="border-white/10 bg-white/[0.04]"
            />
            {localError ? <p className="mt-2 text-sm text-rose-400">{localError}</p> : null}
            {error ? <p className="mt-2 text-sm text-rose-400">{error}</p> : null}
          </div>
        ) : null}

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
            onClick={handleConfirm}
            disabled={isDeleting}
            className="bg-rose-600 shadow-md transition-all hover:scale-105 hover:bg-rose-700 hover:shadow-md disabled:opacity-50 disabled:hover:scale-100"
          >
            {isDeleting ? "Deleting..." : "Delete User"}
          </Button>
        </div>
      </div>
    </div>
  )
}
