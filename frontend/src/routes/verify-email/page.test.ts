import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import Page from "./+page.svelte";
vi.mock("$lib/api/client", () => ({
  get: vi.fn(),
}));
import * as client from "$lib/api/client";
describe("Verify Email Page", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    window.location = { ...window.location, search: "" };
  });
  it("shows error when no token in URL", () => {
    render(Page);
    expect(screen.getByRole("alert")).toHaveTextContent(/invalid|missing/i);
  });
  it("shows verifying state initially with token", () => {
    vi.mocked(client.get).mockReturnValueOnce(new Promise(() => {}));
    window.location = { ...window.location, search: "?token=abc123" };
    render(Page);
    expect(screen.getByText(/verifying/i)).toBeInTheDocument();
  });
  it("shows success on valid token", async () => {
    vi.mocked(client.get).mockResolvedValueOnce({ data: {}, status: 200 });
    window.location = { ...window.location, search: "?token=abc123" };
    render(Page);
    expect(await screen.findByText(/verified|success/i)).toBeInTheDocument();
  });
  it("shows error on expired token", async () => {
    vi.mocked(client.get).mockResolvedValueOnce({
      error: "Token expired",
      status: 400,
    });
    window.location = { ...window.location, search: "?token=expired" };
    render(Page);
    expect(await screen.findByRole("alert")).toHaveTextContent(/expired/i);
  });
});
