# External references

Some things belong in your notes by name but not by value: an address, a
licence key, a token. Write them as an ordinary markdown link with a
`knapper://` destination, and the value stays wherever you already keep it:

```markdown
Address: [Billing address for invoices](knapper://personal/address.billing)
```

A reference is `knapper://<provider>/<locator>`. The **provider** is a name
*you* chose — `personal`, `work`, `family` — and the **locator** is opaque to
knapper: it is never decoded or normalised, and reaches the provider command
as written. Both halves still have a deliberate grammar. Provider names are
lowercase letters, digits, `_` and `-`; locators use ASCII letters, digits,
`.`, `_`, `-` and `/`, are at most 128 characters, and have no empty, `.` or
`..` segments. A destination outside that grammar is not a reference: `refs`
will not list it and `resolve` will refuse it. Keep locators machine-readable —
`address.billing`, not `Street Address`.

## Finding references

```bash
knapper refs                          # every reference in the vault
knapper refs "Projects/application.md"
knapper refs --provider personal --format json
knapper refs --format paths           # just the files, ready for a pipe
```

Each result carries the vault-relative path, line, column, URI, provider,
locator and link label. References inside fenced code, inline code and
`%%comments%%` are ignored, as links there always are, and `knapper context
FILE --format json` reports the same under `references`.

A `knapper://` link is *external*: it never becomes an edge in the link
graph, never shows up as a broken link, and `rename` and `move` leave it
alone — exactly like an `https://` link.

## Configuring a provider

What actually runs lives in `$XDG_CONFIG_HOME/knapper/providers.yaml`
(`~/.config/knapper/providers.yaml` if that is unset) — **never** in
`knapper.yaml`. A vault is synced, shared and cloned; a file that decides
what gets executed must not travel with it. knapper reads no provider
configuration from a vault, and a `providers:` block in a vault config is
refused.

```bash
knapper provider set personal -- op read 'op://Knapper/{locator}/value'
knapper provider set family -- pass show 'knapper/{locator}'
knapper provider list
knapper provider remove personal
```

```yaml
providers:
  personal:
    command: [op, read, "op://Knapper/{locator}/value"]
```

`command` is **argv**, not a shell line: knapper execs it directly, so there
is no quoting, no word splitting and no shell to escape from. Every
`{locator}` in it — in any argument — is replaced with the reference's
locator, and nothing else is substituted.

knapper is provider-agnostic. `op` above is one user's choice; `pass`,
`gopass`, `security`, `bw`, `gcloud secrets`, `vault read`, or a script of
your own work the same way, because knapper knows nothing about any of them.

## Resolving

```bash
knapper resolve "knapper://personal/address.billing"
knapper resolve "knapper://work/tokens/ci.deploy" --dry-run    # print the argv, run nothing
knapper resolve "knapper://work/tokens/ci.deploy" --timeout 30
```

The provider's stdout is the value. Exactly one trailing newline comes off;
after that, the remaining text, including multiple lines, is passed through
unchanged. A value must be UTF-8 text of at most 1 MiB — a reference names a
credential or an address, not a payload; anything larger or binary is refused
with exit 4.
stdin and stderr stay attached to your terminal, so a provider can still
prompt for a PIN, a passphrase or a hardware key.

knapper adds no newline of its own. This keeps redirection byte-exact; when
run directly in a terminal, the next prompt may therefore follow the value.

There is no default timeout — a provider may legitimately wait for you —
`--timeout SECS` bounds the whole resolve, including the wait for provider
stdout to close, and kills the provider process if it is still running.
Provider commands remain responsible for any further processes they start.

What knapper does *not* do: it never caches a resolved value, never writes one
anywhere, never scans one for further references, and never puts one in a
child process's arguments or environment. Failures are distinguishable by exit
status: **2** for a malformed reference or bad usage, **3** for a provider that
is not configured, **4** for a provider command that would not run, failed,
timed out, or returned nothing usable.
