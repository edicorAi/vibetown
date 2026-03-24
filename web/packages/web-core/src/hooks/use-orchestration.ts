import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  fetchTown,
  fetchRigs,
  fetchAgents,
  fetchConvoys,
  fetchConvoy,
  spawnAgent,
  killAgent,
} from "@vibetown/web-core/lib/api";
import type { Agent, Convoy, Rig, Town } from "@vibetown/web-core/lib/api";
import {
  mockTown,
  mockRigs,
  mockAgents,
} from "@vibetown/web-core/lib/mock-data";

export function useTown() {
  return useQuery<Town>({
    queryKey: ["town"],
    queryFn: async () => {
      try {
        return await fetchTown();
      } catch {
        return mockTown;
      }
    },
    retry: 0,
  });
}

export function useRigs() {
  return useQuery<Rig[]>({
    queryKey: ["rigs"],
    queryFn: async () => {
      try {
        return await fetchRigs();
      } catch {
        return mockRigs;
      }
    },
    retry: 0,
  });
}

export function useAgents() {
  return useQuery<Agent[]>({
    queryKey: ["agents"],
    queryFn: async () => {
      try {
        return await fetchAgents();
      } catch {
        return mockAgents;
      }
    },
    refetchInterval: 5_000,
    retry: 0,
  });
}

export function useConvoys() {
  return useQuery<Convoy[]>({
    queryKey: ["convoys"],
    queryFn: async () => {
      try {
        return await fetchConvoys();
      } catch {
        return [];
      }
    },
    retry: 0,
  });
}

export function useConvoy(id: string) {
  return useQuery<Convoy>({
    queryKey: ["convoy", id],
    queryFn: () => fetchConvoy(id),
    enabled: !!id,
  });
}

export function useSpawnAgent() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: spawnAgent,
    onSuccess: () => {
      void qc.invalidateQueries({ queryKey: ["agents"] });
    },
  });
}

export function useKillAgent() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: killAgent,
    onSuccess: () => {
      void qc.invalidateQueries({ queryKey: ["agents"] });
    },
  });
}
