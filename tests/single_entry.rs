//! Single-entry layouts are opt-in and identify entries by position.
use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn vault(config: &str, files: &[(&str, &str)]) -> TempDir {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("pkm.yaml"), config).unwrap();
    for (name, text) in files {
        let path = root.path().join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    root
}
fn run(root: &TempDir, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_pkm"))
        .args(args)
        .env("PKM_NO_UPDATE_CHECK", "1")
        .current_dir(root.path())
        .output()
        .unwrap()
}
fn report(root: &TempDir, extra: &[&str]) -> serde_json::Value {
    let mut args = vec!["lint", "--check", "single-entry", "--format", "json"];
    args.extend_from_slice(extra);
    let output = run(root, &args);
    let value: serde_json::Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&output.stderr)));
    assert_eq!(
        output.status.code(),
        Some(i32::from(
            value["summary"]["total_issues"].as_u64().unwrap() > 0
        ))
    );
    value
}
const CONFIG: &str = "lint:\n  single_entry:\n    - path: Projects\n      exclude: [assets]\n      entry: {type: project}\n";
const ENTRY: &str = "---\ntype: project\n---\n# Project\n";

#[test]
fn accepts_single_notes_and_folders_with_default_main() {
    let v = vault(
        CONFIG,
        &[
            ("Projects/One.md", ENTRY),
            ("Projects/Two/README.md", ENTRY),
            ("Projects/Two/research.md", "# Research"),
            ("Projects/Two/assets/a.md", "# Asset"),
            ("Projects/README.md", "# Index"),
            ("Projects/assets/a.md", "# Asset"),
        ],
    );
    assert_eq!(report(&v, &[])["summary"]["total_issues"], 0);
}

#[test]
fn detects_missing_main_metadata_extra_entries_and_duplicate_units() {
    let v = vault(
        CONFIG,
        &[
            ("Projects/One.md", "# Missing type"),
            ("Projects/Two/material.md", "# Material"),
            ("Projects/Three/README.md", ENTRY),
            ("Projects/Three/nested/extra.md", ENTRY),
            ("Projects/Three.md", ENTRY),
        ],
    );
    let r = report(&v, &[]);
    let codes: Vec<_> = r["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["code"].as_str().unwrap())
        .collect();
    assert_eq!(
        codes,
        [
            "entry-metadata",
            "duplicate-unit",
            "extra-entry",
            "missing-main"
        ]
    );
}

#[test]
fn supports_custom_main_and_custom_entry_fields() {
    let v = vault("lint:\n  single_entry:\n    - path: Areas\n      main: index.org\n      entry: {kind: area}\n", &[("Areas/A/index.org", "#+kind: area\n* Area\n"), ("Areas/A/notes.md", "# Notes")]);
    assert_eq!(report(&v, &[])["summary"]["total_issues"], 0);
}

#[test]
fn reports_empty_folder_and_does_not_follow_symlinks() {
    let v = vault(CONFIG, &[]);
    fs::create_dir_all(v.path().join("Projects/Empty")).unwrap();
    #[cfg(unix)]
    {
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("README.md"), ENTRY).unwrap();
        std::os::unix::fs::symlink(outside.path(), v.path().join("Projects/Linked")).unwrap();
        assert!(report(&v, &[])["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|i| i["code"] == "symlink"));
    }
    assert!(report(&v, &[])["issues"]
        .as_array()
        .unwrap()
        .iter()
        .any(|i| i["code"] == "missing-main"));
}

#[test]
fn selected_material_checks_its_whole_unit_only() {
    let v = vault(
        CONFIG,
        &[
            ("Projects/A/material.md", "# Material"),
            ("Projects/B/material.md", "# Other"),
        ],
    );
    let r = report(&v, &["Projects/A/material.md"]);
    assert_eq!(r["issues"].as_array().unwrap().len(), 1);
    assert_eq!(r["issues"][0]["file"], "Projects/A/README.md");
}

#[test]
fn global_and_layout_exclusions_apply_to_supporting_notes() {
    let v = vault("exclude: [Projects/Hidden]\nlint:\n  single_entry:\n    - path: Projects\n      exclude: [assets, A/private]\n      entry: {type: project}\n", &[("Projects/Hidden/no-main.md", ENTRY), ("Projects/assets/notes.md", ENTRY), ("Projects/A/README.md", ENTRY), ("Projects/A/private/extra.md", ENTRY)]);
    assert_eq!(report(&v, &[])["summary"]["total_issues"], 0);
}

#[test]
fn invalid_configuration_is_rejected_and_schema_exposes_layout() {
    for fields in [
        "path: ../outside",
        "path: Projects\n      main: nested/index.md",
        "path: Projects\n      main: index.txt",
        "path: Projects\n      typo: true",
        "path: Projects\n      entry: {type: null}",
    ] {
        let v = vault(&format!("lint:\n  single_entry:\n    - {fields}\n"), &[]);
        assert!(!run(&v, &["config", "check"]).status.success(), "{fields}");
    }
    let v = vault("", &[]);
    let output = run(&v, &["config", "schema"]);
    let schema: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        schema["properties"]["lint"]["properties"]["single_entry"]["type"],
        "array"
    );
}

#[test]
fn disabled_rule_is_only_run_explicitly_and_unicode_names_collide() {
    let v = vault("lint:\n  rules:\n    single-entry: {enabled: false}\n  single_entry:\n    - path: Projects\n", &[("Projects/Caf\u{e9}.md", "# One"), ("Projects/Cafe\u{301}/README.md", "# Two")]);
    let r = report(&v, &[]);
    assert_eq!(r["issues"][0]["code"], "duplicate-unit");
    let output = run(&v, &["lint", "--format", "json"]);
    let r: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(r["summary"].get("single_entry").is_none());
}

#[test]
fn unreadable_layout_notes_do_not_hide_other_lint_diagnostics() {
    let v = vault(
        CONFIG,
        &[
            ("Projects/Unit.md", ""),
            ("Projects/Unit/README.md", ""),
            ("Projects/Unit/support.md", ""),
            ("Projects/Unit/extra.md", ENTRY),
            ("Index.md", "[[Missing]]"),
        ],
    );
    for path in [
        "Projects/Unit.md",
        "Projects/Unit/README.md",
        "Projects/Unit/support.md",
    ] {
        fs::write(v.path().join(path), [0xff]).unwrap();
    }
    let output = run(&v, &["lint", "--format", "json"]);
    assert_eq!(output.status.code(), Some(1));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["summary"]["unreadable"], 3);
    assert_eq!(value["summary"]["broken_links"], 1);
    let codes: Vec<_> = value["issues"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|issue| issue["type"] == "single-entry")
        .map(|issue| issue["code"].as_str().unwrap())
        .collect();
    assert_eq!(codes, ["duplicate-unit", "extra-entry"]);
}
