# Tasks

Use explicit `YYYY-MM-DD` values for task date filters and creation flags.
`tasks new` uses `--file` when supplied, otherwise `tasks.default_file`.
Use `--file` to scope status changes and `--partial` only when substring
matching is intended. The full task filter and subcommand options appear
in the [`pkm tasks` reference](../reference/commands/tasks.md).

Tasks are plain markdown checkboxes. The emoji date convention popularised by
the [Obsidian Tasks](https://publish.obsidian.md/tasks/) plugin (`📅` due,
`⏳` scheduled, `🛫` start, `✅` done, `🔁` recurring) is read and written but never
required — a vault of bare `- [ ]` items works fine.

Tasks are selected by their full body, which `tasks --format json` returns
as `text`. An Obsidian block ID at the end of a task, such as `^review-7c4`,
is part of that body; status changes keep it at the end of the task.
Completing a recurring task leaves its identity on the completed occurrence,
as the Tasks plugin does: the next occurrence gets no block ID, no `🆔`, and
no `⛔`, so links and dependent tasks keep pointing at the task that was done.

```bash
pkm tasks --overdue                     # past due
pkm tasks --available                   # no start date, or start date <= today
pkm tasks --due-from 2026-08-01 --due-to 2026-08-31
pkm tasks --start-on 2026-08-01
pkm tasks --scheduled-from 2026-08-01 --scheduled-to 2026-08-07
pkm tasks --exclude Archive/ --tag work
pkm tasks new "write the README" --start 2026-07-28 --due 2026-08-01
pkm tasks done "write the README"       # full task body, including any markers
pkm tasks done --file Tasks.md "first" "second" --dry-run
pkm tasks done --file Tasks.md "first" "second"
pkm tasks done "pay rent" --no-recur    # complete without the next occurrence
pkm tasks --prose-only                  # skip checkboxes in fenced examples
```

`tasks done` and `tasks cancel` accept multiple task texts with an exact
vault-relative `--file`. Every text must match one distinct task in that file.
The command checks all matches before writing once; an unmatched, ambiguous,
or repeated match leaves the file unchanged. `--dry-run` previews the changes
without writing. All task status changes match the full checkbox body exactly
by default, including date and recurrence markers. `--partial` opts into
substring matching. A failed exact match reports up to three plausible full
texts with file and line. One-text calls keep their optional `--file` scope.

Every checkbox counts by default, including one written inside a code fence or
a `%%comment%%` — the one place pkm does not mask non-prose, and
deliberately: masking can only ever *hide* a task, and a visible example costs
less than a missing entry on a list you act on. `--prose-only` is the opt-out
for a vault that documents its own conventions.

Completing a recurring task with `tasks done` (or `tasks set done`) writes its
next occurrence on the line above, as the Tasks plugin does. `🔁 every N
days|weeks|months|years` advances from the due date (else scheduled, else
start); `when done` advances from today. The other dates keep their offset,
the created date becomes today, and a month-end date clamps (Jan 31 → Feb 28).
Richer rules such as `every week on Monday` are left alone with a warning;
`--no-recur` skips the next occurrence.

`--available` is also opt-in: it hides only tasks whose `🛫` start date is in
the future. Tasks without a start date remain visible, and plain `pkm
tasks` keeps its established whole-list behaviour. A scheduled date says when
you intend to work; it does not make a task unavailable.

Statuses are configurable: `open` `[ ]`, `wip` `[/]`, `done` `[x]`, and
`cancel` `[-]` are built in, and `pkm.yaml` can override their
markers or add custom statuses, which `pkm tasks set STATUS TEXT` and
`--status` filters then understand.

## Which date counts as today?

Task completion dates and overdue checks use the local date of the process
running pkm. Set the standard `TZ` environment variable when a VM, scheduled
job, or developer machine should use the same calendar day:

```sh
TZ=Asia/Tokyo pkm tasks done --file Projects/Release.md "Review release notes"
TZ=Asia/Tokyo pkm tasks --overdue
```
