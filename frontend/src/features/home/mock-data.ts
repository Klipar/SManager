import type { Agent, CurrentUser, Task } from "./types"

const agents: Agent[] = [
  { id: "agent-main", name: "Agent main", status: "online" },
  { id: "agent-67", name: "Agent 67", status: "online" },
  { id: "agent-aboba", name: "Agent aboba", status: "error" },
  { id: "agent-stu-fein", name: "Agent STU FEIN", status: "offline" },
  { id: "agent-aquila", name: "Agent Aquila", status: "online" },
  { id: "agent-orbit", name: "Agent Orbit", status: "offline" },
]

const currentUser: CurrentUser = {
  username: "Arsenicum",
  role: "admin",
}

const tasksByAgentId: Record<string, Task[]> = {
  "agent-main": [
    {
      id: "task-health-check",
      name: "Health Check",
      scriptType: "run",
      status: "starting",
      description: "Collects cpu, memory, disk and process metrics from host.",
      createdByCore: "core-alpha",
      restartPolicy: "always",
      logs: [
        {
          id: "log-hc-1",
          startedAt: "12.5.2026 19:00",
          status: "ok",
          summary: "Completed cycle with no incidents.",
          output: [
            "[INFO] metrics collector started",
            "[INFO] cpu usage: 37%",
            "[INFO] memory usage: 62%",
            "[INFO] no anomalies detected",
          ],
        },
        {
          id: "log-hc-2",
          startedAt: "12.5.2026 18:00",
          status: "warning",
          summary: "Disk usage is approaching threshold.",
          output: [
            "[INFO] metrics collector started",
            "[WARN] disk /data usage: 83%",
            "[INFO] cleanup job scheduled",
          ],
        },
      ],
    },
    {
      id: "task-sync-logs",
      name: "Sync Logs",
      scriptType: "install",
      status: "executed",
      description: "Pushes compressed logs to centralized storage every hour.",
      createdByCore: "core-alpha",
      restartPolicy: "on-failure",
      logs: [
        {
          id: "log-sl-1",
          startedAt: "12.5.2026 17:00",
          status: "ok",
          summary: "128 files synchronized.",
          output: [
            "[INFO] sync started",
            "[INFO] archived 128 files",
            "[INFO] upload complete",
          ],
        },
      ],
    },
  ],
  "agent-67": [
    {
      id: "task-backup-config",
      name: "Backup Config",
      scriptType: "delete",
      status: "failed",
      description: "Creates configuration snapshots and verifies checksum.",
      createdByCore: "core-beta",
      restartPolicy: "no",
      logs: [
        {
          id: "log-bc-1",
          startedAt: "12.5.2026 19:00",
          status: "error",
          summary: "Checksum verification failed.",
          output: [
            "[INFO] backup started",
            "[ERROR] checksum mismatch on config bundle",
            "[INFO] task stopped",
          ],
        },
      ],
    },
  ],
  "agent-aboba": [],
  "agent-stu-fein": [],
  "agent-aquila": [],
  "agent-orbit": [],
}

export { agents, currentUser, tasksByAgentId }
