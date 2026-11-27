# pkm tasks

```text
Find and filter tasks (- [ ] items) in the vault

Usage: pkm tasks [OPTIONS] [COMMAND]

Commands:
  new     Create a new task
  done    Mark a task as done, adding the next occurrence of a recurring one
  wip     Mark a task as in progress
  cancel  Mark a task as cancelled
  set     Transition a task to a named status
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>
          Path to config file
  -v, --vault <VAULT>
          Path to vault (overrides config)
      --all
          Include completed tasks
      --done
          Show only completed tasks
      --recurring
          Show only recurring tasks
      --overdue
          Show only overdue tasks
      --has-date
          Show only tasks with any date marker
      --available
          Hide tasks whose start date is after today
      --due-on <DUE_ON>

      --due-from <DUE_FROM>

      --due-to <DUE_TO>

      --scheduled-on <SCHEDULED_ON>

      --scheduled-from <SCHEDULED_FROM>

      --scheduled-to <SCHEDULED_TO>

      --start-on <START_ON>

      --start-from <START_FROM>

      --start-to <START_TO>

      --created-from <CREATED_FROM>

      --created-to <CREATED_TO>

      --done-from <DONE_FROM>

      --done-to <DONE_TO>

      --file <FILE>
          Filter by file/folder path
      --exclude <EXCLUDE>
          Exclude files/folders (repeatable)
      --tag <TAG>
          Require this exact tag (repeat to require all)
      --exclude-tag <TAG>
          Exclude results with this exact tag (repeat to exclude any)
      --status <STATUS>
          Filter by status name (repeatable)
  -A, --after <AFTER>
          Lines of context after task
  -B, --before <BEFORE>
          Lines of context before task
  -C, --context <CONTEXT>
          Lines of context on both sides
  -b, --breadcrumbs
          Show heading hierarchy
      --group <GROUP>
          Group results [possible values: file, date]
      --prose-only
          Skip checkboxes in code fences, %%comments%% and inline code
  -f, --format <FORMAT>
          [default: text]
  -h, --help
          Print help
```

## pkm tasks new

```text
Create a new task

Usage: pkm tasks new [OPTIONS] <TEXT>

Arguments:
  <TEXT>

Options:
  -c, --config <CONFIG>        Path to config file
      --file <FILE>            Target file (overrides config default)
      --due <DUE>              Due date (YYYY-MM-DD)
  -v, --vault <VAULT>          Path to vault (overrides config)
      --scheduled <SCHEDULED>  Scheduled date (YYYY-MM-DD)
      --start <START>          Start date (YYYY-MM-DD)
      --recurring <RECURRING>  Recurrence (e.g. 'every 3 months')
      --priority <PRIORITY>    [possible values: high, medium, low]
  -h, --help                   Print help
```

## pkm tasks done

```text
Mark a task as done, adding the next occurrence of a recurring one

Usage: pkm tasks done [OPTIONS] [TEXT]...

Arguments:
  [TEXT]...

Options:
  -c, --config <CONFIG>  Path to config file
      --file <FILE>      Vault-relative file (required for multiple texts)
      --partial          Use partial matching (contains)
  -v, --vault <VAULT>    Path to vault (overrides config)
      --no-recur         Do not create the next occurrence of a recurring task
      --dry-run
  -h, --help             Print help
```

## pkm tasks wip

```text
Mark a task as in progress

Usage: pkm tasks wip [OPTIONS] [TEXT]

Arguments:
  [TEXT]

Options:
  -c, --config <CONFIG>  Path to config file
      --file <FILE>
      --partial
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## pkm tasks cancel

```text
Mark a task as cancelled

Usage: pkm tasks cancel [OPTIONS] [TEXT]...

Arguments:
  [TEXT]...

Options:
  -c, --config <CONFIG>  Path to config file
      --file <FILE>      Vault-relative file (required for multiple texts)
      --partial
  -v, --vault <VAULT>    Path to vault (overrides config)
      --dry-run
  -h, --help             Print help
```

## pkm tasks set

```text
Transition a task to a named status

Usage: pkm tasks set [OPTIONS] <STATUS> [TEXT]

Arguments:
  <STATUS>
  [TEXT]

Options:
  -c, --config <CONFIG>  Path to config file
      --file <FILE>
      --partial
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```
