import React from "react"
import TextareaAutosize from "react-textarea-autosize"
import { useNavigate } from "react-router-dom"
import { Input } from "@/components/ui/input"
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@/components/ui/dropdown-menu"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Download, Play, Trash2 } from "lucide-react"
import { useApp } from "@/contexts/AppContext"
import type { Agent, CreateTaskPayload, RestartPolicy } from "@/types"

type Props = {
  agent: Agent | null
}

type ScriptKind = "install" | "run" | "delete"
type RestartPolicyOption = RestartPolicy | "choose"

const scriptTitles: Record<ScriptKind, string> = {
  install: "Install script",
  run: "Run script",
  delete: "Delete script",
}

const defaultScriptText: Record<ScriptKind, string> = {
  install: "#!/usr/bin/env bash\nset -e\n\n# install dependencies here",
  run: "#!/usr/bin/env bash\nset -e\n\n# run the task here",
  delete: "#!/usr/bin/env bash\nset -e\n\n# cleanup here",
}

export function CreateTaskPanel({ agent }: Props) {
  const navigate = useNavigate()
  const { createTask } = useApp()
  const importInputId = React.useId()
  const dragDepthRef = React.useRef(0)
  const [name, setName] = React.useState("")
  const [description, setDescription] = React.useState("")
  const [restartPolicy, setRestartPolicy] = React.useState<RestartPolicyOption>("choose")
  const [editorOpen, setEditorOpen] = React.useState(false)
  const [editorKind, setEditorKind] = React.useState<ScriptKind>("install")
  const [editorCode, setEditorCode] = React.useState("")
  const [isDragActive, setIsDragActive] = React.useState(false)
  const [scripts, setScripts] = React.useState<Record<ScriptKind, string>>({
    install: "",
    run: "",
    delete: "",
  })
  const [isSubmitting, setIsSubmitting] = React.useState(false)
  const [errorMessage, setErrorMessage] = React.useState<string | null>(null)
  const [nameError, setNameError] = React.useState<string | null>(null)
  const [restartPolicyError, setRestartPolicyError] = React.useState<string | null>(null)

  const restartPolicyLabel =
    restartPolicy === "choose"
      ? "Choose restart policy"
      : restartPolicy

  function openEditor(kind: ScriptKind) {
    setErrorMessage(null)
    setEditorKind(kind)
    setEditorCode(scripts[kind] || defaultScriptText[kind])
    setEditorOpen(true)
  }

  function saveEditor() {
    setScripts((prev) => ({
      ...prev,
      [editorKind]: editorCode,
    }))
    setEditorOpen(false)
  }

  async function loadScriptFile(file: File) {
    if (!file.name.toLowerCase().endsWith(".sh")) {
      setErrorMessage("Only .sh files are allowed")
      return
    }

    if (!file) {
      return
    }

    try {
      const content = await file.text()
      setEditorCode(content)
      setErrorMessage(null)
    } catch {
      setErrorMessage("Failed to read the selected file")
    }
  }

  async function handleImportFile(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0]
    event.target.value = ""

    if (!file) {
      return
    }

    await loadScriptFile(file)
  }

  function handleDragEvent(event: React.DragEvent<HTMLDivElement>) {
    event.preventDefault()
    event.stopPropagation()
  }

  function handleDragEnter(event: React.DragEvent<HTMLDivElement>) {
    handleDragEvent(event)
    dragDepthRef.current += 1
    setIsDragActive(true)
  }

  function handleDragLeave(event: React.DragEvent<HTMLDivElement>) {
    handleDragEvent(event)
    dragDepthRef.current = Math.max(0, dragDepthRef.current - 1)
    if (dragDepthRef.current === 0) {
      setIsDragActive(false)
    }
  }

  async function handleDrop(event: React.DragEvent<HTMLDivElement>) {
    handleDragEvent(event)
    dragDepthRef.current = 0
    setIsDragActive(false)

    const file = event.dataTransfer.files?.[0]
    if (!file) {
      return
    }

    await loadScriptFile(file)
  }

  async function handleCreateTask() {
    if (!agent) {
      setErrorMessage("Select an agent first")
      return
    }

    if (!name.trim()) {
      setNameError("Task name is required")
      return
    }

    if (restartPolicy === "choose") {
      setRestartPolicyError("Restart policy is required")
      return
    }

    setNameError(null)
    setRestartPolicyError(null)
    setIsSubmitting(true)
    setErrorMessage(null)

    const payload: CreateTaskPayload = {
      agentId: agent.id,
      name: name.trim(),
      description: description.trim(),
      installScript: scripts.install.trim(),
      runScript: scripts.run.trim(),
      deleteScript: scripts.delete.trim(),
      restartPolicy,
    }

    try {
      const createdTaskId = await createTask(payload)
      if (!createdTaskId) {
        setErrorMessage("Failed to create task")
        return
      }

      navigate("/", { replace: true })
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <>
      <div className="mb-6">
        <h2 className="text-3xl font-medium tracking-tight text-white">
          Add new Task
          {agent && (
            <>
              {" "}for Agent:{" "}
              <span className="text-[#E53935]">
                {agent.name}
              </span>
            </>
          )}
        </h2>
        {!agent ? <p className="mt-2 text-sm text-amber-300/80">Select an agent from the sidebar before creating a task.</p> : null}
      </div>
      <div className="mb-6">
        <label className="mb-2 block font-medium">Enter task name:</label>
        <Input
          value={name}
          onChange={(e) => {
            setName(e.target.value)
            if (nameError) setNameError(null)
          }}
          placeholder="Task name"
        />
        {nameError ? <div className="mt-2 text-sm text-rose-400">{nameError}</div> : null}
      </div>

      <div className="mb-6">
        <label className="mb-2 block font-medium">Enter task description:</label>
        <TextareaAutosize
          minRows={6}
          maxRows={12}
          value={description}
          onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => setDescription(e.target.value)}
          placeholder="Task description..."
          className="flex w-full resize-none rounded-xl border border-white/10 bg-white/[0.04] px-4 py-3 text-sm text-foreground shadow-sm outline-none transition-colors placeholder:text-white/35 focus:border-white/20 focus:ring-2 focus:ring-white/10"
        />
      </div>

      <div className="mb-8 flex items-center justify-center gap-6">
        <button onClick={() => openEditor("install")} className="flex h-28 w-44 transform flex-col items-center justify-center gap-2 rounded-xl bg-violet-400 text-white shadow-md transition-all hover:scale-105 hover:shadow-lg">
          <Download />
          <div className="mt-1">Set install script</div>
          <div className="text-xs text-white/80">{scripts.install ? "Configured" : "Empty"}</div>
        </button>
        <button onClick={() => openEditor("run")} className="flex h-28 w-44 transform flex-col items-center justify-center gap-2 rounded-xl bg-emerald-400 text-white shadow-md transition-all hover:scale-105 hover:shadow-lg">
          <Play />
          <div className="mt-1">Set run script</div>
          <div className="text-xs text-white/80">{scripts.run ? "Configured" : "Empty"}</div>
        </button>
        <button onClick={() => openEditor("delete")} className="flex h-28 w-44 transform flex-col items-center justify-center gap-2 rounded-xl bg-rose-400 text-white shadow-md transition-all hover:scale-105 hover:shadow-lg">
          <Trash2 />
          <div className="mt-1">Set delete script</div>
          <div className="text-xs text-white/80">{scripts.delete ? "Configured" : "Empty"}</div>
        </button>
      </div>

      <div className="mb-4 flex justify-center">
        <div className="relative w-[36rem]">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <button className="w-full rounded-2xl border border-white/[0.04] bg-[#081017] px-4 py-3 pr-12 text-left text-white shadow-sm">
                <span className="truncate">{restartPolicyLabel}</span>
                <svg aria-hidden="true" viewBox="0 0 20 20" fill="none" className="pointer-events-none absolute right-3 top-1/2 h-5 w-5 -translate-y-1/2 text-white/75">
                  <path d="M5 7.5L10 12.5L15 7.5" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" />
                </svg>
              </button>
            </DropdownMenuTrigger>

            <DropdownMenuContent
              sideOffset={8}
              className="w-[var(--radix-dropdown-menu-trigger-width)] min-w-[var(--radix-dropdown-menu-trigger-width)] rounded-2xl border border-white/[0.04] bg-[#12161d] p-1.5 text-white shadow-[0_24px_70px_rgba(0,0,0,0.45)]"
            >
              <DropdownMenuItem disabled>Choose restart policy</DropdownMenuItem>
              <DropdownMenuItem onClick={() => {
                setRestartPolicy("no")
                if (restartPolicyError) setRestartPolicyError(null)
              }}>No</DropdownMenuItem>
              <DropdownMenuItem onClick={() => {
                setRestartPolicy("always")
                if (restartPolicyError) setRestartPolicyError(null)
              }}>Always</DropdownMenuItem>
              <DropdownMenuItem onClick={() => {
                setRestartPolicy("on-failure")
                if (restartPolicyError) setRestartPolicyError(null)
              }}>On Failure</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>

      {restartPolicyError ? <div className="mt-2 mb-4 text-sm text-rose-400">{restartPolicyError}</div> : null}

      {errorMessage ? (
        <div className="mb-4 text-sm text-rose-400">
          {errorMessage}
        </div>
      ) : null}

      <div className="mb-8 flex justify-end">
        <Button
          className="bg-emerald-600 px-8 py-3 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 hover:shadow-md"
          onClick={handleCreateTask}
          size="lg"
          disabled={isSubmitting || !agent}
        >
          {isSubmitting ? "Creating..." : "Create task"}
        </Button>
      </div>

      {editorOpen ? (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-8" onDragEnter={handleDragEnter} onDragOver={handleDragEvent} onDragLeave={handleDragLeave} onDrop={handleDrop}>
          <div className="absolute inset-0 bg-black/60" onClick={() => setEditorOpen(false)} />
          <Card className={[
            "relative z-10 w-[820px] max-h-[90vh] overflow-y-auto border bg-[#0b0f13] p-6 text-white shadow-lg transition-colors",
            isDragActive ? "border-emerald-400/70 ring-2 ring-emerald-400/30" : "border-white/[0.04]",
          ].join(" ")}>
            <div className="flex items-center justify-between">
              <h3 className="text-2xl text-white">{scriptTitles[editorKind]}</h3>
              <button aria-label="close" onClick={() => setEditorOpen(false)} className="text-white/60 hover:text-white">✕</button>
            </div>

            <div className="mt-4">
              <TextareaAutosize
                minRows={12}
                maxRows={27}
                className="mt-2 w-full resize-none rounded-xl border border-white/10 bg-white/[0.04] px-4 py-3 text-sm text-white shadow-sm outline-none transition-colors placeholder:text-white/35 focus:border-white/20 focus:ring-2 focus:ring-white/10"
                value={editorCode}
                onChange={(e) => setEditorCode(e.target.value)}
              />
            </div>

            <div className="mt-4 flex justify-between">
              <Button asChild variant="outline" className="border-white/[0.06] text-white/70 hover:text-white">
                <label htmlFor={importInputId} onClick={() => setErrorMessage(null)}>
                  Import
                </label>
              </Button>
              <Button className="bg-emerald-600 shadow-md transition-all hover:scale-105 hover:bg-emerald-700 hover:shadow-md" onClick={saveEditor}>Save</Button>
            </div>
          </Card>
        </div>
      ) : null}

      <input
        id={importInputId}
        type="file"
        accept=".sh"
        className="hidden"
        onChange={handleImportFile}
      />
    </>
  )
}

export default CreateTaskPanel
