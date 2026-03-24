import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  fetchInbox,
  fetchSent,
  fetchMailQueue,
  sendMail,
  markMailRead,
} from "@vibetown/web-core/lib/api";
import type { MailMessage } from "@vibetown/web-core/lib/api";
import { mockMail } from "@vibetown/web-core/lib/mock-data";

export function useInbox() {
  return useQuery<MailMessage[]>({
    queryKey: ["mail", "inbox"],
    queryFn: async () => {
      try {
        return await fetchInbox();
      } catch {
        return mockMail;
      }
    },
    refetchInterval: 10_000,
    retry: 0,
  });
}

export function useSent() {
  return useQuery<MailMessage[]>({
    queryKey: ["mail", "sent"],
    queryFn: async () => {
      try {
        return await fetchSent();
      } catch {
        return mockMail.filter((m) => m.read);
      }
    },
    retry: 0,
  });
}

export function useMailQueue() {
  return useQuery<MailMessage[]>({
    queryKey: ["mail", "queue"],
    queryFn: async () => {
      try {
        return await fetchMailQueue();
      } catch {
        return mockMail.filter((m) => !m.read);
      }
    },
    refetchInterval: 10_000,
    retry: 0,
  });
}

export function useSendMail() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: sendMail,
    onSuccess: () => {
      void qc.invalidateQueries({ queryKey: ["mail"] });
    },
  });
}

export function useMarkRead() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: markMailRead,
    onSuccess: () => {
      void qc.invalidateQueries({ queryKey: ["mail"] });
    },
  });
}
