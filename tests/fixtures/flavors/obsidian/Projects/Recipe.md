---
status: open
related: "[[Notes/Lit Review]]"
tags: [research]
---

# Recipe

Depends on [[Notes/Lit Review]] and [[Notes/Lit Review#Method]].
Embedded: ![[Notes/Lit Review]]
By alias: [[文献レビュー]]

```python
# this is a code sample, not a reference
doc = "[[Not A Link]]"  # and #notatag
```

%% [[Also Not A Link]] %%

Inline `[[Nor This]]` too.

#project/active #日本語タグ

- [ ] draft intro 📅 2026-08-01 ⏫
- [ ] review methods ⏳ 2026-08-05 🛫 2026-08-02
- [x] outline ✅ 2026-07-20
- [/] methods section

住所: [請求書送付先の住所](pkm://personal/address.billing)
Deploy: [CI token](<pkm://work/tokens/ci.deploy>) and [DB](pkm://work/db/prod "primary")

`Inline [x](pkm://personal/not-inline)`

%% [x](pkm://personal/not-comment) %%

```text
[x](pkm://personal/not-fenced)
```

Not linked: pkm://personal/not-bare and <pkm://personal/not-autolink>

Malformed: [a](pkm://Personal/upper) [b](pkm://personal/a//b)
[c](pkm://personal/../etc) [d](pkm://personal/a%2Fb) [e](pkm://personal/has space)
