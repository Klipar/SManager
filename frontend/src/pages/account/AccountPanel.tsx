import React from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { sendCoreRequest } from "@/lib/ws"
import { useUser } from "@/contexts/UserContext"
import type { UserData } from "@/types"

function DeleteAccountModal({ open, onClose, onConfirm, error }: { open: boolean; onClose: () => void; onConfirm: (password: string) => void; error?: string | null }) {
  const [password, setPassword] = React.useState("")

  if (!open) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
      <div className="absolute inset-0 bg-black/75" onClick={onClose} />

      <div className="relative z-10 w-[760px] rounded-xl border border-red-500/20 bg-[#0b0f13] p-8 text-white shadow-lg">
        <div className="mb-6 flex items-start gap-4">
          <div className="flex size-14 items-center justify-center rounded-xl bg-red-500/15">
            <svg className="size-7 text-red-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><path d="M12 9v4"></path><path d="M12 17h.01"></path></svg>
          </div>

          <div className="flex-1">
            <h2 className="text-3xl font-semibold">Delete Account</h2>
            <p className="mt-1 text-sm text-white/70">This action will permanently remove your account and all related data.</p>
          </div>
        </div>

        <div className="mb-6 space-y-3 rounded-xl border border-white/[0.04] bg-white/[0.03] p-4">
          <p className="text-sm text-white/80">Are you sure you want to delete your account? This action cannot be undone.</p>
          <p className="text-sm text-white/60">All associated data (tasks, settings, history) will be deleted.</p>
        </div>

        <label className="block mb-2 text-lg font-medium">Enter password to confirm:</label>
        <input
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          className="w-full rounded-xl border border-white/10 bg-white/[0.04] px-4 py-3 text-white outline-none transition-colors placeholder:text-white/35 focus:border-white/20 focus:ring-2 focus:ring-white/10"
        />

        {error ? <div className="mb-4 text-sm text-rose-400">{error}</div> : null}

        <div className="mt-6 flex items-center justify-between">
          <Button variant="outline" className="border-white/[0.06] text-white/70 hover:text-white" onClick={() => { setPassword(""); onClose(); }}>Cancel</Button>
          <Button className="bg-rose-600 py-3 shadow-md transition-all hover:scale-105 hover:bg-rose-700 hover:shadow-md" onClick={() => { onConfirm(password); }}>Delete account</Button>
        </div>
      </div>
    </div>
  )
}

export default function AccountPanel() {
  const { user, logout: contextLogout, updateUser } = useUser()

  const [nickname, setNickname] = React.useState(user?.name || "")
  const [password, setPassword] = React.useState("")
  const [email, setEmail] = React.useState(user?.email || "")
  const [userId, setUserId] = React.useState<number | null>(user?.id ?? null)
  const [modalOpen, setModalOpen] = React.useState(false)
  const [saving, setSaving] = React.useState(false)
  const [error, setError] = React.useState<string | null>(null)
  const [success, setSuccess] = React.useState<string | null>(null)
  const [emailError, setEmailError] = React.useState<string | null>(null)

  React.useEffect(() => {
    if (user) {
      setNickname(user.name || "")
      setEmail(user.email || "")
      setUserId(user.id ?? null)
    }
    setEmailError(null)
  }, [user])

  const isValidEmail = (value: string) => {
    const trimmed = value.trim()
    if (!trimmed) return false
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(trimmed)
  }

  async function handleDeleteConfirm(_pw: string) {
    setError(null)
    if (!_pw || !_pw.trim()) {
      setError('Password is required')
      return
    }
    if (!userId) return

    try {
      const res = await sendCoreRequest('remove-user', { id: userId, password: _pw })
      if (res && res.status === 'ok') {
        setModalOpen(false)
        contextLogout()
      } else {
        setError(res?.message ?? 'Failed to remove user')
      }
    } catch (e) {
      setError(String(e))
    }
  }

  function handleSave() {
    setSaving(true)
    setError(null)
    setSuccess(null)
    setEmailError(null)
    if (!userId) {
      setError('User not identified')
      setSaving(false)
      return
    }

    if (!email.trim()) {
      setEmailError('Email is required')
      setSaving(false)
      return
    }

    if (!isValidEmail(email)) {
      setEmailError('Email must be a valid address like user@example.com')
      setSaving(false)
      return
    }

    const dto: any = { id: userId }
    if (nickname) dto.name = nickname
    if (email) dto.email = email
    if (password) dto.password = password

    sendCoreRequest('update-user', dto)
      .then((res) => {
        if (res && res.status === 'ok') {
          const updatedUserData: UserData = {
            id: res.data?.user?.id ?? userId,
            name: res.data?.user?.name ?? nickname,
            email: res.data?.user?.email ?? email,
            is_admin: res.data?.user?.is_admin ?? user?.is_admin,
          }

          updateUser(updatedUserData)

          setNickname(updatedUserData.name || '')
          setEmail(updatedUserData.email || '')
          setSuccess('Saved')
          setPassword('')
        } else {
          setError(res?.message ?? 'Failed to save')
        }
      })
      .catch((e) => setError(String(e)))
      .finally(() => setSaving(false))
  }

  return (
    <>
      <div className="mb-6">
        <h1 className="text-3xl font-medium tracking-tight text-white">Edit Account</h1>
      </div>

      <div className="mx-auto mt-0 max-w-3xl">
        <div className="mb-8">
          <label className="mb-3 block text-base font-medium text-white/85">Your nickname:</label>
          <Input placeholder="Enter your nickname" value={nickname} onChange={(e) => setNickname(e.target.value)} />
        </div>

        <div className="mb-8">
          <label className="mb-3 block text-base font-medium text-white/85">Your email:</label>
          <Input
            placeholder="Enter your email address"
            value={email}
            onChange={(e) => {
              setEmail(e.target.value)
              if (emailError) setEmailError(null)
            }}
          />
          {emailError ? <div className="mt-2 text-sm text-rose-400">{emailError}</div> : null}
        </div>

        <div className="mb-8">
          <label className="mb-3 block text-base font-medium text-white/85">Your password:</label>
          <Input placeholder="Leave empty to keep current password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
        </div>

        <div className="flex items-center justify-between">
          <Button className="bg-rose-600 px-8 py-3 shadow-md transition-all hover:scale-105 hover:bg-rose-700 hover:shadow-md" size="lg" onClick={() => { setError(null); setModalOpen(true); }}>Delete account</Button>
          <div className="flex items-center gap-3">
            {error ? <div className="text-sm text-rose-400">{error}</div> : null}
            {success ? <div className="text-sm text-emerald-400">{success}</div> : null}
            <Button disabled={saving} className="bg-emerald-600 px-8 py-3 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 hover:shadow-md" size="lg" onClick={handleSave}>{saving ? 'Saving...' : 'Save changes'}</Button>
          </div>
        </div>
      </div>

      <DeleteAccountModal open={modalOpen} onClose={() => setModalOpen(false)} onConfirm={handleDeleteConfirm} error={error} />
    </>
  )
}
