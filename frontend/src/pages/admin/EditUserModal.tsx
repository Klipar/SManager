import React from "react"
import { Button } from "@/components/ui/Button"
import { Input } from "@/components/ui/Input"
import { Label } from "@/components/ui/Label"
import type { AdminUser, EditUserForm } from "@/types"

interface EditUserModalProps {
  open: boolean
  user: AdminUser | null
  onClose: () => void
  onSave: (data: EditUserForm) => void
  isSaving?: boolean
}

export function EditUserModal({ open, user, onClose, onSave, isSaving = false }: EditUserModalProps) {
  const [form, setForm] = React.useState<EditUserForm>({
    name: "",
    email: "",
    password: "",
    role: "user",
  })

  React.useEffect(() => {
    if (user) {
      setForm({
        name: user.name,
        email: user.email,
        password: "",
        role: user.role,
      })
    } else {
      setForm({
        name: "",
        email: "",
        password: "",
        role: "user",
      })
    }
  }, [user, open])

  if (!open) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
      <div className="absolute inset-0 bg-black/60" onClick={onClose} />
      <div className="relative z-10 w-[600px] border border-white/[0.04] bg-[#0b0f13] p-8 text-white shadow-lg rounded-xl">
        <div className="mb-6">
          <h2 className="text-3xl font-medium">{user ? "Edit User" : "Add User"}</h2>
        </div>

        {user && (
          <div className="mb-6 flex items-center gap-4 pb-6 border-b border-white/10">
            <div className="flex size-12 items-center justify-center rounded-lg bg-white/10">
              <span className="text-lg font-medium text-white/80">
                {user.name
                  .split(" ")
                  .map((part) => part[0])
                  .join("")
                  .toUpperCase()}
              </span>
            </div>
            <div>
              <p className="text-sm font-medium text-white/90">{user.name}</p>
              <p className="text-xs text-white/50">Created at {user.createdAt}</p>
            </div>
          </div>
        )}

        <div className="space-y-6">
          <div>
            <Label className="mb-2 block text-sm font-medium">Role</Label>
            <div className="relative">
              <select
                value={form.role}
                onChange={(e) => setForm({ ...form, role: e.target.value as any })}
                className="w-full appearance-none rounded-full border border-white/[0.04] bg-[#081017] px-4 py-3 pr-12 text-white shadow-sm outline-none transition-colors focus:border-white/20 focus:ring-2 focus:ring-white/10"
              >
                <option value="user">User</option>
                <option value="admin">Admin</option>
              </select>
              <svg
                aria-hidden="true"
                viewBox="0 0 20 20"
                fill="none"
                className="pointer-events-none absolute right-3 top-1/2 h-5 w-5 -translate-y-1/2 text-white/75"
              >
                <path
                  d="M5 7.5L10 12.5L15 7.5"
                  stroke="currentColor"
                  strokeWidth="1.6"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
              </svg>
            </div>
          </div>

          <div>
            <Label className="mb-2 block text-sm font-medium">Name</Label>
            <Input
              value={form.name}
              onChange={(e) => setForm({ ...form, name: e.target.value })}
              placeholder="User name"
              className="border-white/10 bg-white/[0.04]"
            />
          </div>

          <div>
            <Label className="mb-2 block text-sm font-medium">Email</Label>
            <Input
              type="email"
              value={form.email}
              onChange={(e) => setForm({ ...form, email: e.target.value })}
              placeholder="user@email.com"
              className="border-white/10 bg-white/[0.04]"
            />
          </div>

          <div>
            <Label className="mb-2 block text-sm font-medium">
              {user ? "New Password" : "Password"}
            </Label>
            <Input
              type="password"
              value={form.password}
              onChange={(e) => setForm({ ...form, password: e.target.value })}
              placeholder={user ? "Leave empty to keep current" : "Enter password"}
              className="border-white/10 bg-white/[0.04]"
            />
          </div>
        </div>

        <div className="mt-8 flex items-center justify-between gap-3">
          <Button
            variant="outline"
            onClick={onClose}
            disabled={isSaving}
            className="border-white/10 text-white/70 hover:text-white disabled:opacity-50"
          >
            Cancel
          </Button>
          <Button
            disabled={isSaving || !form.name || !form.email}
            className="bg-emerald-600 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 disabled:opacity-50 disabled:hover:scale-100"
            onClick={() => {
              onSave(form)
              onClose()
            }}
          >
            {isSaving ? "Saving..." : "Save"}
          </Button>
        </div>
      </div>
    </div>
  )
}
