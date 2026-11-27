//! Names outside ASCII, on a real file system.
//!
//! macOS can store a file name decomposed (NFD: `e` + combining acute) while
//! a link typed in a note is composed (NFC: `é`). Links also match names
//! case-insensitively, as Obsidian does, and that has to hold past ASCII.
//! Each case writes the file under one spelling and links to it under another.

use std::fs;
use std::path::Path;
use std::process::Command;

use unicode_normalization::UnicodeNormalization;

fn nfd(name: &str) -> String {
    name.nfd().collect()
}

fn vault(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("pkm.yaml"), "vault_path: .\n").unwrap();
    for (path, content) in files {
        let path = dir.path().join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }
    dir
}

fn json(root: &Path, args: &[&str]) -> serde_json::Value {
    let output = Command::new(env!("CARGO_BIN_EXE_pkm"))
        .args(args)
        .args(["--format", "json"])
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

/// (file name on disk, link written in the note)
fn spellings() -> Vec<(String, &'static str)> {
    vec![
        // Accented Latin, decomposed on disk.
        (nfd("Café.md"), "Café"),
        (nfd("Müller/Notizen.md"), "Müller/Notizen"),
        // Hangul decomposes into its jamo, so nothing of the NFC bytes survives.
        (nfd("한글 메모.md"), "한글 메모"),
        // Case differs past ASCII.
        ("Über.md".to_string(), "über"),
        ("Заметки.md".to_string(), "заметки"),
        ("Ελληνικά.md".to_string(), "ΕΛΛΗΝΙΚΆ"),
        // Both at once: decomposed on disk, upper case in the link.
        (nfd("café crème.md"), "CAFÉ CRÈME"),
    ]
}

#[test]
fn links_resolve_across_normalization_and_case() {
    for (file, link) in spellings() {
        assert_ne!(file.trim_end_matches(".md"), link);
        let dir = vault(&[
            (file.as_str(), "body\n"),
            ("Index.md", &format!("See [[{link}]].\n")),
        ]);
        let broken = json(dir.path(), &["broken-links"]);
        assert_eq!(broken, serde_json::json!([]), "{file:?} <- [[{link}]]");
        let backlinks = json(dir.path(), &["backlinks", &file]);
        assert_eq!(
            backlinks[0]["source"], "Index.md",
            "{file:?} <- [[{link}]]: {backlinks}"
        );
    }
}

#[test]
fn moving_a_decomposed_file_keeps_resolvable_short_links() {
    for (file, link) in spellings() {
        let dir = vault(&[
            (file.as_str(), "body\n"),
            ("Index.md", &format!("See [[{link}]].\n")),
        ]);
        let plan = json(dir.path(), &["move", &file, "Archive/", "--dry-run"]);
        assert_eq!(
            plan["files_to_update"],
            if link.contains('/') {
                serde_json::json!(["Index.md"])
            } else {
                serde_json::json!([])
            },
            "{file:?} <- [[{link}]]: {plan}"
        );

        json(dir.path(), &["move", &file, "Archive/"]);
        assert_eq!(json(dir.path(), &["broken-links"]), serde_json::json!([]));
        // The rewritten link is composed, however the file name was stored.
        let index = fs::read_to_string(dir.path().join("Index.md")).unwrap();
        assert_eq!(index, index.nfc().collect::<String>(), "{index:?}");
        if link.contains('/') {
            assert!(index.starts_with("See [[Archive/"), "{index:?}");
        } else {
            assert_eq!(index, format!("See [[{link}]].\n"));
        }
    }
}

#[test]
fn a_composed_path_finds_a_decomposed_file() {
    let dir = vault(&[
        (nfd("Café.md").as_str(), "body\n"),
        ("Index.md", "See [[Café]].\n"),
    ]);
    let plan = json(dir.path(), &["move", "Café.md", "Archive/", "--dry-run"]);
    assert_eq!(plan["files_to_update"], serde_json::json!([]), "{plan}");
}
