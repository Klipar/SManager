import type { WSMessage } from "@/types";

type RequestListener = (msg: WSMessage) => void;
type OpenListener = () => void;

class WSClient {
  private ws?: WebSocket;
  private url: string;
  private nextId = 1;
  private pending = new Map<number, (msg: WSMessage) => void>();
  private requestListeners = new Map<string, Set<RequestListener>>();
  private openListeners = new Set<OpenListener>();
  private pingInterval?: ReturnType<typeof setInterval>;
  private autoReconnect = true;

  constructor(url?: string) {
    this.url = url ?? (import.meta.env.VITE_CORE_WS as string) ?? "ws://127.0.0.1:6767";
  }

  connect() {
    if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
      return;
    }
    this.autoReconnect = true;
    this.ws = new WebSocket(this.url);

    this.ws.addEventListener("open", () => {
      console.log(`[WS] Connected`);
      this.startPingInterval();
      this.openListeners.forEach((listener) => {
        try {
          listener();
        } catch (error) {
          console.error("[WS] Open listener failed:", error);
        }
      });
    });

    this.ws.addEventListener("message", (ev) => {
      try {
        const msg = JSON.parse(ev.data) as WSMessage;
        if (!msg || typeof msg.id !== "number") {
          return;
        }

        if (msg.type === "response") {
          const resolver = this.pending.get(msg.id);
          if (resolver) {
            resolver(msg);
            this.pending.delete(msg.id);
          }
          return;
        }

        if (msg.type === "request" && msg.action) {
          const listeners = this.requestListeners.get(msg.action);
          listeners?.forEach((listener) => {
            try {
              listener(msg);
            } catch (error) {
              console.error(`[WS] Request listener failed for ${msg.action}:`, error);
            }
          });
        }
      } catch (e) {
        console.error("Invalid WS message", e);
      }
    });

    this.ws.addEventListener("close", () => {
      console.log(`[WS] Disconnected`);
      this.stopPingInterval();
      if (this.autoReconnect) {
        console.log(`[WS] Reconnecting in 1s...`);
        setTimeout(() => this.connect(), 1000);
      } else {
        console.log(`[WS] Manual disconnect, no reconnect.`);
      }
    });

    this.ws.addEventListener("error", (ev) => {
      console.error(`[WS] Error:`, ev);
    });
  }

  private startPingInterval() {
    if (this.pingInterval) clearInterval(this.pingInterval);
    this.pingInterval = setInterval(() => {
      if (this.ws && this.ws.readyState === WebSocket.OPEN) {
        const id = this.nextId++;
        const pingMsg: WSMessage = { type: "request", id, action: "ping" };
        this.pending.set(id, (msg) => {
          console.log(`[WS] Pong received: id=${msg.id}`);
        });
        try {
          this.ws.send(JSON.stringify(pingMsg));
        } catch (e) {
          console.error(`[WS] Failed to send ping:`, e);
          this.pending.delete(id);
        }
      }
    }, 20000);
  }

  private stopPingInterval() {
    if (this.pingInterval) {
      clearInterval(this.pingInterval);
      this.pingInterval = undefined;
    }
  }

  onOpen(listener: OpenListener) {
    this.openListeners.add(listener);
    if (this.ws?.readyState === WebSocket.OPEN) {
      queueMicrotask(() => {
        try {
          listener();
        } catch (error) {
          console.error("[WS] Open listener failed:", error);
        }
      });
    }

    return () => {
      this.openListeners.delete(listener);
    };
  }

  onRequest(action: string, listener: RequestListener) {
    const listeners = this.requestListeners.get(action) ?? new Set<RequestListener>();
    listeners.add(listener);
    this.requestListeners.set(action, listeners);

    return () => {
      const currentListeners = this.requestListeners.get(action);
      if (!currentListeners) {
        return;
      }

      currentListeners.delete(listener);
      if (currentListeners.size === 0) {
        this.requestListeners.delete(action);
      }
    };
  }

  async sendRequest(action: string, data: any) {
    this.connect();
    const id = this.nextId++;
    const payload: WSMessage = { type: "request", id, action, data };

    return new Promise<WSMessage>((resolve, reject) => {
      const deadline = Date.now() + 10000;

      const checker = () => {
        if (Date.now() > deadline) {
          reject(new Error(`WS connection timeout: ${action}`));
          return;
        }
        if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
          setTimeout(checker, 50);
          return;
        }
        try {
          this.pending.set(id, resolve);
          this.ws.send(JSON.stringify(payload));
          setTimeout(() => {
            if (this.pending.has(id)) {
              console.error(`[WS] Request timeout: id=${id}, action=${action}`);
              this.pending.delete(id);
              reject(new Error(`WS request timed out: ${action}`));
            }
          }, 10000);
        } catch (e) {
          console.error(`[WS] Send error:`, e);
          this.pending.delete(id);
          reject(e);
        }
      };

      checker();
    });
  }

  disconnectAndWait(): Promise<void> {
    return new Promise((resolve) => {
      this.autoReconnect = false;
      this.stopPingInterval();

      this.pending.forEach((resolver, id) => {
        try {
          resolver({ type: "response", id, status: "error", code: 0, message: "Disconnected", data: null });
        } catch {}
      });
      this.pending.clear();

      if (!this.ws || this.ws.readyState === WebSocket.CLOSED || this.ws.readyState === WebSocket.CLOSING) {
        this.ws = undefined;
        resolve();
        return;
      }

      const onClose = () => {
        this.ws?.removeEventListener("close", onClose);
        this.ws = undefined;
        resolve();
      };

      this.ws.addEventListener("close", onClose);
      this.ws.close();
    });
  }

  setUrl(url: string) {
    this.url = url;
    this.ws?.close();
  }
}

const defaultClient = new WSClient();

export function connectCore(url?: string) {
  if (url) {
    defaultClient.setUrl(url);
  }
  defaultClient.connect();
}

export function sendCoreRequest(action: string, data: any) {
  return defaultClient.sendRequest(action, data);
}

export function subscribeCoreOpen(listener: OpenListener) {
  return defaultClient.onOpen(listener);
}

export function subscribeCoreRequest(action: string, listener: RequestListener) {
  return defaultClient.onRequest(action, listener);
}

export function disconnectCore() {
  defaultClient.disconnectAndWait();
}

export async function logout() {
  try { localStorage.removeItem('sm_token'); } catch {}
  try { localStorage.removeItem('sm_userData'); } catch {}
  try { localStorage.removeItem('sm_homeViewState'); } catch {}
  try { localStorage.removeItem('sm_taskStore'); } catch {}

  await defaultClient.disconnectAndWait();
}
