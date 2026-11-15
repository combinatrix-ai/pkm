# Website

The website uses VitePress. Only the landing page lives here. The guides, the
CLI reference and the compatibility matrix are generated from `docs/` before
each build, so the same files read well on GitHub. Edit `docs/`, not
`website/guide/` or `website/reference/`. The build also writes
`llms-full.txt`, every page in one file.

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

Command output on the landing page is real output from a synthetic vault.
Regenerate it with `cargo build --bin knapper` and
`python3 scripts/capture-examples.py`. The questions and the agent's replies
are written by hand in that script; keep them consistent with the output. No
user notes or live account data are used.

Set `KNAPPER_SITE_BASE` to build for a path other than `/knapper/`.
