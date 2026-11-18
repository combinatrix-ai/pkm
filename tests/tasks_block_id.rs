use std::path::Path;
use std::process::Command;

fn tasks_json(root: &Path) -> serde_json::Value {
    let output = Command::new(env!("CARGO_BIN_EXE_pkm"))
        .args(["tasks", "--format", "json"])
        .current_dir(root)
        .env("HOME", root)
        .env("PKM_NO_UPDATE_CHECK", "1")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

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
fn markdown_checkbox_block_ids_are_optional_json_fields() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    std::fs::write(
        dir.path().join("Tasks.md"),
        "- [ ] Milk 📅 2026-11-16 ^milk-7c4\n- [ ] Bread\n- [ ] Bad ^bad_id\n- [ ] Middle ^middle trailing\n- [ ] NoSpace^nospace\n- {{[[TODO]]}} Roam ^roam-123\n",
    )
    .unwrap();

    let tasks = tasks_json(dir.path());
    let tasks = tasks.as_array().unwrap();
    assert_eq!(tasks.len(), 6);
    assert_eq!(tasks[0]["block_id"], "milk-7c4");
    assert_eq!(tasks[0]["text"], "Milk 📅 2026-11-16 ^milk-7c4");
    for task in &tasks[1..] {
        assert!(task.get("block_id").is_none(), "{task}");
    }
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

#[test]
fn a_recurring_task_gets_a_new_id_on_each_occurrence() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    let original = "Water 🔁 every day 📅 2026-11-16 ^water-7c4";
    std::fs::write(dir.path().join("Tasks.md"), format!("- [ ] {original}\n")).unwrap();
    change_status(dir.path(), &["tasks", "done", original]);
    let updated = std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap();
    let lines: Vec<_> = updated.lines().collect();
    assert_eq!(lines.len(), 2, "{updated}");
    assert!(lines[0].starts_with("- [ ] Water "), "{updated}");
    assert!(lines[0].ends_with(" ^water-7c4-r2"), "{updated}");
    assert!(lines[1].starts_with("- [x] Water "), "{updated}");
    assert!(lines[1].ends_with(" ^water-7c4"), "{updated}");
    let next = tasks_json(dir.path());
    assert_eq!(next[0]["block_id"], "water-7c4-r2");

    change_status(
        dir.path(),
        &["tasks", "done", next[0]["text"].as_str().unwrap()],
    );
    let following = tasks_json(dir.path());
    assert_eq!(following[0]["block_id"], "water-7c4-r3");
    let updated = std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap();
    assert!(updated.contains("^water-7c4-r2"));
    assert!(updated.contains("^water-7c4-r3"));
}

#[test]
fn recurring_batch_avoids_existing_ids_and_matches_its_dry_run() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    let first = "First 🔁 every day 📅 2026-11-16 ^job";
    let second = "Second 🔁 every day 📅 2026-11-16 ^job-r2";
    let original = format!("- [ ] {first}\n- [ ] {second}\n- [ ] Other ^job-r3\n");
    std::fs::write(dir.path().join("Tasks.md"), &original).unwrap();
    let args = ["tasks", "done", "--file", "Tasks.md", first, second];
    let mut preview_args = args.to_vec();
    preview_args.push("--dry-run");
    let preview = change_status(dir.path(), &preview_args);
    assert_eq!(
        std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap(),
        original
    );
    assert!(preview.contains("^job-r4"), "{preview}");
    assert!(preview.contains("^job-r5"), "{preview}");

    let applied = change_status(dir.path(), &args);
    assert_eq!(preview.replace("[DRY RUN] ", ""), applied);
    let updated = std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap();
    for id in ["job", "job-r2", "job-r3", "job-r4", "job-r5"] {
        assert_eq!(
            updated.matches(&format!(" ^{id}\n")).count(),
            1,
            "{updated}"
        );
    }
}

#[test]
fn recurring_task_avoids_a_standalone_block_id_in_the_same_file() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    let task = "Pay 🔁 every week 📅 2026-11-20 ^pay";
    let original = format!("- [ ] {task}\n\n| A |\n| - |\n^pay-r2\n");
    std::fs::write(dir.path().join("Tasks.md"), &original).unwrap();

    let preview = change_status(
        dir.path(),
        &["tasks", "done", "--file", "Tasks.md", "--dry-run", task],
    );
    assert!(preview.contains("^pay-r3"), "{preview}");
    assert_eq!(
        std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap(),
        original
    );

    change_status(dir.path(), &["tasks", "done", "--file", "Tasks.md", task]);
    let updated = std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap();
    assert_eq!(updated.matches("^pay-r2").count(), 1, "{updated}");
    assert_eq!(updated.matches("^pay-r3").count(), 1, "{updated}");
}

