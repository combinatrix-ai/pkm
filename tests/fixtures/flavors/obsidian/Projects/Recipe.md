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

住所: [請求書送付先の住所](knapper://personal/address.billing)
Deploy: [CI token](<knapper://work/tokens/ci.deploy>) and [DB](knapper://work/db/prod "primary")

`Inline [x](knapper://personal/not-inline)`

%% [x](knapper://personal/not-comment) %%

```text
[x](knapper://personal/not-fenced)
```

Not linked: knapper://personal/not-bare and <knapper://personal/not-autolink>

Malformed: [a](knapper://Personal/upper) [b](knapper://personal/a//b)
[c](knapper://personal/../etc) [d](knapper://personal/a%2Fb) [e](knapper://personal/has space)
