import React, { createContext, useContext, useEffect, useState, useCallback } from "react";
import { sendCoreRequest } from "@/lib/ws";
import type { Agent, Task } from "@/features/home/types";

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
  refreshAgents: () => void;
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

  const tasksByAgentId: Record<string, Task[]> = {};

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

  const addAgent = useCallback(async (payload: any) => {
    const res = await sendCoreRequest("new-agent", payload);
    if (res?.status === "ok") {
      await refreshAgents();
    }
  }, [refreshAgents]);

  useEffect(() => {
    setIsLoading(true);
    refreshAgents().finally(() => setIsLoading(false));
  }, [refreshAgents]);

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
    refreshAgents,
  };

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}

export function useApp() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useApp must be used within AppProvider");
  return ctx;
}
