use std::path::Path;
use std::process::Command;

fn change_status(root: &Path, args: &[&str]) -> String {
    let output = run_status(root, args);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn run_status(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_pkm"))
        .args(args)
        .current_dir(root)
        .env("HOME", root)
        .env("PKM_NO_UPDATE_CHECK", "1")
        .output()
        .unwrap()
}

#[test]
fn status_changes_keep_the_block_id_at_the_end() {
    for command in [
        vec!["done", "Milk ^milk-7c4"],
        vec!["cancel", "Milk ^milk-7c4"],
        vec!["wip", "Milk ^milk-7c4"],
        vec!["set", "done", "Milk ^milk-7c4"],
    ] {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
        std::fs::write(dir.path().join("Tasks.md"), "- [ ] Milk ^milk-7c4\n").unwrap();
        let mut args = vec!["tasks"];
        args.extend(command);
        change_status(dir.path(), &args);
        let updated = std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap();
        assert!(updated.lines().next().unwrap().ends_with(" ^milk-7c4"));
    }
}

/// Identity stays with the completed occurrence, as in the Tasks plugin: the
/// next occurrence gets no block ID, `🆔`, or `⛔`, so links and dependents
/// keep pointing at the task that was done.
#[test]
fn a_recurring_task_leaves_its_ids_on_the_completed_occurrence() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    let original = "Water 🆔 water1 ⛔ seed1,seed2 🔁 every day 📅 2026-11-16 ^water-7c4";
    std::fs::write(
        dir.path().join("Tasks.md"),
        format!("- [ ] {original}\n- [ ] Harvest ⛔ water1\n"),
    )
    .unwrap();
    change_status(dir.path(), &["tasks", "done", original]);
    let updated = std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap();
    let lines: Vec<_> = updated.lines().collect();
    assert_eq!(lines.len(), 3, "{updated}");
    assert!(
        lines[0].starts_with("- [ ] Water 🔁 every day 📅 2026-11-17"),
        "{updated}"
    );
    for id in ["^", "🆔", "⛔"] {
        assert!(!lines[0].contains(id), "{updated}");
    }
    assert!(
        lines[1].starts_with("- [x] Water 🆔 water1 ⛔ seed1,seed2 "),
        "{updated}"
    );
    assert!(lines[1].ends_with(" ^water-7c4"), "{updated}");
    assert_eq!(lines[2], "- [ ] Harvest ⛔ water1");
}

#[test]
fn a_recurring_dry_run_shows_the_next_occurrence_without_writing() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    let original = "Water 🔁 every day 📅 2026-11-16 ^water-7c4";
    let content = format!("- [ ] {original}\n");
    std::fs::write(dir.path().join("Tasks.md"), &content).unwrap();
    let preview = change_status(
        dir.path(),
        &["tasks", "done", "--file", "Tasks.md", original, "--dry-run"],
    );
    assert!(
        preview.contains("next: Water 🔁 every day 📅 2026-11-17"),
        "{preview}"
    );
    assert!(!preview.contains("water-7c4-"), "{preview}");
    assert_eq!(
        std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap(),
        content
    );
}
