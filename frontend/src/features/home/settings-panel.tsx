import React from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { sendCoreRequest, logout } from "@/lib/ws"

type UserData = { id?: number; name?: string; email?: string; is_admin?: boolean; last_update?: string | null }

type Props = {
  onClose: () => void
  userData?: UserData | null
  onUpdateUser?: (userData: UserData) => void
}

function DeleteAccountModal({ open, onClose, onConfirm }: { open: boolean; onClose: () => void; onConfirm: (password: string) => void }) {
  const [password, setPassword] = React.useState("")

  if (!open) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
      <div className="absolute inset-0 bg-black/60" onClick={onClose} />
      <Card className="relative z-10 w-[760px] border border-white/[0.04] bg-[#0b0f13] p-8 text-white shadow-lg">
        <h2 className="mb-6 text-center text-3xl">Are you sure you want to delete your account?</h2>
        <label className="block mb-2 text-lg font-medium">Enter password to confirm:</label>
        <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} className="w-full rounded-xl border border-white/10 bg-white/[0.04] px-4 py-3 text-white outline-none transition-colors placeholder:text-white/35 focus:border-white/20 focus:ring-2 focus:ring-white/10" />

        <div className="mt-6 flex items-center justify-between">
          <Button variant="outline" className="border-white/[0.06] text-white/70 hover:text-white" onClick={onClose}>Cancel</Button>
          <Button className="bg-emerald-600 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 hover:shadow-md" onClick={() => { onConfirm(password); setPassword("") }}>Confirm</Button>
        </div>
      </Card>
    </div>
  )
}

export default function SettingsPanel({ onClose, userData, onUpdateUser }: Props) {
  const [nickname, setNickname] = React.useState(userData?.name || "")
  const [password, setPassword] = React.useState("")
  const [email, setEmail] = React.useState(userData?.email || "")
  const [userId, setUserId] = React.useState<number | null>(userData?.id ?? null)
  const [lastChanged, setLastChanged] = React.useState("Never changed")
  const [modalOpen, setModalOpen] = React.useState(false)
  const [saving, setSaving] = React.useState(false)
  const [error, setError] = React.useState<string | null>(null)
  const [success, setSuccess] = React.useState<string | null>(null)

  const formatDate = (dateStr: string | null | undefined) => {
    if (!dateStr) return "Never"
    try {
      const date = new Date(dateStr)
      const now = new Date()
      const diffMs = now.getTime() - date.getTime()
      const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

      if (diffDays === 0) return "Today"
      if (diffDays === 1) return "Yesterday"
      if (diffDays < 7) return `${diffDays} days ago`
      if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`
      if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`
      return `${Math.floor(diffDays / 365)} years ago`
    } catch {
      return dateStr
    }
  }

  React.useEffect(() => {
    if (userData?.last_update) {
      setLastChanged(`Last changed ${formatDate(userData.last_update)}`)
    }
  }, [userData?.last_update])

  React.useEffect(() => {
    setNickname(userData?.name ?? "")
    setEmail(userData?.email ?? "")
    setUserId(userData?.id ?? null)
    setLastChanged(userData?.last_update ? `Last changed ${formatDate(userData.last_update)}` : "Never changed")
  }, [userData?.id, userData?.name, userData?.email, userData?.last_update])

  React.useEffect(() => {
    const token = (() => { try { return localStorage.getItem('sm_token') } catch { return null } })()
    if (!token || userId) return
    sendCoreRequest('authenticate', { token })
      .then((res) => {
        if (res && res.status === 'ok' && res.data) {
          const id = res.data.user_id
          if (typeof id === 'number') setUserId(id)
        }
      })
      .catch(() => {})
  }, [])

  function handleDeleteConfirm(pw: string) {
    console.log("delete confirmed with", pw)
    setModalOpen(false)
    if (!userId) return
    sendCoreRequest('remove-user', { id: userId })
      .then((res) => {
        if (res && res.status === 'ok') {
          // logout after account removal
          logout()
        } else {
          console.error('Failed to remove user', res)
        }
      })
      .catch((e) => console.error('WS error', e))
  }

  function handleSave() {
    setSaving(true)
    setError(null)
    setSuccess(null)
    if (!userId) {
      setError('User not identified')
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
            id: res.data?.user?.id ?? userId ?? undefined,
            name: res.data?.user?.name ?? nickname,
            email: res.data?.user?.email ?? email,
            is_admin: res.data?.user?.is_admin ?? userData?.is_admin,
            last_update: res.data?.user?.last_update ?? new Date().toISOString(),
          }

          if (onUpdateUser) {
            onUpdateUser(updatedUserData)
          }

          try {
            localStorage.setItem('sm_userData', JSON.stringify(updatedUserData))
          } catch {}

          setNickname(updatedUserData.name || '')
          setEmail(updatedUserData.email || '')
          setLastChanged(updatedUserData.last_update ? `Last changed ${formatDate(updatedUserData.last_update)}` : 'Never changed')
          setSuccess('Saved')
          setPassword('')
          onClose()
        } else {
          setError(res?.message ?? 'Failed to save')
        }
      })
      .catch((e) => setError(String(e)))
      .finally(() => setSaving(false))
  }

  return (
    <div className="w-full pt-2 pb-8">
      <div className="mb-6">
        <h1 className="text-3xl font-medium tracking-tight text-white">Edit account</h1>
      </div>

      <div className="mx-auto mt-0 max-w-3xl">
        <div className="mb-8">
          <label className="mb-3 block text-base font-medium text-white/85">Your nickname:</label>
          <Input placeholder="Enter your nickname" value={nickname} onChange={(e) => setNickname(e.target.value)} />
        </div>

        <div className="mb-8">
          <label className="mb-3 block text-base font-medium text-white/85">Your email:</label>
          <Input placeholder="Enter your email address" value={email} onChange={(e) => setEmail(e.target.value)} />
        </div>

        <div className="mb-8">
          <label className="mb-3 block text-base font-medium text-white/85">Your password:</label>
          <Input placeholder="Leave empty to keep current password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
        </div>

        <div className="mb-12 text-muted-foreground">{lastChanged}</div>

        <div className="flex items-center justify-between">
          <Button className="bg-rose-600 px-8 py-3 shadow-md transition-all hover:scale-105 hover:bg-rose-700 hover:shadow-md" size="lg" onClick={() => setModalOpen(true)}>Delete account</Button>
          <div className="flex items-center gap-3">
            {error ? <div className="text-sm text-rose-400">{error}</div> : null}
            {success ? <div className="text-sm text-emerald-400">{success}</div> : null}
            <Button disabled={saving} className="bg-emerald-600 px-8 py-3 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 hover:shadow-md" size="lg" onClick={handleSave}>{saving ? 'Saving...' : 'Save changes'}</Button>
          </div>
        </div>
      </div>

      <DeleteAccountModal open={modalOpen} onClose={() => setModalOpen(false)} onConfirm={handleDeleteConfirm} />
    </div>
  )
}
