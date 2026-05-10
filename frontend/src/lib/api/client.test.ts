import { describe, it, expect, vi, beforeEach } from "vitest";
import * as client from "./client";

beforeEach(() => {
  vi.restoreAllMocks();
  client.setAccessToken(null);
  client.setOnAuthFailure(null);
});

it("includes Authorization header when token is set", async () => {
  const mockFetch = vi.fn().mockResolvedValueOnce({
    ok: true,
    status: 200,
    json: () => Promise.resolve({}),
  });
  globalThis.fetch = mockFetch;

  client.setAccessToken("my-token");
  await client.get("/test");

  expect(mockFetch).toHaveBeenCalledWith(
    expect.any(String),
    expect.objectContaining({
      headers: expect.objectContaining({
        Authorization: "Bearer my-token",
      }),
    }),
  );
});

it("sets Content-Type for POST requests", async () => {
  const mockFetch = vi.fn().mockResolvedValue({
    ok: true,
    status: 200,
    json: () => Promise.resolve({}),
  });
  globalThis.fetch = mockFetch;
  await client.post("/test", { key: "value" });
  expect(mockFetch).toHaveBeenCalledWith(
    expect.any(String),
    expect.objectContaining({
      method: "POST",
      headers: expect.objectContaining({
        "Content-Type": "application/json",
      }),
      body: JSON.stringify({ key: "value" }),
    }),
  );
});

it("sends credentials include", async () => {
  const mockFetch = vi.fn().mockResolvedValue({
    ok: true,
    status: 200,
    json: () => Promise.resolve({}),
  });
  globalThis.fetch = mockFetch;
  await client.get("/test");
  expect(mockFetch).toHaveBeenCalledWith(
    expect.any(String),
    expect.objectContaining({
      credentials: "include",
    }),
  );
});

it("returns error on non-ok response", async () => {
  const mockFetch = vi.fn().mockResolvedValue({
    ok: false,
    status: 400,
    json: () => Promise.resolve({ error: "Bad request" }),
  });
  globalThis.fetch = mockFetch;
  const result = await client.get("/test");
  expect(result.error).toBe("Bad request");
  expect(result.status).toBe(400);
  expect(result.data).toBeUndefined();
});

it("calls onAuthFailure when refresh also returns 401", async () => {
  const mockFetch = vi
    .fn()
    .mockResolvedValueOnce({
      ok: false,
      status: 401,
      json: () => Promise.resolve({ error: "Unauthorized" }),
    })
    .mockResolvedValueOnce({
      ok: false,
      status: 401,
      json: () => Promise.resolve({ error: "Invalid refresh token" }),
    });
  globalThis.fetch = mockFetch;
  const onAuthFailure = vi.fn();
  client.setOnAuthFailure(onAuthFailure);
  const result = await client.get("/protected");
  expect(onAuthFailure).toHaveBeenCalled();
  expect(result.error).toBe("Unauthorized");
});

it("retries original request after successful token refresh", async () => {
  const mockFetch = vi
    .fn()
    .mockResolvedValueOnce({
      ok: false,
      status: 401,
      json: () => Promise.resolve({ error: "Unauthorized" }),
    })
    .mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ access_token: "new-token" }),
    })
    .mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ data: "success" }),
    });
  globalThis.fetch = mockFetch;
  client.setAccessToken("expired-token");
  client.setOnAuthFailure(vi.fn());
  const result = await client.get("/protected");
  expect(result.data).toEqual({ data: "success" });
  expect(result.status).toBe(200);
});
