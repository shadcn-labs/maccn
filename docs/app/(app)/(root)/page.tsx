import { HomeBento } from "@/components/home-bento";
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
        <section className="container-wrapper">
          <div className="container flex flex-col items-center gap-4 py-16 text-center md:py-20 lg:py-24">
            <h1 className="from-foreground via-foreground to-foreground/65 max-w-7xl bg-linear-to-b bg-clip-text text-4xl font-bold tracking-tight text-transparent sm:text-5xl md:text-6xl">
              Beautiful macOS UIs, made simple
            </h1>

            <p className="max-w-2xl text-lg text-muted-foreground sm:text-xl">
              Drop in controls with AppKit metrics, states, and keyboard
              behavior. Built on GPUI.
            </p>

            <HomeCtas className="mt-4" />
          </div>
        </section>

        <section className="container-wrapper">
          <div className="container pb-12 md:pb-16 lg:pb-24">
            <HomeBento />
          </div>
        </section>
      </PageTransition>
    </>
  );
}
