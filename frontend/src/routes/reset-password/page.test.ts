import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Page from "./+page.svelte";
vi.mock("$lib/api/client", () => ({ post: vi.fn() }));
import * as client from "$lib/api/client";
describe("Reset Password Page", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    window.location = { ...window.location, search: "" };
  });
  it("shows error when no token in URL", () => {
    render(Page);
    expect(screen.getByRole("alert")).toHaveTextContent(
      /missing|invalid|token/i,
    );
  });
  it("renders password form with valid token", () => {
    window.location = { ...window.location, search: "?token=abc123" };
    render(Page);
    expect(screen.getByLabelText(/new password/i)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /reset/i })).toBeInTheDocument();
  });
  it("calls reset endpoint on submit", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({ data: {}, status: 200 });
    window.location = { ...window.location, search: "?token=abc123" };
    render(Page);
    await userEvent.type(screen.getByLabelText(/new password/i), "newpass123");
    await userEvent.click(screen.getByRole("button", { name: /reset/i }));
    expect(vi.mocked(client.post)).toHaveBeenCalledWith(
      "/auth/reset-password",
      {
        token: "abc123",
        password: "newpass123",
      },
    );
  });
  it("shows success after reset", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({ data: {}, status: 200 });
    window.location = { ...window.location, search: "?token=abc123" };
    render(Page);
    await userEvent.type(screen.getByLabelText(/new password/i), "newpass123");
    await userEvent.click(screen.getByRole("button", { name: /reset/i }));
    expect(await screen.findByText(/password.*updated/i)).toBeInTheDocument();
  });
});
