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
  last_update?: string | null;
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

export type TaskStatus = "ok" | "starting" | "failed" | "stopped" | "executed";

export type ScriptType = "install" | "run" | "delete";

export type TaskLog = {
  id: string;
  startedAt: string;
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
  lastLogin: string | null;
  lastUpdate: string;
  createdAt?: string;
};

export type EditUserForm = {
  name: string;
  email: string;
  password: string;
  role: UserRole;
};
