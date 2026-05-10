import { render, screen } from "@testing-library/svelte";
import Page from "./+page.svelte";

describe("Homepage", () => {
  it("renders without errors", () => {
    render(Page);
    expect(screen.getByText("Welcome to SvelteKit")).toBeInTheDocument();
  });
});
