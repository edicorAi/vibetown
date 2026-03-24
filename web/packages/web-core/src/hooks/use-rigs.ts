import { useQuery } from "@tanstack/react-query"
import { fetchRigs } from "@vibetown/web-core/lib/api"
import type { Rig } from "@vibetown/web-core/lib/api"
import { mockRigs } from "@vibetown/web-core/lib/mock-data"

export function useRigs() {
  return useQuery<Rig[]>({
    queryKey: ["rigs"],
    queryFn: async () => {
      try {
        return await fetchRigs()
      } catch {
        return mockRigs
      }
    },
    retry: 0,
  })
}
