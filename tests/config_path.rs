//! A config passed with `-c` places the vault in the folder the file is in.

use std::fs;
use std::process::Command;

use tempfile::tempdir;

#[test]
fn a_config_named_without_a_folder_puts_the_vault_where_it_is() {
    let root = tempdir().unwrap();
    fs::write(root.path().join("my.yaml"), "exclude: []\n").unwrap();
    fs::write(root.path().join("Note.md"), "- [ ] A task\n[[Missing]]\n").unwrap();

    let run = |args: &[&str]| {
        let output = Command::new(env!("CARGO_BIN_EXE_pkm"))
            .args(args)
            .current_dir(root.path())
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap()
    };

    for config in ["my.yaml", "./my.yaml"] {
        let tasks = run(&["-c", config, "tasks"]);
        assert!(tasks.contains("A task"), "{config}: {tasks}");

        let lint: serde_json::Value = serde_json::from_str(&run(&[
            "-c",
            config,
            "lint",
            "--check",
            "broken-links",
            "--format",
            "json",
        ]))
        .unwrap();
        assert_eq!(lint["summary"]["broken_links"], 1, "{config}");
    }
}
