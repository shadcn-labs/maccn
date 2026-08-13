import type { ReactNode } from "react";

/**
 * Embeds the interactive WASM demo for a maccn component.
 *
 * The demo is a GPUI app compiled to WebAssembly and built into
 * `public/examples` (see `crates/maccn/examples/wasm`). Each component page
 * passes its slug as `component`, and the demo renders it inside a window
 * frame that mirrors a native macOS window.
 */
export const ComponentPreview = ({
  component,
  children,
}: {
  /** The showcase slug, e.g. `"button"`. */
  component: string;
  children?: ReactNode;
}) => (
  <>
    {children}
    <section className="component-example not-prose my-6">
      <div className="component-example__label">
        <span>Example</span>
        <span className="component-example__live">Rust &amp; WASM</span>
      </div>
      <div className="mac-window">
        <div className="mac-window__bar">
          <span className="mac-window__lights" aria-hidden="true">
            <i />
            <i />
            <i />
          </span>
          <span className="mac-window__title">{component} — maccn</span>
        </div>
        <iframe
          src={`/examples/?component=${encodeURIComponent(component)}`}
          title={`${component} interactive example`}
          allow="cross-origin-isolated"
        />
      </div>
    </section>
  </>
);
