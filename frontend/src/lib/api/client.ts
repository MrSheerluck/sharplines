let accessToken: string | null = null;
let onAuthFailure: (() => void) | null = null;

export interface ApiResult<T> {
  data?: T;
  error?: string;
  status: number;
}

export function setAccessToken(token: string | null) {
  accessToken = token;
}

export function setOnAuthFailure(callback: (() => void) | null) {
  onAuthFailure = callback;
}

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
): Promise<ApiResult<T>> {
  const headers: Record<string, string> = {};
  if (body !== undefined) {
    headers["Content-Type"] = "application/json";
  }
  if (accessToken) {
    headers["Authorization"] = `Bearer ${accessToken}`;
  }
  const response = await fetch(`http://localhost:3000${path}`, {
    method,
    headers,
    body: body !== undefined ? JSON.stringify(body) : undefined,
    credentials: "include",
  });
  if (!response.ok) {
    if (response.status === 401 && onAuthFailure) {
      const refreshResponse = await fetch(
        "http://localhost:3000/auth/refresh",
        {
          method: "POST",
          credentials: "include",
        },
      );
      if (refreshResponse.ok) {
        const refreshData = await refreshResponse.json();
        accessToken = refreshData.access_token;
        const retryHeaders: Record<string, string> = {};
        if (body !== undefined) {
          retryHeaders["Content-Type"] = "application/json";
        }
        retryHeaders["Authorization"] = `Bearer ${accessToken}`;
        const retry = await fetch(`http://localhost:3000${path}`, {
          method,
          headers: retryHeaders,
          body: body !== undefined ? JSON.stringify(body) : undefined,
          credentials: "include",
        });
        const retryData = await retry.json();
        if (!retry.ok) {
          return {
            error: retryData.error || "Request failed",
            status: retry.status,
          };
        }
        return { data: retryData as T, status: retry.status };
      }
      onAuthFailure();
    }
    const data = await response.json();
    return { error: data.error || "Request failed", status: response.status };
  }
  const data = await response.json();
  return { data: data as T, status: response.status };
}

export async function post<T>(
  path: string,
  body: unknown,
): Promise<ApiResult<T>> {
  return request<T>("POST", path, body);
}

export async function get<T>(path: string): Promise<ApiResult<T>> {
  return request<T>("GET", path);
}
