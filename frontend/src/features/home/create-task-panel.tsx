import React from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Download, Play, Trash2 } from "lucide-react"

import type { Agent } from "@/types"

type Props = {
  agent: Agent | null
}

export function CreateTaskPanel({ agent }: Props) {
  const [name, setName] = React.useState("")
  const [description, setDescription] = React.useState("")
  const [restartPolicy, setRestartPolicy] = React.useState("no")
  const [editorOpen, setEditorOpen] = React.useState(false)
  const [editorCode, setEditorCode] = React.useState("")

  function openEditor(kind: string) {
    setEditorCode(`# ${kind} script\n# write bash here`)
    setEditorOpen(true)
  }

  return (
    <>
      <div className="mb-6">
        <h2 className="text-3xl font-medium tracking-tight text-white">Create task{agent ? ` — ${agent.name}` : ""}</h2>
      </div>
      <div className="mb-6">
        <label className="mb-2 block font-medium">Enter task name:</label>
        <Input value={name} onChange={(e) => setName(e.target.value)} placeholder="Task name" />
      </div>

      <div className="mb-6">
        <label className="mb-2 block font-medium">Enter task description:</label>
        <textarea
          value={description}
          onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => setDescription(e.target.value)}
          rows={6}
          placeholder="Task description..."
          className="flex w-full rounded-xl border border-white/10 bg-white/[0.04] px-4 py-3 text-sm text-foreground shadow-sm outline-none transition-colors placeholder:text-white/35 focus:border-white/20 focus:ring-2 focus:ring-white/10"
        />
      </div>

      <div className="mb-8 flex items-center justify-center gap-6">
        <button onClick={() => openEditor('install')} className="flex h-28 w-44 transform flex-col items-center justify-center gap-2 rounded-xl bg-violet-400 text-white shadow-md transition-all hover:scale-105 hover:shadow-lg">
          <Download />
          <div className="mt-1">Set install script</div>
        </button>
        <button onClick={() => openEditor('run')} className="flex h-28 w-44 transform flex-col items-center justify-center gap-2 rounded-xl bg-emerald-400 text-white shadow-md transition-all hover:scale-105 hover:shadow-lg">
          <Play />
          <div className="mt-1">Set run script</div>
        </button>
        <button onClick={() => openEditor('delete')} className="flex h-28 w-44 transform flex-col items-center justify-center gap-2 rounded-xl bg-rose-400 text-white shadow-md transition-all hover:scale-105 hover:shadow-lg">
          <Trash2 />
          <div className="mt-1">Set delete script</div>
        </button>
      </div>

      <div className="mb-4 flex justify-center">
        <div className="relative w-[36rem]">
          <select value={restartPolicy} onChange={(e) => setRestartPolicy(e.target.value)} className="w-full appearance-none rounded-full border border-white/[0.04] bg-[#081017] px-4 py-3 pr-12 text-white shadow-sm">
            <option value="">Choose restart policy</option>
            <option value="no">No</option>
            <option value="always">Always</option>
            <option value="on-failure">On Failure</option>
          </select>
          <svg aria-hidden="true" viewBox="0 0 20 20" fill="none" className="pointer-events-none absolute right-3 top-1/2 h-5 w-5 -translate-y-1/2 text-white/75">
            <path d="M5 7.5L10 12.5L15 7.5" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" />
          </svg>
        </div>
      </div>

      <div className="mb-8 flex justify-end">
        <Button className="bg-emerald-600 px-8 py-3 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 hover:shadow-md" onClick={() => { /* TODO: create task action */ }} size="lg">Create task</Button>
      </div>

      {editorOpen ? (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
          <div className="absolute inset-0 bg-black/60" onClick={() => setEditorOpen(false)} />
          <Card className="relative z-10 w-[820px] border border-white/[0.04] bg-[#0b0f13] p-6 text-white shadow-lg">
            <div className="flex items-center justify-between">
              <h3 className="text-2xl text-white">Run script</h3>
              <button aria-label="close" onClick={() => setEditorOpen(false)} className="text-white/60 hover:text-white">✕</button>
            </div>

            <div className="mt-4">
              <label className="block text-lg font-medium text-white">Code:</label>
              <textarea className="mt-2 min-h-[240px] w-full rounded-xl border border-white/10 bg-white/[0.04] px-4 py-3 text-sm text-white shadow-sm outline-none transition-colors placeholder:text-white/35 focus:border-white/20 focus:ring-2 focus:ring-white/10" value={editorCode} onChange={(e) => setEditorCode(e.target.value)} />
            </div>

            <div className="mt-4 flex justify-between">
              <Button variant="outline" className="border-white/[0.06] text-white/70 hover:text-white">Import</Button>
              <Button className="bg-emerald-600 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 hover:shadow-md" onClick={() => { console.log('save', editorCode); setEditorOpen(false) }}>Save</Button>
            </div>
          </Card>
        </div>
      ) : null}
    </>
  )
}

export default CreateTaskPanel
