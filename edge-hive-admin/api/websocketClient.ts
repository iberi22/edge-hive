
import { SystemMetric } from '../types';

const WS_URL = 'ws://localhost:9001';

class WebSocketManager {
    private ws: WebSocket | null = null;
    private static instance: WebSocketManager;
    private subscriptions: Map<string, ((data: any) => void)[]> = new Map();

    private constructor() {
        this.connect();
    }

    public static getInstance(): WebSocketManager {
        if (!WebSocketManager.instance) {
            WebSocketManager.instance = new WebSocketManager();
        }
        return WebSocketManager.instance;
    }

    private connect() {
        this.ws = new WebSocket(WS_URL);

        this.ws.onopen = () => {
            console.log('WebSocket connected');
            this.subscriptions.forEach((_, topic) => {
                this.subscribe(topic, () => {});
            });
        };

        this.ws.onmessage = (event) => {
            const message = JSON.parse(event.data);
            if (message.Event && this.subscriptions.has(message.Event.topic)) {
                this.subscriptions.get(message.Event.topic)?.forEach(cb => cb(message.Event.data));
            }
        };

        this.ws.onclose = () => {
            console.log('WebSocket disconnected, attempting to reconnect...');
            setTimeout(() => this.connect(), 1000);
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.ws?.close();
        };
    }

    public subscribe(topic: string, callback: (data: any) => void) {
        if (!this.subscriptions.has(topic)) {
            this.subscriptions.set(topic, []);
        }
        this.subscriptions.get(topic)?.push(callback);

        if (this.ws?.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify({ Subscribe: { topic } }));
        }
    }

    public unsubscribe(topic: string) {
        this.subscriptions.delete(topic);
        if (this.ws?.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify({ Unsubscribe: { topic } }));
        }
    }
}

export const websocketApi = WebSocketManager.getInstance();
