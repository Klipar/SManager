// --- WebSocket Message ---
export type WSMessage = {
  type: "request" | "response";
  id: number;
  action?: string;
  status?: "ok" | "error";
  code?: number;
  message?: string;
  data?: any;
};

// --- User Data ---
export type UserData = {
  id?: number;
  name?: string;
  email?: string;
  is_admin?: boolean;
};

// --- Agent ---
export type AgentStatus = "online" | "offline" | "error";

export type Agent = {
  id: string;
  name: string;
  status: AgentStatus;
  ip?: string;
  description?: string;
  port?: number;
};

// --- Task ---
export type RestartPolicy = "no" | "always" | "on-failure";

export type CreateTaskPayload = {
  agentId: string;
  name: string;
  description: string;
  installScript: string;
  runScript: string;
  deleteScript: string;
  restartPolicy: RestartPolicy;
};

export type TaskStatus = "ok" | "starting" | "failed" | "stopped" | "executed";

export type ScriptType = "install" | "run" | "delete";

export type TaskLog = {
  id: string;
  startedAt: string;
  endedAt?: string;
  scriptType: ScriptType;
  status: "ok" | "warning" | "error";
  summary: string;
  output: string[];
};

export type Task = {
  id: string;
  name: string;
  scriptType: ScriptType;
  status: TaskStatus;
  description: string;
  createdByCore: string;
  restartPolicy: RestartPolicy;
  installScript?: string | null;
  runScript?: string | null;
  deleteScript?: string | null;
  logs: TaskLog[];
};

export type CurrentUser = {
  username: string;
  role: "admin" | "user";
};

// --- Admin ---
export type UserRole = "admin" | "user";

export type AdminUser = {
  id: number;
  name: string;
  email: string;
  role: UserRole;
  createdAt?: string;
  updatedAt: string;
  lastLogin: string | null;
};

export type EditUserForm = {
  name: string;
  email: string;
  password: string;
  role: UserRole;
};
