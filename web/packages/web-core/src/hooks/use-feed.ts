import { useEffect, useRef, useState, useCallback } from "react";
import type { FeedEvent } from "@vibetown/web-core/lib/api";
import { useQuery } from "@tanstack/react-query";
import { fetchFeedEvents } from "@vibetown/web-core/lib/api";
import { mockFeedEvents } from "@vibetown/web-core/lib/mock-data";

/**
 * Poll-based feed events hook.
 * Falls back to mock data when API is unavailable.
 */
export function useFeedEvents() {
  return useQuery<FeedEvent[]>({
    queryKey: ["feed-events"],
    queryFn: async () => {
      try {
        return await fetchFeedEvents();
      } catch {
        return mockFeedEvents;
      }
    },
    refetchInterval: 5_000,
    retry: 0,
  });
}

/**
 * WebSocket-based real-time feed hook.
 * Connects to `ws(s)://<host>/api/feed/ws` and appends events as they arrive.
 * Falls back gracefully if the WebSocket endpoint is unavailable.
 */
export function useFeedWebSocket(maxEvents = 50) {
  const [events, setEvents] = useState<FeedEvent[]>([]);
  const [connected, setConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  const addEvent = useCallback(
    (event: FeedEvent) => {
      setEvents((prev) => [event, ...prev].slice(0, maxEvents));
    },
    [maxEvents]
  );

  useEffect(() => {
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const wsUrl = `${protocol}//${window.location.host}/api/feed/ws`;

    let ws: WebSocket;
    try {
      ws = new WebSocket(wsUrl);
    } catch {
      return;
    }

    wsRef.current = ws;

    ws.onopen = () => setConnected(true);
    ws.onclose = () => setConnected(false);
    ws.onerror = () => setConnected(false);

    ws.onmessage = (msg) => {
      try {
        const data = JSON.parse(msg.data as string) as FeedEvent;
        addEvent(data);
      } catch {
        // ignore malformed messages
      }
    };

    return () => {
      ws.close();
      wsRef.current = null;
    };
  }, [addEvent]);

  return { events, connected };
}
