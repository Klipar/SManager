import { Outlet } from "react-router-dom";
import { Sidebar } from "../shared/sidebar/Sidebar";
import { AddAgentModal } from "../agent/AddAgentModal";
import AddTaskModal from "../task/AddTaskModal";
import { useApp } from "@/contexts/AppContext";
import { useState } from "react";

function Layout() {
  const { addAgent, agents, createTaskModalOpen, setCreateTaskModalOpen, createTaskAgentId, createTask } = useApp();
  const [showAddAgent, setShowAddAgent] = useState(false);

  const handleSaveAgent = (payload: any) => {
    addAgent(payload).finally(() => setShowAddAgent(false));
  };

  const handleSaveTask = (payload: any) => {
    createTask(payload).finally(() => setCreateTaskModalOpen(false));
  };

  const agent = createTaskAgentId ? (agents.find(a => a.id === createTaskAgentId) ?? null) : null;

  return (
    <div className="flex min-h-screen w-full flex-col md:flex-row">
      <Sidebar onOpenAddAgent={() => setShowAddAgent(true)} />
      <main className="relative flex-1 bg-[#070b10]">
        <div
          aria-hidden="true"
          className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(34,211,238,0.08),transparent_30%),linear-gradient(135deg,rgba(255,255,255,0.02),transparent_40%)]"
        />
        <div className="relative h-full w-full py-5 pl-0 pr-5 sm:pl-1 sm:pr-6 md:py-8 md:pl-2 md:pr-10">
          <div className="mx-auto w-full max-w-7xl px-8">
            <Outlet />
          </div>
        </div>
      </main>
      <AddAgentModal open={showAddAgent} onClose={() => setShowAddAgent(false)} onSave={handleSaveAgent} />
      <AddTaskModal open={createTaskModalOpen} agent={agent} onClose={() => setCreateTaskModalOpen(false)} onSave={handleSaveTask} />
    </div>
  );
}

export { Layout }
