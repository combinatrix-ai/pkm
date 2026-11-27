//! Notes are UTF-8: a byte order mark is tolerated, anything else is reported.

use std::fs;
use std::process::{Command, Output};

use tempfile::{tempdir, TempDir};

const BOM: &[u8] = b"\xEF\xBB\xBF";
const SHIFT_JIS: &[u8] = b"# \x83\x81\x83\x82\n\n- [ ] \x94\x83\x82\xA4\n";

struct Vault {
    root: TempDir,
}

impl Vault {
    fn new(config: &str, files: &[(&str, &[u8])]) -> Self {
        let root = tempdir().unwrap();
        fs::write(
            root.path().join("pkm.yaml"),
            format!("vault_path: .\n{config}"),
        )
        .unwrap();
        for (relative, content) in files {
            let path = root.path().join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, content).unwrap();
        }
        Self { root }
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_pkm"))
            .args(args)
            .current_dir(self.root.path())
            .output()
            .unwrap()
    }

    fn ok(&self, args: &[&str]) -> String {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    }

    fn json(&self, args: &[&str]) -> serde_json::Value {
        serde_json::from_slice(&self.run(args).stdout).unwrap()
    }

    fn bytes(&self, relative: &str) -> Vec<u8> {
        fs::read(self.root.path().join(relative)).unwrap()
    }
}

fn with_bom(text: &str) -> Vec<u8> {
    [BOM, text.as_bytes()].concat()
}

fn issues_of(report: &serde_json::Value, kind: &str) -> Vec<serde_json::Value> {
    report["issues"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|issue| issue["type"] == kind)
        .cloned()
        .collect()
}

#[test]
fn a_note_with_a_byte_order_mark_reads_like_one_without() {
    let text = "---\ntype: project\ntags: [alpha]\n---\n# Title\n\n- [ ] Buy milk\n";
    let vault = Vault::new(
        "",
        &[
            ("Plain.md", text.as_bytes()),
            ("Marked.md", &with_bom(text)),
            (
                "Heading.md",
                &with_bom("# First line\n\nSome body text here.\n"),
            ),
            ("Linker.md", b"[[Heading#First line]]\n"),
        ],
    );

    let found = vault.ok(&["query", "--where", "type=project"]);
    assert!(
        found.contains("Plain.md") && found.contains("Marked.md"),
        "{found}"
    );

    let tags = vault.ok(&["tags"]);
    assert!(tags.contains("alpha (2)"), "{tags}");

    let report = vault.json(&["lint", "--format", "json"]);
    assert_eq!(report["summary"]["unreadable"], 0);
    assert_eq!(report["summary"]["broken_links"], 0);
    let without_frontmatter: Vec<_> = issues_of(&report, "frontmatter")
        .iter()
        .map(|issue| issue["file"].clone())
        .collect();
    assert_eq!(without_frontmatter, ["Heading.md", "Linker.md"]);

    let context = vault.json(&["context", "Marked", "--format", "json"]);
    assert_eq!(context["content"], "# Title\n\n- [ ] Buy milk\n");
}

#[test]
fn an_edit_keeps_the_byte_order_mark_a_note_had_and_adds_none() {
    let vault = Vault::new(
        "",
        &[
            ("Marked.md", &with_bom("- [ ] Buy milk\n[[Old]]\n")),
            ("Plain.md", b"- [ ] Call home\n[[Old]]\n"),
            ("Old.md", &with_bom("# Old\n")),
            ("Dir/Inside.md", &with_bom("[[Old]]\n")),
        ],
    );

    vault.ok(&["tasks", "done", "Buy milk"]);
    vault.ok(&["tasks", "done", "Call home"]);
    vault.ok(&["rename", "Old", "New"]);
    vault.ok(&["move", "Dir", "Moved"]);

    let marked = vault.bytes("Marked.md");
    assert!(marked.starts_with(BOM) && !marked[3..].starts_with(BOM));
    let marked = String::from_utf8(marked).unwrap();
    assert!(
        marked.contains("- [x] Buy milk") && marked.contains("[[New]]"),
        "{marked}"
    );

    let plain = String::from_utf8(vault.bytes("Plain.md")).unwrap();
    assert!(
        plain.starts_with("- [x] Call home") && plain.contains("[[New]]"),
        "{plain}"
    );

    assert_eq!(vault.bytes("New.md"), with_bom("# Old\n"));
    assert_eq!(vault.bytes("Moved/Dir/Inside.md"), with_bom("[[New]]\n"));
}

#[test]
fn a_note_that_is_not_utf8_is_reported_and_the_rest_is_still_checked() {
    let vault = Vault::new(
        "lint:\n  paths:\n    - path: '.'\n      where: ['type=project']\n      headings:\n        required: [Result]\n",
        &[
            ("Legacy.md", SHIFT_JIS),
            ("Project.md", b"---\ntype: project\n---\n[[Legacy]] [[Missing]]\n"),
        ],
    );

    let report = vault.json(&["lint", "--format", "json"]);
    assert_eq!(
        issues_of(&report, "unreadable"),
        [serde_json::json!({
            "type": "unreadable",
            "file": "Legacy.md",
            "detail": "Not valid UTF-8 (first invalid byte at offset 2)",
            "severity": "error"
        })]
    );
    assert_eq!(report["summary"]["unreadable"], 1);
    let broken: Vec<_> = issues_of(&report, "broken-link")
        .iter()
        .map(|issue| issue["detail"].clone())
        .collect();
    assert_eq!(broken, ["[[Missing]]"]);
    assert_eq!(issues_of(&report, "headings").len(), 1);

    let text = String::from_utf8(vault.run(&["lint"]).stdout).unwrap();
    assert!(
        text.contains("❌ Unreadable notes: 1\n  Legacy.md: Not valid UTF-8"),
        "{text}"
    );
}

#[test]
fn a_selection_reports_only_the_unreadable_notes_in_it() {
    let vault = Vault::new(
        "lint:\n  rules:\n    unreadable:\n      exclude: [Imports]\n",
        &[
            ("Legacy.md", SHIFT_JIS),
            ("Imports/Old.md", SHIFT_JIS),
            ("Fine.md", b"# Fine\n\nLong enough to count.\n"),
        ],
    );

    let selected = |file: &str| {
        let report = vault.json(&["lint", file, "--check", "unreadable", "--format", "json"]);
        report["summary"]["unreadable"].clone()
    };
    assert_eq!(selected("Legacy.md"), 1);
    assert_eq!(selected("Fine.md"), 0);

    let whole = vault.json(&["lint", "--check", "unreadable", "--format", "json"]);
    assert_eq!(whole["summary"]["unreadable"], 1);
    assert_eq!(whole["issues"][0]["file"], "Legacy.md");
}
