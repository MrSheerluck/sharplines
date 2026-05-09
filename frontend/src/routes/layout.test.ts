import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import Layout from "./+layout.svelte";
vi.mock("$lib/stores/auth.svelte", () => ({
  getUser: vi.fn(),
  getAccessToken: vi.fn(),
  logout: vi.fn(),
}));
import * as auth from "$lib/stores/auth.svelte";
describe("App Layout", () => {
  beforeEach(() => vi.clearAllMocks());
  it("shows login and register when unauthenticated", () => {
    vi.mocked(auth.getUser).mockReturnValue(null);
    vi.mocked(auth.getAccessToken).mockReturnValue(null);
    render(Layout, { props: { children: () => "" } });
    expect(screen.getByRole("link", { name: /log in/i })).toHaveAttribute(
      "href",
      "/login",
    );
    expect(screen.getByRole("link", { name: /register/i })).toHaveAttribute(
      "href",
      "/register",
    );
  });
  it("shows logo and brand name", () => {
    vi.mocked(auth.getUser).mockReturnValue(null);
    vi.mocked(auth.getAccessToken).mockReturnValue(null);
    render(Layout, { props: { children: () => "" } });
    expect(screen.getAllByText("SharpLines").length).toBeGreaterThan(0);
  });

  it("shows user info and logout when authenticated", () => {
    vi.mocked(auth.getUser).mockReturnValue({
      id: "1",
      email: "test@example.com",
      display_name: "TestUser",
      lichess_user_id: null,
      email_verified: true,
      created_at: "2026-01-01T00:00:00Z",
    });
    vi.mocked(auth.getAccessToken).mockReturnValue("valid-token");
    render(Layout, { props: { children: () => "" } });
    expect(screen.getByText("TestUser")).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /log out/i }),
    ).toBeInTheDocument();
    expect(
      screen.queryByRole("link", { name: /log in/i }),
    ).not.toBeInTheDocument();
  });
});
