import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import ProtectedRoute from "./ProtectedRoute.svelte";
vi.mock("$lib/stores/auth.svelte", () => ({
  getUser: vi.fn(),
  getAccessToken: vi.fn(),
}));
import * as auth from "$lib/stores/auth.svelte";
const mockUser = {
  id: "1",
  email: "test@test.com",
  display_name: null,
  lichess_user_id: null,
  email_verified: true,
  created_at: "",
};
describe("ProtectedRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // @ts-ignore
    delete window.location;
    window.location = { href: "" } as any;
  });
  it("redirects to login when unauthenticated", () => {
    vi.mocked(auth.getUser).mockReturnValue(null);
    vi.mocked(auth.getAccessToken).mockReturnValue(null);
    render(ProtectedRoute, { props: { children: () => "" } });
    expect(window.location.href).toBe("/login");
  });
  it("does not redirect when authenticated", () => {
    vi.mocked(auth.getUser).mockReturnValue(mockUser);
    vi.mocked(auth.getAccessToken).mockReturnValue("token");
    render(ProtectedRoute, { props: { children: () => "" } });
    expect(window.location.href).not.toBe("/login");
  });
});
