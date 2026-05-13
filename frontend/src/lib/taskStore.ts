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

function logStatusFromMessage(message: string, returnCode?: number | null): TaskLog["status"] {
  if (returnCode !== null && returnCode !== undefined) {
    if (returnCode !== 0) return "error";
    return "ok";
  }
  const lower = message.toLowerCase();
  if (lower.includes("error") || lower.includes("fail")) return "error";
  if (lower.includes("warn")) return "warning";
  return "ok";
}

function normalizeScriptType(script: unknown) {
  const value = String(script ?? "").toLowerCase();
  if (value === "install" || value === "delete") {
    return value;
  }
  return "run";
}

export function normalizeLog(rawLog: any): TaskLog {
  const output = String(rawLog?.output ?? "");
  const startedAt = rawLog?.start_time || new Date().toISOString();
  const endedAt = rawLog?.end_time || rawLog?.endTime;

  return {
    id: String(rawLog?.id ?? `${startedAt}-${output.slice(0, 12)}`),
    startedAt,
    endedAt,
    scriptType: normalizeScriptType(rawLog?.script),
    status: logStatusFromMessage(output, rawLog?.return_code),
    summary: output.split("\n")[0] || output.slice(0, 100),
    output: output ? output.split("\n") : [],
  };
}

export function buildTaskStatus(logs: TaskLog[], hasStoredMeta: boolean): TaskStatus {
  if (logs.some((log) => log.status === "error")) return "Failed";
  if (logs.length > 0) return "Executed";
  return hasStoredMeta ? "Starting" : "Stopped";
}

export function buildTaskName(taskId: string, storedTask?: StoredTaskRecord) {
  return storedTask?.name ?? `Task ${taskId}`;
}

export function buildTaskDescription(taskId: string, storedTask?: StoredTaskRecord) {
  return storedTask?.description ?? `Task ${taskId}`;
}
