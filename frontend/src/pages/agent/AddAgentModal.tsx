import { useState } from "react"
import { Button } from "@/components/ui/Button"
import { Input } from "@/components/ui/Input"
import { Label } from "@/components/ui/Label"

type AddAgentModalProps = {
  open: boolean
  onClose: () => void
  onSave: (payload: { name: string; ip: string; description?: string; port: number }) => void
}

function AddAgentModal({ open, onClose, onSave }: AddAgentModalProps) {
  const [name, setName] = useState("")
  const [ip, setIp] = useState("")
  const [description, setDescription] = useState("")
  const [port, setPort] = useState("")
  const [error, setError] = useState<string | null>(null)

  if (!open) return null

  const isValidIpOrHost = (value: string) => {
    const trimmed = value.trim()
    if (!trimmed) return false
    const ipv4Pattern = /^(25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d)){3}$/
    const ipv6Pattern = /^(([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}|([0-9A-Fa-f]{1,4}:){1,7}:|([0-9A-Fa-f]{1,4}:){1,6}:[0-9A-Fa-f]{1,4}|([0-9A-Fa-f]{1,4}:){1,5}(:[0-9A-Fa-f]{1,4}){1,2}|([0-9A-Fa-f]{1,4}:){1,4}(:[0-9A-Fa-f]{1,4}){1,3}|([0-9A-Fa-f]{1,4}:){1,3}(:[0-9A-Fa-f]{1,4}){1,4}|([0-9A-Fa-f]{1,4}:){1,2}(:[0-9A-Fa-f]{1,4}){1,5}|[0-9A-Fa-f]{1,4}:((:[0-9A-Fa-f]{1,4}){1,6})|:((:[0-9A-Fa-f]{1,4}){1,7}|:))(%.+)?$/
    const hostnamePattern = /^(?=.{1,253}$)(localhost|((?!-)[A-Za-z0-9-]{1,63}(?<!-))(\.(?!-)[A-Za-z0-9-]{1,63}(?<!-))+)$/

    return ipv4Pattern.test(trimmed) || ipv6Pattern.test(trimmed) || hostnamePattern.test(trimmed)
  }

  const validate = () => {
    const trimmedName = name.trim()
    const trimmedIp = ip.trim()
    const trimmedPort = port.trim()

    if (!trimmedName) return "Name is required"
    if (!trimmedIp) return "IP is required"
    if (!isValidIpOrHost(trimmedIp)) return "IP must be a valid IPv4, IPv6 address or hostname (e.g. example.com or localhost)"

    if (!trimmedPort) return "Port is required"

    const parsedPort = Number(trimmedPort)
    if (!Number.isInteger(parsedPort) || parsedPort < 1 || parsedPort > 65535) {
      return "Port must be a number between 1 and 65535"
    }

    return null
  }

  const handleSave = () => {
    const validationError = validate()
    if (validationError) {
      setError(validationError)
      return false
    }

    setError(null)
    onSave({
      name: name.trim(),
      ip: ip.trim(),
      description: description.trim() || undefined,
      port: parseInt(port.trim(), 10),
    })
    setName("")
    setIp("")
    setDescription("")
    setPort("")
    return true
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
      <div className="absolute inset-0 bg-black/60" onClick={onClose} />

      <div className="relative z-10 w-[600px] border border-white/[0.04] bg-[#0b0f13] p-8 text-white shadow-lg rounded-xl">
        <div className="mb-6">
          <h2 className="text-3xl font-medium">Add Agent</h2>
        </div>

        <div className="grid grid-cols-1 gap-3">
          {error ? (
            <div className="rounded-xl border border-red-500/20 bg-red-500/10 px-4 py-3 text-sm text-red-200">
              {error}
            </div>
          ) : null}

          <div>
            <Label className="mb-1 block text-sm text-white/80">Name</Label>
            <Input
              value={name}
              onChange={(e) => {
                setName(e.target.value)
                if (error) setError(null)
              }}
              placeholder="Agent display name"
            />
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
            <Label className="mb-1 block text-sm text-white/80">IP</Label>
            <Input
              value={ip}
              onChange={(e) => {
                setIp(e.target.value)
                if (error) setError(null)
              }}
              placeholder="192.0.2.1 or host.example.com"
            />
          </div>

          <div>
            <Label className="mb-1 block text-sm text-white/80">Port</Label>
            <Input
              value={port}
              onChange={(e) => {
                setPort(e.target.value)
                if (error) setError(null)
              }}
              placeholder="6767"
              inputMode="numeric"
            />
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
              const saved = handleSave()
              if (saved) onClose()
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
