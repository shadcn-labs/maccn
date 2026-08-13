import { HomeCtas } from "@/components/home-ctas";
import { PageTransition } from "@/components/page-transition";
import { ROUTES } from "@/constants/routes";
import { BreadcrumbJsonLd } from "@/seo/json-ld";

export const dynamic = "force-static";
export const revalidate = false;

export default function IndexPage() {
  return (
    <>
      <BreadcrumbJsonLd items={[{ name: "Home", path: ROUTES.HOME }]} />
      <PageTransition>
        <section className="container-wrapper relative">
          <div className="container flex flex-col items-center gap-4 py-16 text-center md:py-20 lg:py-24">
            <h1 className="from-foreground via-foreground to-foreground/65 max-w-7xl bg-linear-to-b bg-clip-text text-4xl font-bold tracking-tight text-transparent sm:text-5xl md:text-6xl">
              maccn
            </h1>

            <p className="max-w-2xl text-lg text-muted-foreground sm:text-xl">
              macOS-inspired controls, built for GPUI on top of gpui-base.
              AppKit metrics, light and dark appearances, and a system accent —
              rendered live in your browser via WebAssembly.
            </p>

            <HomeCtas className="mt-4" />
          </div>
        </section>

        <section className="container-wrapper pb-8 lg:pb-12">
          <div className="container flex flex-col items-center gap-6">
            <div className="component-example w-full max-w-3xl">
              <div className="component-example__label">
                <span>Live demo</span>
                <span className="component-example__live">Rust &amp; WASM</span>
              </div>
              <div className="mac-window">
                <div className="mac-window__bar">
                  <span className="mac-window__lights" aria-hidden="true">
                    <i />
                    <i />
                    <i />
                  </span>
                  <span className="mac-window__title">maccn — Overview</span>
                </div>
                <iframe
                  src="/examples/?component=overview"
                  title="maccn overview example"
                  allow="cross-origin-isolated"
                />
              </div>
            </div>
          </div>
        </section>
      </PageTransition>
    </>
  );
}