#[test]
fn recurrence_only_assigns_a_new_id_when_the_source_has_one() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    let body = "Water 🔁 every day 📅 2026-11-16";
    std::fs::write(dir.path().join("Tasks.md"), format!("- [ ] {body}\n")).unwrap();
    change_status(dir.path(), &["tasks", "done", body]);
    let next = tasks_json(dir.path());
    assert!(next[0].get("block_id").is_none(), "{next}");

    let with_id = "Stretch 🔁 every day 📅 2026-11-16 ^stretch";
    std::fs::write(dir.path().join("Tasks.md"), format!("- [ ] {with_id}\n")).unwrap();
    change_status(dir.path(), &["tasks", "done", with_id, "--no-recur"]);
    let updated = std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap();
    assert_eq!(updated.lines().count(), 1, "{updated}");
    assert!(updated.ends_with(" ^stretch\n"), "{updated}");
}

#[test]
fn every_status_change_can_select_one_exact_block_id() {
    for action in [
        vec!["done"],
        vec!["cancel"],
        vec!["wip"],
        vec!["set", "done"],
    ] {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
        std::fs::write(
            dir.path().join("Tasks.md"),
            "- [ ] Short ^job-r2\n- [ ] Longer ^job-r20\n",
        )
        .unwrap();
        let mut args = vec!["tasks"];
        args.extend(action);
        args.extend(["--file", "Tasks.md", "--block-id", "job-r2"]);
        change_status(dir.path(), &args);
        let updated = std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap();
        assert!(!updated.starts_with("- [ ] Short "), "{args:?}: {updated}");
        assert!(
            updated.contains("- [ ] Longer ^job-r20\n"),
            "{args:?}: {updated}"
        );
        assert!(updated.lines().next().unwrap().ends_with(" ^job-r2"));
    }
}

#[test]
fn block_id_selector_refuses_ambiguous_or_invalid_requests_without_writing() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    let original = "- [ ] First ^job-r20\n- [ ] Second ^duplicate\n- [ ] Third ^duplicate\n";
    std::fs::write(dir.path().join("Tasks.md"), original).unwrap();
    for args in [
        vec![
            "tasks",
            "cancel",
            "--file",
            "Tasks.md",
            "--block-id",
            "job-r2",
        ],
        vec![
            "tasks",
            "cancel",
            "--file",
            "Tasks.md",
            "--block-id",
            "duplicate",
        ],
        vec!["tasks", "cancel", "--block-id", "job-r20"],
        vec![
            "tasks",
            "cancel",
            "--file",
            "Tasks",
            "--block-id",
            "job-r20",
        ],
        vec![
            "tasks",
            "cancel",
            "--file",
            "Tasks.md",
            "--block-id",
            "^job-r20",
        ],
        vec![
            "tasks",
            "cancel",
            "--file",
            "Tasks.md",
            "--block-id",
            "bad_id",
        ],
        vec![
            "tasks",
            "cancel",
            "--file",
            "Tasks.md",
            "--block-id",
            "job-r20",
            "First ^job-r20",
        ],
        vec![
            "tasks",
            "cancel",
            "--file",
            "Tasks.md",
            "--block-id",
            "job-r20",
            "--partial",
        ],
    ] {
        let result = run_status(dir.path(), &args);
        assert!(!result.status.success(), "{args:?}");
        assert_eq!(
            std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap(),
            original
        );
    }
}

#[test]
fn invalid_task_selector_arguments_exit_as_usage_errors() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    std::fs::write(dir.path().join("Tasks.md"), "- [ ] First ^first\n").unwrap();
    for args in [
        vec!["tasks", "wip"],
        vec!["tasks", "done", "First", "Second"],
        vec!["tasks", "done", "--block-id", "first"],
        vec![
            "tasks",
            "done",
            "--file",
            "Tasks.md",
            "--block-id",
            "^first",
        ],
        vec![
            "tasks",
            "done",
            "--file",
            "Tasks.md",
            "--block-id",
            "first",
            "First",
        ],
    ] {
        let result = run_status(dir.path(), &args);
        assert_eq!(
            result.status.code(),
            Some(2),
            "{args:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

#[test]
fn block_id_dry_run_and_recurring_done_use_the_existing_plan() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    let original = "- [ ] Water 🔁 every day 📅 2026-11-16 ^water\n";
    std::fs::write(dir.path().join("Tasks.md"), original).unwrap();
    let args = ["tasks", "done", "--file", "Tasks.md", "--block-id", "water"];
    let mut preview_args = args.to_vec();
    preview_args.push("--dry-run");
    let preview = change_status(dir.path(), &preview_args);
    assert_eq!(
        std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap(),
        original
    );
    let applied = change_status(dir.path(), &args);
    assert_eq!(preview.replace("[DRY RUN] ", ""), applied);
    let updated = std::fs::read_to_string(dir.path().join("Tasks.md")).unwrap();
    assert!(
        updated.lines().next().unwrap().ends_with(" ^water-r2"),
        "{updated}"
    );
    assert!(
        updated.lines().nth(1).unwrap().ends_with(" ^water"),
        "{updated}"
    );
}
