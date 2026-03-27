/**
 * API client for communicating with the Vibetown Rust server.
 */

const API_BASE = import.meta.env.VITE_API_BASE ?? "/api";

export async function apiFetch<T>(
  path: string,
  init?: RequestInit
): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    ...init,
    credentials: "same-origin",
    headers: {
      "Content-Type": "application/json",
      ...init?.headers,
    },
  });
  if (res.status === 401 && !path.startsWith("/auth/")) {
    // Redirect to login if not already there
    if (!window.location.hash.startsWith("#/login")) {
      window.location.hash = "#/login";
    }
    throw new Error("Authentication required");
  }
  if (!res.ok) {
    throw new Error(`API error: ${res.status} ${res.statusText}`);
  }
  return res.json();
}

// ---------------------------------------------------------------------------
// Auth types & functions
// ---------------------------------------------------------------------------

export interface AuthUser {
  id: string;
  email: string;
  username: string | null;
  display_name: string | null;
  auth_provider: string;
  avatar_url: string | null;
  is_active: boolean;
  is_admin: boolean;
  created_at: string;
}

export interface AuthProvider {
  name: string;
  provider_type: string;
  enabled: boolean;
}

export interface AuthStatus {
  has_users: boolean;
  auth_required: boolean;
  providers: AuthProvider[];
}

export function fetchAuthStatus(): Promise<AuthStatus> {
  return apiFetch<AuthStatus>("/auth/status");
}

export function fetchCurrentUser(): Promise<{ user: AuthUser }> {
  return apiFetch<{ user: AuthUser }>("/auth/me");
}

export function login(email: string, password: string): Promise<{ user: AuthUser }> {
  return apiFetch<{ user: AuthUser }>("/auth/login", {
    method: "POST",
    body: JSON.stringify({ email, password }),
  });
}

export function ldapLogin(username: string, password: string): Promise<{ user: AuthUser }> {
  return apiFetch<{ user: AuthUser }>("/auth/ldap/login", {
    method: "POST",
    body: JSON.stringify({ username, password }),
  });
}

export function logout(): Promise<void> {
  return apiFetch<void>("/auth/logout", { method: "POST" });
}

export function fetchAuthProviders(): Promise<AuthProvider[]> {
  return apiFetch<AuthProvider[]>("/auth/providers");
}

// ---------------------------------------------------------------------------
// Types matching the protobuf messages
// ---------------------------------------------------------------------------

export interface Town {
  id: string;
  name: string;
  owner: string;
  config_json: string;
}

export interface Rig {
  id: string;
  town_id: string;
  name: string;
  repo_url: string;
  beads_prefix: string;
}

export interface Agent {
  id: string;
  name: string;
  role: string;
  rig_id: string;
  status: string;
  runtime: string;
  last_activity_at: string;
}

export interface Convoy {
  id: string;
  name: string;
  status: string;
  formula: string;
}

export interface MailMessage {
  id: string;
  from_addr: string;
  to_addr: string;
  subject: string;
  body: string;
  priority: string;
  message_type: string;
  read: boolean;
  created_at: string;
}

export interface FeedEvent {
  id: string;
  event_type: string;
  source: string;
  summary: string;
  severity: string;
  created_at: string;
}

export interface MergeRequest {
  id: string;
  branch: string;
  target_branch: string;
  status: string;
  pr_url: string;
  agent_id: string;
  queued_at: string;
}

export interface WorkItem {
  id: string;
  item_type: string;
  title: string;
  description: string;
  status: string;
  priority: number;
  assignee: string;
}

// ---------------------------------------------------------------------------
// Typed API functions
// ---------------------------------------------------------------------------

export function fetchTown(): Promise<Town> {
  return apiFetch<Town>("/town");
}

export function fetchRigs(): Promise<Rig[]> {
  return apiFetch<Rig[]>("/rigs");
}

export function fetchAgents(): Promise<Agent[]> {
  return apiFetch<Agent[]>("/agents");
}

export function fetchConvoys(): Promise<Convoy[]> {
  return apiFetch<Convoy[]>("/convoys");
}

export function fetchConvoy(id: string): Promise<Convoy> {
  return apiFetch<Convoy>(`/convoys/${id}`);
}

export function spawnAgent(data: {
  name: string;
  role: string;
  runtime: string;
  rig_id: string;
}): Promise<Agent> {
  return apiFetch<Agent>("/agents", {
    method: "POST",
    body: JSON.stringify(data),
  });
}

export function killAgent(id: string): Promise<void> {
  return apiFetch<void>(`/agents/${id}`, { method: "DELETE" });
}

export async function fetchInbox(): Promise<MailMessage[]> {
  const res = await apiFetch<{ messages: MailMessage[] } | MailMessage[]>("/mail/inbox");
  return Array.isArray(res) ? res : res.messages ?? [];
}

export async function fetchSent(): Promise<MailMessage[]> {
  const res = await apiFetch<{ messages: MailMessage[] } | MailMessage[]>("/mail/sent");
  return Array.isArray(res) ? res : res.messages ?? [];
}

export async function fetchMailQueue(): Promise<MailMessage[]> {
  const res = await apiFetch<{ messages: MailMessage[] } | MailMessage[]>("/mail/queue");
  return Array.isArray(res) ? res : res.messages ?? [];
}

export function sendMail(data: {
  from_addr: string;
  to_addr: string;
  subject: string;
  body: string;
  priority: string;
}): Promise<MailMessage> {
  return apiFetch<MailMessage>("/mail", {
    method: "POST",
    body: JSON.stringify(data),
  });
}

export function markMailRead(id: string): Promise<void> {
  return apiFetch<void>(`/mail/${id}/read`, { method: "PATCH" });
}

export function fetchFeedEvents(): Promise<FeedEvent[]> {
  return apiFetch<FeedEvent[]>("/feed");
}

export function fetchMergeRequests(): Promise<MergeRequest[]> {
  return apiFetch<MergeRequest[]>("/merge-queue");
}

export function queueMerge(data: {
  branch: string;
  target_branch: string;
  pr_url: string;
}): Promise<MergeRequest> {
  return apiFetch<MergeRequest>("/merge-queue", {
    method: "POST",
    body: JSON.stringify(data),
  });
}
