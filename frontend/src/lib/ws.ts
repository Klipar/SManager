type WSMessage = {
  type: "request" | "response";
  id: number;
  action?: string;
  status?: "ok" | "error";
  code?: number;
  message?: string;
  data?: any;
}

class WSClient {
  private ws?: WebSocket;
  private url: string;
  private nextId = 1;
  private pending = new Map<number, (msg: WSMessage) => void>();
  private pingInterval?: ReturnType<typeof setInterval>;

  constructor(url?: string) {
    this.url = url ?? (import.meta.env.VITE_CORE_WS as string) ?? "ws://127.0.0.1:6767";
  }

  connect() {
    if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
      console.log(`[WS] Already connected or connecting`);
      return;
    }
    console.log(`[WS] Connecting to ${this.url}`);
    this.ws = new WebSocket(this.url);

    this.ws.addEventListener("open", () => {
      console.log(`[WS] Connected`);
      this.startPingInterval();
    });

    this.ws.addEventListener("message", (ev) => {
      try {
        const msg = JSON.parse(ev.data) as WSMessage;
        console.log(`[WS] Received response: id=${msg.id}, status=${msg.status}`);
        if (msg && typeof msg.id === "number") {
          const resolver = this.pending.get(msg.id);
          if (resolver) {
            resolver(msg);
            this.pending.delete(msg.id);
          }
        }
      } catch (e) {
        console.error("Invalid WS message", e);
      }
    });

    this.ws.addEventListener("close", () => {
      console.log(`[WS] Disconnected, reconnecting in 1s...`);
      this.stopPingInterval();
      setTimeout(() => this.connect(), 1000);
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
          console.log(`[WS] Sent ping: id=${id}`);
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

  async sendRequest(action: string, data: any) {
    this.connect();
    const id = this.nextId++;
    const payload: WSMessage = { type: "request", id, action, data };
    console.log(`[WS] Sending request: id=${id}, action=${action}`);

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
          console.log(`[WS] Sent request: id=${id}, action=${action}`);
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
