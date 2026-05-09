import { describe, it, expect, vi, beforeEach } from "vitest";
import * as auth from "./auth.svelte";
import * as client from "$lib/api/client";

vi.mock("$lib/api/client", () => ({
  post: vi.fn(),
  get: vi.fn(),
}));

const mockUser = {
  id: "550e8400-e29b-41d4-a716-446655440000",
  email: "test@example.com",
  display_name: "TestUser",
  lichess_user_id: null,
  email_verified: true,
  created_at: "2026-05-09T12:00:00Z",
};

describe("Auth Store", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    auth.resetAuthState();
  });

  it("initializes with unauthenticated state", () => {
    expect(auth.getUser()).toBeNull();
    expect(auth.getAccessToken()).toBeNull();
  });

  it("login updates state correctly on success", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({
      data: { access_token: "test_access_token", user: mockUser },
      status: 200,
    });

    await auth.login("test@example.com", "password123");
    expect(auth.getUser()).toEqual(mockUser);
    expect(auth.getAccessToken()).toBe("test_access_token");
    expect(vi.mocked(client.post)).toHaveBeenCalledWith("/auth/login", {
      email: "test@example.com",
      password: "password123",
    });
  });

  it("login throws on error and keeps state unchanged", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({
      error: "Invalid credentials",
      status: 401,
    });

    await expect(auth.login("test@xample.com", "wrong")).rejects.toThrow(
      "Invalid credentials",
    );
    expect(auth.getUser()).toBeNull();
    expect(auth.getAccessToken()).toBeNull();
  });

  it("logout clears user and access token", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({
      data: { access_token: "tok", user: mockUser },
      status: 200,
    });
    await auth.login("test@example.com", "pass");

    vi.mocked(client.post).mockResolvedValueOnce({ data: {}, status: 200 });
    await auth.logout();

    expect(auth.getUser()).toBeNull();
    expect(auth.getAccessToken()).toBeNull();
  });

  it("refreshToken updates access token", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({
      data: { access_token: "refreshed_token" },
      status: 200,
    });
    await auth.refreshToken();
    expect(auth.getAccessToken()).toBe("refreshed_token");
  });

  it("refreshToken throws failure", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({
      error: "Invalid refresh token",
      status: 401,
    });

    await expect(auth.refreshToken()).rejects.toThrow("Invalid refresh token");
  });

  it("register does not change auth state", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({
      data: { messsage: "Check your email to verify your account" },
      status: 201,
    });

    await auth.register("new@example.com", "password123", "NewUser");
    expect(auth.getUser()).toBeNull();
    expect(auth.getAccessToken()).toBeNull();
    expect(vi.mocked(client.post)).toHaveBeenCalledWith("/auth/register", {
      email: "new@example.com",
      password: "password123",
      display_name: "NewUser",
    });
  });

  it("register throws on error", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({
      error: "Email already exists",
      status: 400,
    });
    await expect(
      auth.register("existing@example.com", "pass123"),
    ).rejects.toThrow("Email already exists");
  });
  it("handles network errors gracefully", async () => {
    vi.mocked(client.post).mockRejectedValueOnce(new Error("Network error"));
    await expect(auth.login("test@example.com", "pass")).rejects.toThrow(
      "Network error",
    );
    expect(auth.getUser()).toBeNull();
    expect(auth.getAccessToken()).toBeNull();
  });
});
