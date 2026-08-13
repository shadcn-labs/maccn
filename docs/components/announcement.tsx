import { ArrowRightIcon } from "lucide-react";
import Link from "next/link";

import { Badge } from "@/components/ui/badge";
import { ROUTES } from "@/constants/routes";

export const Announcement = () => (
  <Badge asChild variant="secondary" className="bg-transparent">
    <Link href={ROUTES.DOCS_COMPONENTS}>
      <span className="flex size-2 rounded-full bg-blue-500" title="New" />
      macOS-inspired components for GPUI <ArrowRightIcon />
    </Link>
  </Badge>
);
