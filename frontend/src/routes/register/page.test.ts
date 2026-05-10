import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import Page from "./+page.svelte";
vi.mock("$lib/stores/auth.svelte", () => ({
  register: vi.fn(),
  getUser: vi.fn(() => null),
}));
describe("Register Page", () => {
  it("renders email, password, and display name inputs", () => {
    render(Page);
    expect(screen.getByLabelText("Email")).toBeInTheDocument();
    expect(screen.getByLabelText("Password")).toBeInTheDocument();
    expect(screen.getByLabelText("Display Name")).toBeInTheDocument();
  });

  it("calls register action on form submit", async () => {
    const { register: mockRegister } = await import("$lib/stores/auth.svelte");
    vi.mocked(mockRegister).mockResolvedValueOnce(undefined);
    render(Page);
    await userEvent.type(screen.getByLabelText("Email"), "new@example.com");
    await userEvent.type(screen.getByLabelText("Password"), "password123");
    await userEvent.type(screen.getByLabelText("Display Name"), "NewUser");
    await userEvent.click(screen.getByRole("button", { name: /register/i }));
    expect(vi.mocked(mockRegister)).toHaveBeenCalledWith(
      "new@example.com",
      "password123",
      "NewUser",
    );
  });
  it("shows error message on failed registration", async () => {
    const { register: mockRegister } = await import("$lib/stores/auth.svelte");
    vi.mocked(mockRegister).mockRejectedValueOnce(
      new Error("Email already exists"),
    );
    render(Page);
    await userEvent.type(
      screen.getByLabelText("Email"),
      "existing@example.com",
    );
    await userEvent.type(screen.getByLabelText("Password"), "password123");
    await userEvent.click(screen.getByRole("button", { name: /register/i }));
    expect(await screen.findByRole("alert")).toHaveTextContent(
      "Email already exists",
    );
  });
  it("shows validation error for short password", async () => {
    render(Page);
    await userEvent.type(screen.getByLabelText("Email"), "test@example.com");
    await userEvent.type(screen.getByLabelText("Password"), "123");
    await userEvent.click(screen.getByRole("button", { name: /register/i }));
    expect(screen.getByRole("alert")).toHaveTextContent(
      "at least 8 characters",
    );
  });
  it("shows success message after registration", async () => {
    const { register: mockRegister } = await import("$lib/stores/auth.svelte");
    vi.mocked(mockRegister).mockResolvedValueOnce(undefined);
    render(Page);
    await userEvent.type(screen.getByLabelText("Email"), "new@example.com");
    await userEvent.type(screen.getByLabelText("Password"), "password123");
    await userEvent.click(screen.getByRole("button", { name: /register/i }));
    expect(await screen.findByText(/check your email/i)).toBeInTheDocument();
  });
  it("has a link to the login page", () => {
    render(Page);
    expect(screen.getByRole("link", { name: /log in/i })).toHaveAttribute(
      "href",
      "/login",
    );
  });
});
