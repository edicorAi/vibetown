import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  fetchAuthStatus,
  fetchCurrentUser,
  login as apiLogin,
  ldapLogin as apiLdapLogin,
  logout as apiLogout,
  type AuthUser,
  type AuthStatus,
} from "../lib/api";

export function useAuthStatus() {
  return useQuery<AuthStatus>({
    queryKey: ["auth", "status"],
    queryFn: fetchAuthStatus,
    retry: false,
    staleTime: 60_000,
  });
}

export function useCurrentUser(enabled = true) {
  return useQuery<{ user: AuthUser }>({
    queryKey: ["auth", "me"],
    queryFn: fetchCurrentUser,
    retry: false,
    staleTime: 30_000,
    enabled,
  });
}

export function useLogin() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ email, password }: { email: string; password: string }) =>
      apiLogin(email, password),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["auth"] });
    },
  });
}

export function useLdapLogin() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ username, password }: { username: string; password: string }) =>
      apiLdapLogin(username, password),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["auth"] });
    },
  });
}

export function useLogout() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => apiLogout(),
    onSuccess: () => {
      queryClient.clear();
      window.location.hash = "#/login";
    },
  });
}
