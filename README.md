<p align="center">
  <img src="https://maccn.vercel.app/og" alt="maccn banner" />
</p>

<h1 align="center">maccn</h1>

<p align="center">
  macOS-inspired UI components for Rust applications.
</p>

## What's inside

- 🍎 **19 components** — Badge, Box, Button, Checkbox, Glass Panel, Help Button, Label, Pop-Up Button, Progress, Radio Group, Search Field, Secure Field, Segmented Control, Separator, Slider, Spinner, Stepper, Switch, and Text Field
- 📏 **Five control sizes** — `mini`, `small`, `regular`, `large`, and `extraLarge` on every control that has them
- 🌗 **Light and dark** — driven by a single theme on any element
- 🎨 **System accents** — every control follows one accent color

## Repository layout

This is a pnpm + Cargo monorepo:

```
├── crates/
│   └── maccn/                  # the component library (Rust)
│       ├── examples/showcase.rs# native demo
│       └── examples/wasm/      # browser demo (built into docs/public/examples)
├── docs/                       # Next.js documentation site (Fumadocs)
├── Cargo.toml                  # Rust workspace
├── package.json                # pnpm workspace root
└── pnpm-workspace.yaml
```

## Getting started

Run the native component demo:

```bash
cargo run -p maccn --example showcase                # open the overview
cargo run -p maccn --example showcase -- switch      # open one component
```

Run the docs site:

```bash
pnpm install
pnpm dev
```

Build the browser demos into the docs site (required for the live examples on each component page):

```bash
make -C crates/maccn/examples/wasm install   # once: wasm target + wasm-bindgen + deps
make -C crates/maccn/examples/wasm build     # outputs to docs/public/examples
```

## Deploying the docs site

Deploy the entire Next.js app like any other Next.js project — no special
build step is required. The browser demos are plain static assets under
`docs/public/examples`, so they deploy alongside the site automatically.

### 1. Build

```bash
pnpm install
pnpm build
```

### 2. Serve

Run the production server on the platform of your choice:

```bash
pnpm start        # runs `next start` (defaults to 0.0.0.0:3000)
```

`next start` reads `PORT` and `HOSTNAME` from the environment:

```bash
PORT=8080 HOSTNAME=0.0.0.0 SITE_URL=https://components.example.com pnpm start
```

### 3. Reverse proxy (optional)

Put Nginx or Caddy in front of it for TLS and a public hostname:

```nginx
# /etc/nginx/sites-available/maccn
server {
  listen 443 ssl;
  server_name components.example.com;

  location / {
    proxy_pass http://127.0.0.1:3000;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
    proxy_set_header Host $host;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
  }
}
```

Or, with Caddy:

```caddy
components.example.com {
  reverse_proxy 127.0.0.1:3000
}
```

### 4. Keep it running

Use a process manager so the server restarts and starts on boot:

```bash
# pm2
pm2 start "pnpm start" --name maccn-docs
pm2 save && pm2 startup

# or systemd
# /etc/systemd/system/maccn-docs.service
#   WorkingDirectory=/srv/maccn
#   ExecStart=/usr/bin/pnpm start
```

### Docker

```dockerfile
FROM node:22-alpine AS deps
WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY docs/package.json docs/
RUN corepack enable && pnpm install --frozen-lockfile

FROM node:22-alpine AS build
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN pnpm build

FROM node:22-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production
COPY --from=build /app .
RUN corepack enable
EXPOSE 3000
ENV PORT=3000 HOSTNAME=0.0.0.0
CMD ["pnpm", "start"]
```

## Scripts

- `pnpm dev` — run the docs site
- `pnpm build` / `pnpm start` — build and serve the docs site
- `pnpm typecheck` — typecheck the docs site
- `pnpm check` / `pnpm fix` — lint and format the whole repo (ultracite)
- `pnpm wasm:build` — build the browser demos into `docs/public/examples`

## License

[MIT](./LICENSE)

macOS and AppKit are trademarks of Apple Inc. This project is not affiliated with, endorsed by, or sponsored by Apple.
