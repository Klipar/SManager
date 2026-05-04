import type { RestartPolicy, TaskLog, TaskStatus } from "@/types";

export type StoredTaskRecord = {
  id: string;
  agentId: string;
  name: string;
  description: string;
  installScript: string;
  runScript: string;
  deleteScript: string;
  restartPolicy: RestartPolicy;
  createdByCore: string;
};

const TASK_STORE_KEY = "sm_taskStore";

export function loadTaskStore(): Record<string, StoredTaskRecord> {
  try {
    const raw = localStorage.getItem(TASK_STORE_KEY);
    if (!raw) return {};
    const parsed = JSON.parse(raw) as Record<string, StoredTaskRecord>;
    return parsed ?? {};
  } catch {
    return {};
  }
}

export function saveTaskStore(store: Record<string, StoredTaskRecord>) {
  try {
    localStorage.setItem(TASK_STORE_KEY, JSON.stringify(store));
  } catch {}
}

export function generateTaskId(existingIds: Set<number>) {
  const base = Math.floor(Date.now() / 1000);
  let candidate = base;

  while (existingIds.has(candidate)) {
    candidate += 1;
  }

  return candidate;
}

function logStatusFromMessage(message: string): TaskLog["status"] {
  const lower = message.toLowerCase();
  if (lower.includes("error") || lower.includes("fail")) return "error";
  if (lower.includes("warn")) return "warning";
  return "ok";
}

export function normalizeLog(rawLog: any): TaskLog {
  const message = String(rawLog?.message ?? "");
  const startedAt = rawLog?.timestamp ? String(rawLog.timestamp) : new Date().toISOString();

  return {
    id: String(rawLog?.id ?? `${startedAt}-${message.slice(0, 12)}`),
    startedAt,
    status: logStatusFromMessage(message),
    summary: message,
    output: message ? message.split("\n") : [],
  };
}

export function buildTaskStatus(logs: TaskLog[], hasStoredMeta: boolean): TaskStatus {
  if (logs.some((log) => log.status === "error")) return "failed";
  if (logs.length > 0) return "executed";
  return hasStoredMeta ? "starting" : "stopped";
}

export function buildTaskName(taskId: string, storedTask?: StoredTaskRecord) {
  return storedTask?.name ?? `Task ${taskId}`;
}

export function buildTaskDescription(taskId: string, storedTask?: StoredTaskRecord) {
  return storedTask?.description ?? `Task ${taskId}`;
}
