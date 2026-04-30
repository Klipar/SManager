import React from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"

type Props = {
  onClose: () => void
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

export default function SettingsPanel({ onClose }: Props) {
  const [nickname, setNickname] = React.useState("")
  const [password, setPassword] = React.useState("")
  const [email, setEmail] = React.useState("")
  const [lastChanged] = React.useState("Last changed 2 days ago")
  const [modalOpen, setModalOpen] = React.useState(false)

  function handleDeleteConfirm(pw: string) {
    console.log("delete confirmed with", pw)
    setModalOpen(false)
  }

  return (
    <div className="w-full pt-2 pb-8">
      <div className="mb-6 flex items-center justify-between gap-4">
        <h1 className="text-3xl font-medium tracking-tight text-white">Edit account</h1>
        <button onClick={onClose} aria-label="close" className="text-muted-foreground">✕</button>
      </div>

      <div className="mx-auto mt-0 max-w-3xl">
        <div className="mb-8">
          <label className="mb-3 block text-base font-medium text-white/85">Your nickname:</label>
          <Input placeholder="Your nickname" value={nickname} onChange={(e) => setNickname(e.target.value)} />
        </div>

        <div className="mb-8">
          <label className="mb-3 block text-base font-medium text-white/85">Your password:</label>
          <Input placeholder="Your password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
        </div>

        <div className="mb-8">
          <label className="mb-3 block text-base font-medium text-white/85">Your email:</label>
          <Input placeholder="Your@email.here" value={email} onChange={(e) => setEmail(e.target.value)} />
        </div>

        <div className="mb-12 text-muted-foreground">{lastChanged}</div>

        <div className="flex items-center justify-between">
          <Button className="bg-rose-600 px-8 py-3 shadow-md transition-all hover:scale-105 hover:bg-rose-700 hover:shadow-md" size="lg" onClick={() => setModalOpen(true)}>Delete account</Button>
          <Button className="bg-emerald-600 px-8 py-3 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 hover:shadow-md" size="lg" onClick={onClose}>Save changes</Button>
        </div>
      </div>

      <DeleteAccountModal open={modalOpen} onClose={() => setModalOpen(false)} onConfirm={handleDeleteConfirm} />
    </div>
  )
}
