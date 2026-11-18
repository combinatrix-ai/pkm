use std::path::Path;
use std::process::{Command, Output};

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pkm"))
        .args(args)
        .current_dir(root)
        .env("HOME", root)
        .env("PKM_NO_UPDATE_CHECK", "1")
        .output()
        .unwrap()
}

fn vault(body: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    std::fs::write(dir.path().join("Tasks.md"), body).unwrap();
    dir
}

fn body(root: &Path) -> String {
    std::fs::read_to_string(root.join("Tasks.md")).unwrap()
}

#[test]
fn done_previews_then_changes_multiple_tasks_in_one_file() {
    let original =
        "# Tasks\n- [ ] First\n- [ ] Second 🔁 every 1 month 📅 2026-10-20\n- [ ] Third\n";
    let dir = vault(original);
    let args = [
        "tasks",
        "done",
        "--file",
        "Tasks.md",
        "First",
        "Second 🔁 every 1 month 📅 2026-10-20",
    ];
    let preview = run(dir.path(), &[&args[..], &["--dry-run"]].concat());
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    assert!(String::from_utf8_lossy(&preview.stdout).contains("[DRY RUN]"));
    assert_eq!(body(dir.path()), original);

    let applied = run(dir.path(), &args);
    assert!(
        applied.status.success(),
        "{}",
        String::from_utf8_lossy(&applied.stderr)
    );
    let changed = body(dir.path());
    assert!(changed.contains("- [x] First ✅ "), "{changed}");
    assert!(
        changed.contains("📅 2026-11-20 ➕ ")
            && changed.contains("\n- [x] Second 🔁 every 1 month 📅 2026-10-20 ✅ "),
        "{changed}"
    );
    assert!(changed.contains("- [ ] Third\n"), "{changed}");
}

