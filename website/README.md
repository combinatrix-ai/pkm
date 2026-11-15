# Website

The website uses VitePress. The landing page and quick start live here; the
CLI manual and compatibility matrix are generated from `docs/` before each
build. Edit those source files, not `website/reference/`.

```sh
npm ci
npm run docs:dev
npm run docs:build
npm run docs:preview
```

The output is `website/.vitepress/dist/`, configured for `/knapper/`.
The PR builds the site but does not deploy it. Publish the output through the
organization's website pipeline when `combinatrix.ai/knapper/` is ready.

VitePress is pinned to 2.0.0-alpha.21. The older stable series currently pulls
dependencies with reported vulnerabilities; run `npm audit` when updating.
The generated output is static and needs no Node.js runtime in production.
