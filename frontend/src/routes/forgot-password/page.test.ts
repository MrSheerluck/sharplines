import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Page from "./+page.svelte";
vi.mock("$lib/api/client", () => ({ post: vi.fn() }));
import * as client from "$lib/api/client";
describe("Forgot Password Page", () => {
  beforeEach(() => vi.clearAllMocks());
  it("renders email input", () => {
    render(Page);
    expect(screen.getByLabelText("Email")).toBeInTheDocument();
  });
  it("shows success message after submitting email", async () => {
    vi.mocked(client.post).mockResolvedValueOnce({ data: {}, status: 200 });
    render(Page);
    await userEvent.type(screen.getByLabelText("Email"), "user@example.com");
    await userEvent.click(screen.getByRole("button", { name: /send/i }));
    expect(await screen.findByText(/check your email/i)).toBeInTheDocument();
  });
  it("has a link back to login", () => {
    render(Page);
    expect(
      screen.getByRole("link", { name: /back to login/i }),
    ).toHaveAttribute("href", "/login");
  });
});
