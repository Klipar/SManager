import { useNavigate } from "react-router-dom";
import { AddAgentButton } from "./add-agent-button";
import { AgentList } from "./agent-list";
import { SidebarHeader } from "./sidebar-header";
import { UserFooter } from "./user-footer";
import { Separator } from "@/components/ui/separator";
import { useApp } from "@/contexts/AppContext";
import { useUser } from "@/contexts/UserContext";
import { currentUser } from "./mock-data";

type SidebarProps = {
  onOpenAddAgent?: () => void;
};

function Sidebar({ onOpenAddAgent }: SidebarProps) {
  const {
    agents,
    selectedAgentId,
    expandedAgentId,
    selectedTaskId,
    tasksByAgentId,
    setSelectedAgentId,
    setSelectedTaskId,
    setExpandedAgentId,
    setCreateTaskAgentId,
    isSidebarCollapsed,
    toggleSidebar,
    sidebarWidth,
    setSidebarWidth,
  } = useApp();

  const { user } = useUser();
  const displayUser = user?.name ? { ...currentUser, username: user.name } : currentUser;
  const navigate = useNavigate();

  const handleSelectAgent = (agentId: string) => {
    setSelectedAgentId(agentId);
    setSelectedTaskId(null);
    const newExpanded = expandedAgentId === agentId ? null : agentId;
    setExpandedAgentId(newExpanded);
  };

  const handleSelectTask = (taskId: string) => {
    setSelectedTaskId(taskId);
  };

  const handleAddTask = (agentId: string) => {
    setCreateTaskAgentId(agentId);
    navigate(`/task/new/${agentId}`);
  };

  return (
    <aside
      className="relative flex h-auto w-full shrink-0 flex-col border-b border-white/5 bg-white/[0.018] backdrop-blur-xl md:sticky md:top-0 md:h-screen md:border-b-0 md:border-r md:flex-none md:[width:var(--sidebar-width)]"
      style={{ ["--sidebar-width" as string]: `${isSidebarCollapsed ? 68 : sidebarWidth}px` } as React.CSSProperties}
    >
      <div className={isSidebarCollapsed ? "flex h-full min-h-0 flex-col items-center gap-4 px-2 py-4" : "flex h-full min-h-0 flex-col gap-4 px-3 py-4"}>
        <SidebarHeader isCollapsed={isSidebarCollapsed} onToggleCollapse={toggleSidebar} />
        <AddAgentButton isCollapsed={isSidebarCollapsed} onOpen={onOpenAddAgent} />
        <div className="flex min-h-0 flex-1 flex-col gap-2 overflow-y-auto">
          {!isSidebarCollapsed && <p className="px-1 text-[11px] font-medium uppercase tracking-[0.16em] text-white/30">Agents</p>}
          <AgentList
            agents={agents}
            selectedAgentId={selectedAgentId}
            expandedAgentId={expandedAgentId}
            isCollapsed={isSidebarCollapsed}
            selectedTaskId={selectedTaskId}
            tasksByAgentId={tasksByAgentId}
            onSelectAgent={handleSelectAgent}
            onSelectTask={handleSelectTask}
            onAddTask={handleAddTask}
          />
        </div>
        {!isSidebarCollapsed ? (
          <div className="flex w-full flex-col">
            <Separator className="bg-white/[0.04]" />
            <UserFooter user={displayUser} isCollapsed={isSidebarCollapsed} onOpenAccount={() => navigate("/account")} onOpenAdminPanel={() => navigate("/admin")} />
          </div>
        ) : (
          <UserFooter user={displayUser} isCollapsed={isSidebarCollapsed} onOpenAccount={() => navigate("/account")} onOpenAdminPanel={() => navigate("/admin")} />
        )}
      </div>
      {!isSidebarCollapsed && (
        <button
          type="button"
          onPointerDown={(e) => {
            const startX = e.clientX;
            const startWidth = sidebarWidth;
            const onMove = (me: PointerEvent) => setSidebarWidth(Math.min(320, Math.max(220, startWidth + (me.clientX - startX))));
            const onUp = () => {
              window.removeEventListener("pointermove", onMove);
              window.removeEventListener("pointerup", onUp);
            };
            window.addEventListener("pointermove", onMove);
            window.addEventListener("pointerup", onUp);
          }}
          className="absolute top-0 -right-2 h-full w-4 cursor-col-resize border-l border-transparent hover:border-cyan-400/20"
          aria-label="Resize sidebar"
        />
      )}
    </aside>
  );
}

export { Sidebar }
