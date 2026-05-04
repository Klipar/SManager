import React, { createContext, useContext, useEffect, useState, useCallback } from "react";
import { sendCoreRequest } from "@/lib/ws";
import type { Agent, CreateTaskPayload, Task, TaskLog } from "@/types";
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
  isSidebarCollapsed: boolean;
  sidebarWidth: number;
  tasksByAgentId: Record<string, Task[]>;
  setSelectedAgentId: (id: string | null) => void;
  setSelectedTaskId: (id: string | null) => void;
  setSelectedLogId: (id: string | null) => void;
  setExpandedAgentId: (id: string | null) => void;
  setCreateTaskAgentId: (id: string | null) => void;
  toggleSidebar: () => void;
  setSidebarWidth: (width: number) => void;
  addAgent: (payload: any) => Promise<void>;
  createTask: (payload: CreateTaskPayload) => Promise<string | null>;
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
      isSidebarCollapsed,
      sidebarWidth,
    });
  }, [selectedAgentId, expandedAgentId, selectedTaskId, selectedLogId, createTaskAgentId, isSidebarCollapsed, sidebarWidth]);

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
      const [tasksRes, logsRes] = await Promise.all([
        sendCoreRequest("get-all-tasks", null),
        sendCoreRequest("get-logs", null),
      ]);

      if (tasksRes?.status !== "ok") {
        setTasksByAgentId({});
        return;
      }

      const rawTasks = tasksRes.data?.tasks ?? [];
      const rawLogs = logsRes?.status === "ok" ? (logsRes.data?.logs ?? []) : [];

      const logsByTaskId = new Map<string, TaskLog[]>();
      for (const rawLog of rawLogs) {
        const taskId = rawLog?.task_id;
        if (taskId === null || taskId === undefined) continue;

        const normalizedLog = normalizeLog(rawLog);
        const key = String(taskId);
        const existing = logsByTaskId.get(key) ?? [];
        existing.push(normalizedLog);
        logsByTaskId.set(key, existing);
      }

      const groupedTasks: Record<string, Task[]> = {};

      for (const rawTask of rawTasks) {
        const taskId = String(rawTask?.id ?? "");
        const agentId = String(rawTask?.agent_id ?? "");
        if (!taskId || !agentId) continue;

        const storedTask = taskStoreSnapshot[taskId];
        const logs = logsByTaskId.get(taskId) ?? [];

        const task: Task = {
          id: taskId,
          name: buildTaskName(taskId, storedTask),
          scriptType: "run",
          status: buildTaskStatus(logs, Boolean(storedTask)),
          description: buildTaskDescription(taskId, storedTask),
          createdByCore: storedTask?.createdByCore ?? "Core",
          restartPolicy: storedTask?.restartPolicy ?? "no",
          logs,
        };

        if (!groupedTasks[agentId]) {
          groupedTasks[agentId] = [];
        }

        groupedTasks[agentId].push(task);
      }

      Object.values(groupedTasks).forEach((tasks) => {
        tasks.sort((left, right) => Number(right.id) - Number(left.id));
      });

      setTasksByAgentId(groupedTasks);
    } catch {
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

    await refreshTasks(nextTaskStore);
    return taskRecord.id;
  }, [refreshTasks, taskStore]);

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
    isSidebarCollapsed,
    sidebarWidth,
    tasksByAgentId,
    setSelectedAgentId,
    setSelectedTaskId,
    setSelectedLogId,
    setExpandedAgentId,
    setCreateTaskAgentId,
    toggleSidebar,
    setSidebarWidth,
    addAgent,
    createTask,
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
