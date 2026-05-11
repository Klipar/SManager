import { ChevronLeft, ChevronRight } from "lucide-react"
import { Button } from "@/components/ui/button"
import SManagerLogo from "@/assets/SManagerLogoLight.svg"

type SidebarHeaderProps = {
  isCollapsed: boolean
  onToggleCollapse: () => void
}

function SidebarHeader({ isCollapsed, onToggleCollapse }: SidebarHeaderProps) {
  return (
    <div className={isCollapsed ? "flex justify-center px-1 py-1" : "flex items-center justify-between gap-3 px-1 py-1"}>
      {isCollapsed ? (
        <Button
          type="button"
          variant="ghost"
          size="icon"
          onClick={onToggleCollapse}
          className="group relative size-8 p-0 hover:opacity-80"
          aria-label="Expand sidebar"
        >
          <img src={SManagerLogo} alt="SManager" className="size-6 transition-opacity duration-200 group-hover:opacity-0" aria-hidden="true" />
          <ChevronRight className="absolute size-4 opacity-0 transition-opacity duration-200 group-hover:opacity-100" aria-hidden="true" />
        </Button>
      ) : (
        <>
          <div className="flex items-center gap-3">
            <img src={SManagerLogo} alt="SManager" className="size-6" aria-hidden="true" />
            <p className="text-xl font-semibold tracking-tight">
              <span className="text-[#E53935]">S</span>
              <span className="text-[#EAEAEA]">Manager</span>
            </p>
          </div>

          <Button
            type="button"
            variant="ghost"
            size="icon-sm"
            onClick={onToggleCollapse}
            className="size-8 rounded-xl border border-white/5 bg-white/[0.03] p-0 text-white/45 hover:bg-white/[0.06] hover:text-white"
            aria-label="Collapse sidebar"
          >
            <ChevronLeft className="size-4" aria-hidden="true" />
          </Button>
        </>
      )}
    </div>
  )
}

export { SidebarHeader }
