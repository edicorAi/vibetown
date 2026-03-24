import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  fetchMergeRequests,
  queueMerge,
} from "@vibetown/web-core/lib/api";
import type { MergeRequest } from "@vibetown/web-core/lib/api";
import { mockMergeRequests } from "@vibetown/web-core/lib/mock-data";

export function useMergeRequests() {
  return useQuery<MergeRequest[]>({
    queryKey: ["merge-queue"],
    queryFn: async () => {
      try {
        return await fetchMergeRequests();
      } catch {
        return mockMergeRequests;
      }
    },
    refetchInterval: 10_000,
    retry: 0,
  });
}

export function useQueueMerge() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: queueMerge,
    onSuccess: () => {
      void qc.invalidateQueries({ queryKey: ["merge-queue"] });
    },
  });
}
