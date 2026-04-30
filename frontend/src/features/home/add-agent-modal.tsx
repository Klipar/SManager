import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
// Use native textarea to avoid missing UI primitive

type AddAgentModalProps = {
  open: boolean
  onClose: () => void
  onSave: (payload: { name: string; ip?: string; description?: string; sin?: string }) => void
}

function AddAgentModal({ open, onClose, onSave }: AddAgentModalProps) {
  const [name, setName] = useState("")
  const [ip, setIp] = useState("")
  const [description, setDescription] = useState("")
  const [sin, setSin] = useState("")

  if (!open) return null

  const handleSave = () => {
    if (!name.trim()) return
    onSave({ name: name.trim(), ip: ip.trim() || undefined, description: description.trim() || undefined, sin: sin.trim() || undefined })
    setName("")
    setIp("")
    setDescription("")
    setSin("")
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
      <div className="absolute inset-0 bg-black/60" onClick={onClose} />

      <div className="relative z-10 w-[600px] border border-white/[0.04] bg-[#0b0f13] p-8 text-white shadow-lg rounded-xl">
        <div className="mb-6">
          <h2 className="text-3xl font-medium">Add Agent</h2>
        </div>

        <div className="grid grid-cols-1 gap-3">
          <div>
            <Label className="mb-1 block text-sm text-white/80">Name</Label>
            <Input value={name} onChange={(e) => setName(e.target.value)} placeholder="Agent display name" />
          </div>

          <div>
            <Label className="mb-1 block text-sm text-white/80">IP</Label>
            <Input value={ip} onChange={(e) => setIp(e.target.value)} placeholder="192.0.2.1 or host.example.com" />
          </div>

          <div>
            <Label className="mb-1 block text-sm text-white/80">Description</Label>
            <textarea
              className="mt-2 min-h-[80px] w-full rounded-xl border border-white/10 bg-white/[0.04] px-4 py-2 text-sm text-white shadow-sm outline-none transition-colors placeholder:text-white/35 focus:border-white/20 focus:ring-2 focus:ring-white/10"
              value={description}
              onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => setDescription(e.target.value)}
              placeholder="Optional description"
            />
          </div>

          <div>
            <Label className="mb-1 block text-sm text-white/80">SIN (TLS name)</Label>
            <Input value={sin} onChange={(e) => setSin(e.target.value)} placeholder="Common Name from TLS cert" />
            <p className="mt-1 text-xs text-white/50">This should match the name issued in TLS certificates.</p>
          </div>
        </div>

        <div className="mt-8 flex items-center justify-between gap-3">
          <Button
            variant="outline"
            onClick={onClose}
            className="border-white/10 text-white/70 hover:text-white"
          >
            Cancel
          </Button>
          <Button
            className="bg-emerald-600 shadow-md transition-all hover:scale-105 hover:bg-emerald-700"
            onClick={() => {
              handleSave()
              onClose()
            }}
          >
            Save
          </Button>
        </div>
      </div>
    </div>
  )
}

export { AddAgentModal }
