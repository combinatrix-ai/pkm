# Install

Download the binary for your platform from
[Releases](https://github.com/combinatrix-ai/knapper/releases). It is one
file, about 5 MB, with nothing to install alongside it — no interpreter, no
virtualenv, no dependency resolution. That is the point: knapper is for
machines where you would rather not stand up a runtime.

```bash
curl -fsSL https://raw.githubusercontent.com/combinatrix-ai/knapper/v0.1.0/install.sh | sh
```

That puts the binary in `~/.local/bin`, verifies it against the release's
versioned checksum manifest, and registers the agent skill with whichever of Claude Code and
Codex it finds. `--bin-dir`, `--version` and `--skill none` change all three;
`--help` lists them.

Or take the archive yourself, from
[Releases](https://github.com/combinatrix-ai/knapper/releases):

Choose the archive matching your OS and CPU. v0.1.0 supports macOS arm64 /
x86_64 and Linux arm64 / x86_64 (musl). Windows is not a release target.

Verify its GitHub Actions provenance before extracting it:

```bash
gh attestation verify knapper-v0.1.0-aarch64-apple-darwin.tar.gz --repo combinatrix-ai/knapper
```

Linux builds are musl-linked, so they run on Alpine and in a scratch
container.

Because there is no package manager in that story, knapper upgrades itself:

```bash
knapper self-update --check   # is there a newer release?
knapper self-update           # install it
```

The updater verifies the checksum manifest's Sigstore provenance against the
repository's numeric identity, release tag and release workflow, then passes
the authenticated archive digest to its embedded installer. It never downloads
a mutable installer to execute. Installation is atomic in the current binary's
directory, which must be writable. `--yes` confirms noninteractive updates;
`--force` reinstalls the latest version; `--no-skill` preserves managed skills.

Automatic checks only notify; they never replace the executable. They run only
with a terminal on stderr, outside CI, and leave stdout unchanged. Version and
check-time cache files live under `$XDG_CACHE_HOME/knapper` (default
`~/.cache/knapper`). No notes or provider values are sent. A failed check retains
the last known notice. Explicit updates and configured provider commands may
also use the network. Deleting the cache clears update-check state.

For the first installation, the public installer verifies archive checksums;
use `gh attestation verify` above for independent provenance verification.
The installed binary's updater performs provenance verification itself.

<details>
<summary>From source</summary>

```bash
cargo install --git https://github.com/combinatrix-ai/knapper knapper
```

</details>
