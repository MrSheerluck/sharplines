import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/svelte";
import Page from "./+page.svelte";
import userEvent from "@testing-library/user-event";

vi.mock("$lib/stores/auth.svelte", () => ({
  login: vi.fn(),
  getUser: vi.fn(() => null),
}));

describe("Login Page", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });
  it("renders email and password inputs", () => {
    render(Page);
    expect(screen.getByLabelText("Email")).toBeInTheDocument();
    expect(screen.getByLabelText("Password")).toBeInTheDocument();
  });

  it("calls login action on form submit", async () => {
    const { login: mockLogin } = await import("$lib/stores/auth.svelte");

    render(Page);
    await userEvent.type(screen.getByLabelText("Email"), "test@example.com");
    await userEvent.type(screen.getByLabelText("Password"), "password123");
    await userEvent.click(screen.getByRole("button", { name: /log in/i }));
    expect(vi.mocked(mockLogin)).toHaveBeenCalledWith(
      "test@example.com",
      "password123",
    );
  });

  it("shows error message on failed login", async () => {
    const { login: mockLogin } = await import("$lib/stores/auth.svelte");
    vi.mocked(mockLogin).mockRejectedValueOnce(
      new Error("Invalid credentials"),
    );
    render(Page);
    await userEvent.type(screen.getByLabelText("Email"), "test@example.com");
    await userEvent.type(screen.getByLabelText("Password"), "wrong");
    await userEvent.click(screen.getByRole("button", { name: /log in/i }));
    expect(await screen.findByRole("alert")).toHaveTextContent(
      "Invalid credentials",
    );
  });

  it("shows loading state during submission", async () => {
    const { login: mockLogin } = await import("$lib/stores/auth.svelte");
    let resolvePromise!: () => void;
    vi.mocked(mockLogin).mockReturnValueOnce(
      new Promise<void>((resolve) => {
        resolvePromise = resolve;
      }),
    );
    render(Page);
    await userEvent.type(screen.getByLabelText("Email"), "test@example.com");
    await userEvent.type(screen.getByLabelText("Password"), "password123");
    await userEvent.click(screen.getByRole("button", { name: /log in/i }));
    expect(screen.getByRole("button", { name: /logging in/i })).toBeDisabled();
    resolvePromise();
  });

  it("shows validation error for empty email", async () => {
    render(Page);
    await userEvent.type(screen.getByLabelText("Password"), "password123");
    await userEvent.click(screen.getByRole("button", { name: /log in/i }));
    expect(screen.getByRole("alert")).toHaveTextContent("Email is required");
  });

  it("has a link to the register page", () => {
    render(Page);
    expect(screen.getByRole("link", { name: /register/i })).toHaveAttribute(
      "href",
      "/register",
    );
  });

  it("shows Lichess login button", () => {
    render(Page);
    expect(screen.getByRole("link", { name: /lichess/i })).toHaveAttribute(
      "href",
      "http://localhost:3000/auth/lichess",
    );
  });
  it("has a link to forgot password", () => {
    render(Page);
    expect(screen.getByRole("link", { name: /forgot/i })).toHaveAttribute(
      "href",
      "/forgot-password",
    );
  });
});
