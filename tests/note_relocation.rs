use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
fn vault(files: &[(&str, &str)]) -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("pkm.yaml"), "vault_path: .").unwrap();
    for (name, body) in files {
        let path = root.path().join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    }
    root
}
fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pkm"))
        .args(args)
        .env("PKM_NO_UPDATE_CHECK", "1")
        .current_dir(root)
        .output()
        .unwrap()
}
#[test]
fn moving_and_renaming_preserves_incoming_outgoing_and_competing_targets() {
    let incoming = "![[A/Note#^id|label]] [note](../A/Note.md) [[B/Note]]";
    let root = vault(&[
        ("A/Note.md", "[data](./Data.md) [[#Summary]]"),
        ("B/Note.md", "other"),
        ("A/Data.md", "data"),
        ("Diary/Today.md", incoming),
        ("B/Index.md", "[[Note]]"),
        ("C/Index.md", "[[New]]"),
        ("D/New.md", "existing"),
    ]);
    let p = root.path();
    let command = ["move", "A/Note.md", "C/New.md"];
    let preview = run(
        p,
        &[&command[..], &["--dry-run", "--format", "json"]].concat(),
    );
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    let plan: serde_json::Value = serde_json::from_slice(&preview.stdout).unwrap();
    assert!(plan["changes"]
        .as_array()
        .unwrap()
        .iter()
        .any(|c| c["resolved"] == "A/Data.md" && c["line"] == 1));
    assert_eq!(
        fs::read_to_string(p.join("Diary/Today.md")).unwrap(),
        incoming
    );
    let result = run(p, &command);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for (path, body) in [
        ("C/New.md", "[data](../A/Data.md) [[#Summary]]"),
        ("B/Note.md", "other"),
        (
            "Diary/Today.md",
            "![[C/New#^id|label]] [note](../C/New.md) [[B/Note]]",
        ),
        ("B/Index.md", "[[Note]]"),
        ("C/Index.md", "[[D/New|New]]"),
    ] {
        assert_eq!(fs::read_to_string(p.join(path)).unwrap(), body);
    }
}
#[test]
fn ambiguous_source_or_affected_reference_stops_before_the_move() {
    let root = vault(&[
        ("A/Note.md", "a"),
        ("B/Note.md", "b"),
        ("Index.md", "heading\n[[Note]]"),
    ]);
    let p = root.path();
    for args in [
        ["rename", "Note", "Next"],
        ["rename", "A/Note.md", "Next"],
        ["move", "Missing/Note.md", "New/"],
    ] {
        let output = run(p, &args);
        assert!(!output.status.success());
        let diagnostic = String::from_utf8_lossy(&output.stderr);
        if args[1] == "A/Note.md" {
            assert!(
                diagnostic.contains("Index.md:2")
                    && diagnostic.contains("A/Note.md")
                    && diagnostic.contains("B/Note.md")
            );
        }
        assert_eq!(fs::read_to_string(p.join("A/Note.md")).unwrap(), "a");
        assert_eq!(fs::read_to_string(p.join("B/Note.md")).unwrap(), "b");
    }
}

#[test]
fn lint_backlinks_and_rename_agree_on_a_nonexistent_folder() {
    let root = vault(&[
        ("Notes/Thesis.md", "# Thesis"),
        ("Short.md", "[[Thesis]]"),
        ("Qualified.md", "[[Notes/Thesis]]"),
        ("Wrong.md", "[[Wrong/Thesis.md]]"),
    ]);
    let json = |args: &[&str], code| {
        let output = run(root.path(), &[args, &["--format", "json"]].concat());
        assert_eq!(output.status.code(), Some(code));
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()
    };
    let lint = json(&["lint", "--check", "broken-links"], 1);
    assert_eq!(lint["summary"]["broken_links"], 1);
    assert_eq!(lint["issues"][0]["file"], "Wrong.md");
    let backlinks = json(&["backlinks", "Notes/Thesis.md"], 0);
    let mut sources: Vec<_> = backlinks
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b["source"].as_str().unwrap())
        .collect();
    sources.sort();
    assert_eq!(sources, ["Qualified.md", "Short.md"]);
    let plan = json(&["rename", "Notes/Thesis.md", "Paper", "--dry-run"], 0);
    assert_eq!(
        plan["files_to_update"],
        serde_json::json!(["Qualified.md", "Short.md"])
    );
}
