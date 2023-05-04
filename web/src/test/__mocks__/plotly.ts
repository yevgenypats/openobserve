import { vi } from "vitest";

vi.mock("plotly.js", () => {
  return {
    default: {
      newPlot: vi.fn((ref) => {
        if (typeof ref === "string") ref = document.getElementById(ref);
        ref.on = vi.fn();
      }),
      addTraces: vi.fn(),
      restyle: vi.fn(),
      relayout: vi.fn(),
      redraw: vi.fn(),
      react: vi.fn(),
    },
  };
});
