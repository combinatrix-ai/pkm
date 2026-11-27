use std::{fs, process::Command};

#[test]
fn tags_select_task_lines_or_whole_notes() {
    let vault = tempfile::tempdir().unwrap();
    let root = vault.path();
    fs::write(root.join("pkm.yaml"), "{}").unwrap();
    fs::write(root.join("Alpha.md"), "---\ntags: [project]\n---\n- [ ] ship #work #now\n- [ ] read #work #someday\n- [ ] later #work #to-read\n").unwrap();
    fs::write(root.join("Beta.md"), "#project #work/client\n").unwrap();
    let json = |args: &[&str]| {
        let output = Command::new(env!("CARGO_BIN_EXE_pkm"))
            .args(args)
            .args(["--format", "json"])
            .env("PKM_NO_UPDATE_CHECK", "1")
            .current_dir(root)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap()
    };
    let tasks = json(&[
        "tasks",
        "--tag",
        "work",
        "--exclude-tag",
        "someday",
        "--exclude-tag",
        "to-read",
    ]);
    assert_eq!(tasks.len(), 1);
    assert!(tasks[0]["text"].as_str().unwrap().contains("ship"));
    assert!(json(&["tasks", "--tag", "work", "--tag", "project"]).is_empty());
    let notes = json(&[
        "query",
        "--tag",
        "project",
        "--exclude-tag",
        "someday",
        "--limit",
        "1",
    ]);
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0]["path"], "Beta.md");
    let notes = json(&[
        "query",
        "--tag",
        "project",
        "--tag",
        "work",
        "--exclude-tag",
        "Someday",
    ]);
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0]["path"], "Alpha.md");
}
