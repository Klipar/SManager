import React, { createContext, useContext, useEffect, useState, useCallback } from "react";
import { sendCoreRequest, subscribeCoreRequest } from "@/lib/ws";
import type { Agent, CreateTaskPayload, ScriptType, Task, TaskLog } from "@/types";
import {
  buildTaskDescription,
  buildTaskName,
  buildTaskStatus,
  generateTaskId,
  loadTaskStore,
  normalizeLog,
  saveTaskStore,
  type StoredTaskRecord,
} from "@/lib/taskStore";

type AppContextType = {
  agents: Agent[];
  isLoading: boolean;
  selectedAgentId: string | null;
  selectedTaskId: string | null;
  selectedLogId: string | null;
  expandedAgentId: string | null;
  createTaskAgentId: string | null;
  createTaskModalOpen: boolean;
  isSidebarCollapsed: boolean;
  sidebarWidth: number;
  tasksByAgentId: Record<string, Task[]>;
  setSelectedAgentId: (id: string | null) => void;
  setSelectedTaskId: (id: string | null) => void;
  setSelectedLogId: (id: string | null) => void;
  setExpandedAgentId: (id: string | null) => void;
  setCreateTaskAgentId: (id: string | null) => void;
  setCreateTaskModalOpen: (open: boolean) => void;
  toggleSidebar: () => void;
  setSidebarWidth: (width: number) => void;
  addAgent: (payload: any) => Promise<void>;
  createTask: (payload: CreateTaskPayload) => Promise<string | null>;
  saveTaskRecord: (taskId: string, updates: Partial<Omit<StoredTaskRecord, "id">>) => Promise<boolean>;
  removeTaskRecord: (taskId: string) => Promise<boolean>;
  runTask: (taskId: string, scriptType: ScriptType) => Promise<boolean>;
  stopTask: (taskId: string) => Promise<boolean>;
  refreshAgents: () => void;
  refreshTasks: () => Promise<void>;
};

const AppContext = createContext<AppContextType | undefined>(undefined);

const VIEW_STATE_KEY = "sm_homeViewState";

function loadViewState() {
  try {
    const raw = localStorage.getItem(VIEW_STATE_KEY);
    if (!raw) return {};
    return JSON.parse(raw);
  } catch {
    return {};
  }
}

function saveViewState(state: Partial<AppContextType>) {
  const current = loadViewState();
  const next = { ...current, ...state };
  try {
    localStorage.setItem(VIEW_STATE_KEY, JSON.stringify(next));
  } catch {}
}