#[test]
fn cancel_rejects_missing_ambiguous_and_duplicate_matches_without_writing() {
    let original = "- [ ] Alpha\n- [ ] Alpha again\n- [ ] Beta\n";
    let dir = vault(original);
    for args in [
        vec!["tasks", "cancel", "--file", "Tasks.md", "Beta", "Missing"],
        vec!["tasks", "cancel", "--file", "Tasks.md", "Beta", "Beta"],
        vec![
            "tasks",
            "cancel",
            "--file",
            "Tasks.md",
            "Al",
            "Beta",
            "--partial",
        ],
        vec!["tasks", "cancel", "--file", "Tasks", "Alpha", "Beta"],
        vec!["tasks", "cancel", "Alpha", "Beta"],
    ] {
        let result = run(dir.path(), &args);
        assert!(!result.status.success(), "{args:?}");
        assert_eq!(body(dir.path()), original, "{args:?}");
    }

    let result = run(
        dir.path(),
        &["tasks", "cancel", "--file", "Tasks.md", "Alpha", "Beta"],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let changed = body(dir.path());
    assert!(changed.contains("- [-] Alpha ❌ "), "{changed}");
    assert!(changed.contains("- [ ] Alpha again\n"), "{changed}");
    assert!(changed.contains("- [-] Beta ❌ "), "{changed}");
}

#[test]
fn done_no_recur_applies_to_every_selected_task() {
    let original =
        "- [ ] First 🔁 every 1 month 📅 2026-10-20\n- [ ] Second 🔁 every 1 month 📅 2026-10-20\n";
    let dir = vault(original);
    let result = run(
        dir.path(),
        &[
            "tasks",
            "done",
            "--file",
            "Tasks.md",
            "First 🔁 every 1 month 📅 2026-10-20",
            "Second 🔁 every 1 month 📅 2026-10-20",
            "--no-recur",
        ],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let changed = body(dir.path());
    assert_eq!(changed.matches("- [x]").count(), 2, "{changed}");
    assert!(!changed.contains("- [ ]"), "{changed}");
}

#[test]
fn two_recurring_tasks_keep_each_next_occurrence_above_its_completion() {
    let dir = vault(
        "- [ ] First 🔁 every 1 month 📅 2026-10-20\n- [ ] Second 🔁 every 1 month 📅 2026-10-20\n",
    );
    let result = run(
        dir.path(),
        &[
            "tasks",
            "done",
            "--file",
            "Tasks.md",
            "First 🔁 every 1 month 📅 2026-10-20",
            "Second 🔁 every 1 month 📅 2026-10-20",
        ],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let changed = body(dir.path());
    let lines: Vec<_> = changed.lines().collect();
    assert_eq!(lines.len(), 4, "{changed}");
    assert!(lines[0].starts_with("- [ ] First "), "{changed}");
    assert!(lines[1].starts_with("- [x] First "), "{changed}");
    assert!(lines[2].starts_with("- [ ] Second "), "{changed}");
    assert!(lines[3].starts_with("- [x] Second "), "{changed}");
    let report = String::from_utf8_lossy(&result.stdout);
    assert!(report.contains("Tasks.md:1"), "{report}");
    assert!(report.contains("Tasks.md:2"), "{report}");
    assert!(report.contains("Tasks.md:3"), "{report}");
    assert!(report.contains("Tasks.md:4"), "{report}");
}

#[test]
fn status_changes_require_full_body_and_show_plausible_matches() {
    let original = "- [ ] Buy milk 📅 2026-11-16\n";
    let dir = vault(original);
    for args in [
        vec!["tasks", "done", "--file", "Tasks.md", "Buy milk"],
        vec!["tasks", "cancel", "--file", "Tasks.md", "Buy milk"],
        vec!["tasks", "wip", "--file", "Tasks.md", "Buy milk"],
        vec!["tasks", "set", "done", "--file", "Tasks.md", "Buy milk"],
    ] {
        let result = run(dir.path(), &args);
        assert!(!result.status.success(), "{args:?}");
        let error = String::from_utf8_lossy(&result.stderr);
        assert!(
            error.contains("No exact task found: \"Buy milk\""),
            "{error}"
        );
        assert!(
            error.contains("Possible match: \"Buy milk 📅 2026-11-16\" (Tasks.md:1)"),
            "{error}"
        );
        assert!(error.contains("--partial"), "{error}");
        assert_eq!(body(dir.path()), original, "{args:?}");
    }

    let result = run(
        dir.path(),
        &[
            "tasks",
            "done",
            "--file",
            "Tasks.md",
            "Buy milk",
            "--partial",
        ],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(body(dir.path()).contains("- [x] Buy milk 📅 2026-11-16 ✅ "));
}

#[test]
fn batch_rejects_shortened_text_without_changing_any_task() {
    let original = "- [ ] Alpha\n- [ ] Buy milk 📅 2026-11-16\n";
    let dir = vault(original);
    let result = run(
        dir.path(),
        &["tasks", "cancel", "--file", "Tasks.md", "Alpha", "Buy milk"],
    );
    assert!(!result.status.success());
    assert_eq!(body(dir.path()), original);
    let error = String::from_utf8_lossy(&result.stderr);
    assert!(error.contains("Buy milk 📅 2026-11-16"), "{error}");
    assert!(error.contains("Tasks.md:2"), "{error}");
}

#[test]
fn suggestions_respect_file_scope_and_stop_at_three() {
    let dir =
        vault("- [ ] Buy milk 📅 2026-11-16\n- [ ] Buy bread\n- [ ] Buy eggs\n- [ ] Buy butter\n");
    std::fs::write(dir.path().join("Other.md"), "- [ ] Buy cheese\n").unwrap();
    let result = run(dir.path(), &["tasks", "done", "--file", "Tasks.md", "Buy"]);
    assert!(!result.status.success());
    let error = String::from_utf8_lossy(&result.stderr);
    assert_eq!(error.matches("Possible match:").count(), 3, "{error}");
    assert!(!error.contains("Other.md"), "{error}");
    assert!(!error.contains("Buy cheese"), "{error}");
}
