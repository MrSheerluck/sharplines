import { post } from "$lib/api/client";

export interface User {
  id: string;
  email: string;
  display_name: string | null;
  lichess_user_id: string | null;
  email_verified: boolean;
  created_at: string;
}

let user: User | null = $state(null);
let accessToken: string | null = $state(null);

export function getUser(): User | null {
  return user;
}

export function getAccessToken(): string | null {
  return accessToken;
}

export async function login(email: string, password: string): Promise<void> {
  const result = await post<{ access_token: string; user: User }>(
    "/auth/login",
    {
      email,
      password,
    },
  );
  if (result.error) throw new Error(result.error);
  user = result.data!.user;
  accessToken = result.data!.access_token;
}

export async function logout(): Promise<void> {
  await post("/auth/logout", {});
  user = null;
  accessToken = null;
}

export async function refreshToken(): Promise<void> {
  const result = await post<{ access_token: string }>("/auth/refresh", {});
  if (result.error) throw new Error(result.error);
  accessToken = result.data!.access_token;
}

export async function register(
  email: string,
  password: string,
  display_name?: string,
): Promise<void> {
  const result = await post<{ message: string }>("/auth/register", {
    email,
    password,
    display_name,
  });
  if (result.error) throw new Error(result.error);
}

export function resetAuthState(): void {
  user = null;
  accessToken = null;
}