export function AppProvider({ children }: { children: React.ReactNode }) {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [selectedAgentId, setSelectedAgentId] = useState<string | null>(null);
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null);
  const [selectedLogId, setSelectedLogId] = useState<string | null>(null);
  const [expandedAgentId, setExpandedAgentId] = useState<string | null>(null);
  const [createTaskAgentId, setCreateTaskAgentId] = useState<string | null>(null);
  const [createTaskModalOpen, setCreateTaskModalOpen] = useState(false);
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
  const [sidebarWidth, setSidebarWidth] = useState(228);
  const [taskStore, setTaskStore] = useState<Record<string, StoredTaskRecord>>(() => loadTaskStore());
  const [tasksByAgentId, setTasksByAgentId] = useState<Record<string, Task[]>>({});

  useEffect(() => {
    saveTaskStore(taskStore);
  }, [taskStore]);

  useEffect(() => {
    const saved = loadViewState();
    if (saved.selectedAgentId) setSelectedAgentId(saved.selectedAgentId);
    if (saved.expandedAgentId) setExpandedAgentId(saved.expandedAgentId);
    if (saved.selectedTaskId) setSelectedTaskId(saved.selectedTaskId);
    if (saved.selectedLogId) setSelectedLogId(saved.selectedLogId);
    if (saved.createTaskAgentId) setCreateTaskAgentId(saved.createTaskAgentId);
    if (saved.createTaskModalOpen !== undefined) setCreateTaskModalOpen(saved.createTaskModalOpen);
    if (saved.isSidebarCollapsed !== undefined) setIsSidebarCollapsed(saved.isSidebarCollapsed);
    if (saved.sidebarWidth) setSidebarWidth(saved.sidebarWidth);
  }, []);

  useEffect(() => {
    saveViewState({
      selectedAgentId,
      expandedAgentId,
      selectedTaskId,
      selectedLogId,
      createTaskAgentId,
      createTaskModalOpen,
      isSidebarCollapsed,
      sidebarWidth,
    });
  }, [selectedAgentId, expandedAgentId, selectedTaskId, selectedLogId, createTaskAgentId, createTaskModalOpen, isSidebarCollapsed, sidebarWidth]);

  const refreshAgents = useCallback(async () => {
    try {
      const res = await sendCoreRequest("get-all-agents", null);
      if (res?.status === "ok") {
        const rawAgents = res.data?.agents ?? [];
        const normalized = rawAgents.map((a: any, idx: number) => ({
          id: String(a?.id ?? a?._id ?? a?.uuid ?? `agent-${idx}`),
          name: a?.name ?? `Unnamed ${idx + 1}`,
          status: a?.status ?? "offline",
          ip: a?.ip,
          description: a?.description,
          port: a?.port ?? a?.sin,
        }));
        setAgents(normalized);
      } else {
        setAgents([]);
      }
    } catch {
      setAgents([]);
    }
  }, []);

  const refreshTasks = useCallback(async (taskStoreSnapshot: Record<string, StoredTaskRecord> = taskStore) => {
    try {
      const agentsRes = await sendCoreRequest("get-all-agents", null);
      const agentList = agentsRes?.status === "ok" ? (agentsRes.data?.agents ?? []) : [];

      const runsByTaskId = new Map<string, TaskLog[]>();
      const groupedTasks: Record<string, Task[]> = {};

      await Promise.all(agentList.map(async (a: any) => {
        const agentId = Number(a?.id ?? a?._id ?? a?.uuid ?? NaN);
        if (Number.isNaN(agentId)) return;

        const [tasksRes, runsRes] = await Promise.all([
          sendCoreRequest("get-all-tasks", { agent_id: agentId }),
          sendCoreRequest("get-runs", { agent_id: agentId }),
        ]);

        const rawTasks = tasksRes?.status === "ok" ? (tasksRes.data?.tasks ?? []) : [];
        const rawRuns = runsRes?.status === "ok" ? (runsRes.data?.runs ?? []) : [];

        for (const rawRun of rawRuns) {
          const taskId = rawRun?.task_id;
          if (taskId === null || taskId === undefined) continue;

          const normalizedLog = normalizeLog(rawRun);
          const key = String(taskId);
          const existing = runsByTaskId.get(key) ?? [];
          existing.push(normalizedLog);
          runsByTaskId.set(key, existing);
        }

        for (const rawTask of rawTasks) {
          const taskId = String(rawTask?.id ?? "");
          const agentIdStr = String(agentId);
          if (!taskId) continue;

          const storedTask = taskStoreSnapshot[taskId];
          const logs = runsByTaskId.get(taskId) ?? [];

          const task: Task = {
            id: taskId,
            name: rawTask?.name ?? buildTaskName(taskId, storedTask),
            scriptType: "run",
            status: (rawTask?.status as Task["status"] | undefined) ?? buildTaskStatus(logs, Boolean(storedTask)),
            description: rawTask?.description ?? buildTaskDescription(taskId, storedTask),
            createdByCore: storedTask?.createdByCore ?? "Agent",
            restartPolicy: (rawTask?.restart_policy as Task["restartPolicy"] | undefined) ?? storedTask?.restartPolicy ?? "no",
            installScript: rawTask?.install_script ?? storedTask?.installScript ?? "",
            runScript: rawTask?.run_script ?? storedTask?.runScript ?? "",
            deleteScript: rawTask?.delete_script ?? storedTask?.deleteScript ?? "",
            logs,
          };

          if (!groupedTasks[agentIdStr]) groupedTasks[agentIdStr] = [];
          groupedTasks[agentIdStr].push(task);
        }
      }));

      Object.values(groupedTasks).forEach((tasks) => {
        tasks.sort((left, right) => Number(right.id) - Number(left.id));
      });

      setTasksByAgentId(groupedTasks);
    } catch (e) {
      console.error("refreshTasks error:", e);
      setTasksByAgentId({});
    }
  }, [taskStore]);

  const addAgent = useCallback(async (payload: any) => {
    const res = await sendCoreRequest("new-agent", payload);
    if (res?.status === "ok") {
      await refreshAgents();
    }
  }, [refreshAgents]);

  const createTask = useCallback(async (payload: CreateTaskPayload) => {
    const agentId = Number.parseInt(payload.agentId, 10);
    if (Number.isNaN(agentId)) {
      return null;
    }

    const existingIds = new Set<number>(Object.keys(taskStore).map((taskId) => Number(taskId)).filter((taskId) => Number.isFinite(taskId)));
    const taskId = generateTaskId(existingIds);

    const taskRecord: StoredTaskRecord = {
      id: String(taskId),
      agentId: payload.agentId,
      name: payload.name.trim() || `Task ${taskId}`,
      description: payload.description.trim(),
      installScript: payload.installScript,
      runScript: payload.runScript,
      deleteScript: payload.deleteScript,
      restartPolicy: payload.restartPolicy,
      createdByCore: "Core",
    };

    const res = await sendCoreRequest("new-task", {
      id: taskId,
      agent_id: agentId,
      name: taskRecord.name,
      description: taskRecord.description,
      install_script: taskRecord.installScript,
      run_script: taskRecord.runScript,
      delete_script: taskRecord.deleteScript,
      restart_policy: taskRecord.restartPolicy,
    });

    if (res?.status !== "ok") {
      return null;
    }

    const nextTaskStore = {
      ...taskStore,
      [taskRecord.id]: taskRecord,
    };

    setTaskStore(nextTaskStore);

    setSelectedAgentId(payload.agentId);
    setExpandedAgentId(payload.agentId);
    setSelectedTaskId(taskRecord.id);
    setSelectedLogId(null);
    setCreateTaskAgentId(payload.agentId);
    setCreateTaskModalOpen(false);

    await refreshTasks(nextTaskStore);
    return taskRecord.id;
  }, [refreshTasks, taskStore]);

  const saveTaskRecord = useCallback(async (taskId: string, updates: Partial<Omit<StoredTaskRecord, "id">>) => {
    const numericTaskId = Number.parseInt(taskId, 10);
    if (Number.isNaN(numericTaskId)) {
      return false;
    }

    const currentTask = taskStore[taskId];

    let fallbackAgentId: string | null = null;
    let fallbackTask: Task | null = null;
    for (const [agentId, tasks] of Object.entries(tasksByAgentId)) {
      const found = tasks.find((task) => task.id === taskId);
      if (found) {
        fallbackAgentId = agentId;
        fallbackTask = found;
        break;
      }
    }

    const resolvedAgentId = updates.agentId ?? currentTask?.agentId ?? fallbackAgentId;
    if (!resolvedAgentId) {
      return false;
    }

    const numericAgentId = Number.parseInt(resolvedAgentId, 10);
    if (Number.isNaN(numericAgentId)) {
      return false;
    }

    const nextTask: StoredTaskRecord = {
      id: taskId,
      agentId: resolvedAgentId,
      name: updates.name ?? currentTask?.name ?? fallbackTask?.name ?? `Task ${taskId}`,
      description: updates.description ?? currentTask?.description ?? fallbackTask?.description ?? "",
      installScript: updates.installScript ?? currentTask?.installScript ?? fallbackTask?.installScript ?? "",
      runScript: updates.runScript ?? currentTask?.runScript ?? fallbackTask?.runScript ?? "",
      deleteScript: updates.deleteScript ?? currentTask?.deleteScript ?? fallbackTask?.deleteScript ?? "",
      restartPolicy: updates.restartPolicy ?? currentTask?.restartPolicy ?? fallbackTask?.restartPolicy ?? "no",
      createdByCore: currentTask?.createdByCore ?? fallbackTask?.createdByCore ?? "Agent",
    };

    const res = await sendCoreRequest("update-task", {
      agent_id: numericAgentId,
      id: numericTaskId,
      name: nextTask.name,
      description: nextTask.description || null,
      install_script: nextTask.installScript || null,
      run_script: nextTask.runScript || null,
      delete_script: nextTask.deleteScript || null,
      restart_policy: nextTask.restartPolicy,
      status: fallbackTask?.status ?? "stopped",
    });

    if (res?.status !== "ok") {
      return false;
    }

    const nextTaskStore = {
      ...taskStore,
      [taskId]: nextTask,
    };

    setTaskStore(nextTaskStore);
    await refreshTasks(nextTaskStore);
    return true;
  }, [refreshTasks, taskStore, tasksByAgentId]);

  const removeTaskRecord = useCallback(async (taskId: string) => {
    const nextTaskStore = { ...taskStore };
    delete nextTaskStore[taskId];

    setTaskStore(nextTaskStore);
    await refreshTasks(nextTaskStore);
    return Boolean(taskStore[taskId]);
  }, [refreshTasks, taskStore]);

  const runTask = useCallback(async (taskId: string, scriptType: ScriptType) => {
    const numericTaskId = Number.parseInt(taskId, 10);
    if (Number.isNaN(numericTaskId)) {
      return false;
    }

    try {
      let agentIdStr: string | undefined = taskStore[taskId]?.agentId;
      if (!agentIdStr) {
        for (const [aid, tasks] of Object.entries(tasksByAgentId)) {
          if (tasks.some((t) => t.id === taskId)) {
            agentIdStr = aid;
            break;
          }
        }
      }

      if (!agentIdStr) return false;

      const agent_id = Number.parseInt(agentIdStr, 10);
      if (Number.isNaN(agent_id)) return false;

      const scriptTypeForCore = scriptType === "install"
        ? "Install"
        : scriptType === "run"
          ? "Run"
          : "Delete";

      const payload = {
        agent_id,
        task_id: numericTaskId,
        script_type: scriptTypeForCore,
      };

      console.log("[run-task] request", payload);

      const res = await sendCoreRequest("run-task", payload);

      if (res?.status !== "ok") {
        console.error("[run-task] failed response", res);
        return false;
      }

      await refreshTasks();
      return true;
    } catch {
      console.error("[run-task] request error");
      return false;
    }
  }, [refreshTasks, taskStore, tasksByAgentId]);

  const stopTask = useCallback(async (taskId: string) => {
    const numericTaskId = Number.parseInt(taskId, 10);
    if (Number.isNaN(numericTaskId)) {
      return false;
    }

    try {
      let agentIdStr: string | undefined = taskStore[taskId]?.agentId;
      if (!agentIdStr) {
        for (const [aid, tasks] of Object.entries(tasksByAgentId)) {
          if (tasks.some((t) => t.id === taskId)) {
            agentIdStr = aid;
            break;
          }
        }
      }

      if (!agentIdStr) return false;

      const agent_id = Number.parseInt(agentIdStr, 10);
      if (Number.isNaN(agent_id)) return false;

      const res = await sendCoreRequest("stop-task", {
        agent_id,
        task_id: numericTaskId,
      });

      if (res?.status !== "ok") {
        return false;
      }

      await refreshTasks();
      return true;
    } catch {
      return false;
    }
  }, [refreshTasks, taskStore, tasksByAgentId]);

  useEffect(() => {
    const unsubscribeRuns = subscribeCoreRequest("execution_stream", () => {
      void refreshTasks();
    });

    const unsubscribeRunUpdate = subscribeCoreRequest("run_update", (msg) => {
      try {
        const run = msg?.data?.run;
        if (!run || !run.task_id) return;

        const taskId = String(run.task_id);
        const normalizedLog = normalizeLog(run);

        setTasksByAgentId((prevAgentTasks) => {
          const updated = { ...prevAgentTasks };
          for (const agentId in updated) {
            const tasks = updated[agentId];
            const taskIndex = tasks.findIndex((t) => t.id === taskId);
            if (taskIndex !== -1) {
              const task = tasks[taskIndex];
              const logIndex = task.logs.findIndex((l) => l.id === normalizedLog.id);

              if (logIndex !== -1) {
                task.logs[logIndex] = normalizedLog;
              } else {
                task.logs.push(normalizedLog);
              }

              tasks[taskIndex] = task;
              updated[agentId] = [...tasks];
            }
          }
          return updated;
        });
      } catch (error) {
        console.error("Error handling run_update:", error);
      }
    });

    return () => {
      unsubscribeRuns();
      unsubscribeRunUpdate();
    };
  }, [refreshAgents, refreshTasks, taskStore]);

  useEffect(() => {
    setIsLoading(true);
    Promise.all([refreshAgents(), refreshTasks()]).finally(() => setIsLoading(false));
  }, [refreshAgents, refreshTasks]);

  useEffect(() => {
    if (isLoading) return;
    if (!selectedAgentId || !selectedTaskId) return;

    const agentTasks = tasksByAgentId[selectedAgentId] ?? [];
    if (!agentTasks.some((task) => task.id === selectedTaskId)) {
      setSelectedTaskId(null);
      setSelectedLogId(null);
    }
  }, [isLoading, selectedAgentId, selectedTaskId, tasksByAgentId]);

  useEffect(() => {
    if (!isLoading && selectedAgentId && !agents.some((a) => a.id === selectedAgentId)) {
      setSelectedAgentId(null);
      setExpandedAgentId(null);
      setSelectedTaskId(null);
      setSelectedLogId(null);
      setCreateTaskAgentId(null);
    }
  }, [agents, isLoading, selectedAgentId]);

  const toggleSidebar = () => setIsSidebarCollapsed((prev) => !prev);

  const value: AppContextType = {
    agents,
    isLoading,
    selectedAgentId,
    selectedTaskId,
    selectedLogId,
    expandedAgentId,
    createTaskAgentId,
    createTaskModalOpen,
    isSidebarCollapsed,
    sidebarWidth,
    tasksByAgentId,
    setSelectedAgentId,
    setSelectedTaskId,
    setSelectedLogId,
    setExpandedAgentId,
    setCreateTaskAgentId,
    setCreateTaskModalOpen,
    toggleSidebar,
    setSidebarWidth,
    addAgent,
    createTask,
    saveTaskRecord,
    removeTaskRecord,
    runTask,
    stopTask,
    refreshAgents,
    refreshTasks,
  };

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}

export function useApp() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useApp must be used within AppProvider");
  return ctx;
}
