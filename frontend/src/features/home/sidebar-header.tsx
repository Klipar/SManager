import { ChevronLeft, ChevronRight, Orbit } from "lucide-react"

type SidebarHeaderProps = {
  isCollapsed: boolean
  onToggleCollapse: () => void
}

function SidebarHeader({ isCollapsed, onToggleCollapse }: SidebarHeaderProps) {
  return (
    <div className={isCollapsed ? "flex justify-center px-1 py-1" : "flex items-center justify-between gap-3 px-1 py-1"}>
      {isCollapsed ? (
        <button
          type="button"
          onClick={onToggleCollapse}
          className="group relative flex size-10 items-center justify-center rounded-2xl border border-white/5 bg-white/[0.03] text-cyan-200 transition-colors hover:bg-white/[0.05] hover:text-white"
          aria-label="Expand sidebar"
        >
          <Orbit className="size-5 transition-opacity duration-200 group-hover:opacity-0" aria-hidden="true" />
          <ChevronRight className="absolute size-4 opacity-0 transition-opacity duration-200 group-hover:opacity-100" aria-hidden="true" />
        </button>
      ) : (
        <>
          <div className="flex items-center gap-3">
            <div className="flex size-9 items-center justify-center rounded-2xl border border-white/5 bg-white/[0.03] text-cyan-200">
              <Orbit className="size-5" aria-hidden="true" />
            </div>
            <p className="text-[15px] font-semibold tracking-tight text-white">SManager</p>
          </div>

          <button
            type="button"
            onClick={onToggleCollapse}
            className="flex size-8 items-center justify-center rounded-xl border border-white/5 bg-white/[0.03] text-white/45 transition-colors hover:bg-white/[0.06] hover:text-white"
            aria-label="Collapse sidebar"
          >
            <ChevronLeft className="size-4" aria-hidden="true" />
          </button>
        </>
      )}
    </div>
  )
}

export { SidebarHeader }