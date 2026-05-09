import React from "react"
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@/components/ui/dropdown-menu"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
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
  const [nameError, setNameError] = React.useState<string | null>(null)
  const [emailError, setEmailError] = React.useState<string | null>(null)
  const [passwordError, setPasswordError] = React.useState<string | null>(null)

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
    setNameError(null)
    setEmailError(null)
    setPasswordError(null)
  }, [user, open])

  if (!open) return null

  const isValidEmail = (value: string) => {
    const trimmed = value.trim()
    if (!trimmed) return false
    const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    return emailPattern.test(trimmed)
  }

  const validate = () => {
    const nextNameError = !form.name.trim() ? "Name is required" : null

    let nextEmailError: string | null = null
    if (!form.email.trim()) {
      nextEmailError = "Email is required"
    } else if (!isValidEmail(form.email)) {
      nextEmailError = "Email must be a valid address like user@example.com"
    }

    let nextPasswordError: string | null = null
    if (!user && !form.password.trim()) {
      nextPasswordError = "Password is required"
    } else if (form.password && form.password.trim().length === 0) {
      nextPasswordError = "Password must have at least 1 character"
    }

    setNameError(nextNameError)
    setEmailError(nextEmailError)
    setPasswordError(nextPasswordError)

    return !nextNameError && !nextEmailError && !nextPasswordError
  }

  const handleSave = () => {
    const isValid = validate()
    if (!isValid) {
      return false
    }

    setNameError(null)
    setEmailError(null)
    setPasswordError(null)
    onSave(form)
    return true
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
      <div className="absolute inset-0 bg-black/60" onClick={onClose} />
      <div className="relative z-10 w-[600px] max-h-[90vh] overflow-y-auto rounded-xl border border-white/[0.04] bg-[#0b0f13] p-8 text-white shadow-lg">
        <div className="mb-6">
          <h2 className="text-3xl font-medium">{user ? "Edit User" : "Add new User"}</h2>
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
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <button className="w-full rounded-2xl border border-white/[0.04] bg-[#081017] px-4 py-3 pr-12 text-left text-white shadow-sm">
                    <span className="truncate">{form.role}</span>
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
                  </button>
                </DropdownMenuTrigger>

                <DropdownMenuContent
                  sideOffset={8}
                  className="w-[var(--radix-dropdown-menu-trigger-width)] min-w-[var(--radix-dropdown-menu-trigger-width)] rounded-2xl border border-white/[0.04] bg-[#12161d] p-1.5 text-white shadow-[0_24px_70px_rgba(0,0,0,0.45)]"
                >
                  <DropdownMenuItem onClick={() => setForm({ ...form, role: "user" })}>User</DropdownMenuItem>
                  <DropdownMenuItem onClick={() => setForm({ ...form, role: "admin" })}>Admin</DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </div>

          <div>
            <Label className="mb-2 block text-sm font-medium">Name</Label>
            <Input
              value={form.name}
              onChange={(e) => {
                setForm({ ...form, name: e.target.value })
                if (nameError) setNameError(null)
              }}
              placeholder="User name"
              className="border-white/10 bg-white/[0.04]"
            />
            {nameError ? <div className="mt-2 text-sm text-rose-400">{nameError}</div> : null}
          </div>

          <div>
            <Label className="mb-2 block text-sm font-medium">Email</Label>
            <Input
              type="email"
              value={form.email}
              onChange={(e) => {
                setForm({ ...form, email: e.target.value })
                if (emailError) setEmailError(null)
              }}
              placeholder="user@email.com"
              className="border-white/10 bg-white/[0.04]"
            />
            {emailError ? <div className="mt-2 text-sm text-rose-400">{emailError}</div> : null}
          </div>

          <div>
            <Label className="mb-2 block text-sm font-medium">
              {user ? "New Password" : "Password"}
            </Label>
            <Input
              type="password"
              value={form.password}
              onChange={(e) => {
                setForm({ ...form, password: e.target.value })
                if (passwordError) setPasswordError(null)
              }}
              placeholder={user ? "Leave empty to keep current" : "Enter password"}
              className="border-white/10 bg-white/[0.04]"
            />
            {passwordError ? <div className="mt-2 text-sm text-rose-400">{passwordError}</div> : null}
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
            disabled={isSaving}
            className="bg-emerald-600 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 disabled:opacity-50 disabled:hover:scale-100"
            onClick={() => {
              const saved = handleSave()
              if (saved) onClose()
            }}
          >
            {isSaving ? "Saving..." : user ? "Save user" : "Create user"}
          </Button>
        </div>
      </div>
    </div>
  )
}
