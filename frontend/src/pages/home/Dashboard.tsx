import { useApp } from "@/contexts/AppContext";
import { TaskWorkspace } from "../agent/TaskWorkspace";
import { EmptyState } from "../agent/EmptyState";

function Dashboard() {
  const { selectedAgentId, agents, selectedTaskId, selectedLogId, setSelectedLogId, tasksByAgentId } = useApp();
  const agent = agents.find((a) => a.id === selectedAgentId) ?? null;
  const tasks = agent ? (tasksByAgentId[agent.id] ?? []) : [];
  const task = tasks.find((t) => t.id === selectedTaskId) ?? null;
  const log = task?.logs?.find((l) => l.id === selectedLogId) ?? null;

  return (
    <div className="flex h-full flex-col justify-center">
      {agent ? (
        <TaskWorkspace
          key={agent.id}
          agent={agent}
          selectedTask={task}
          selectedLog={log}
          onSelectLog={setSelectedLogId}
        />
      ) : (
        <EmptyState />
      )}
    </div>
  );
}

export { Dashboard }
