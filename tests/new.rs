use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_knapper"))
        .args(args)
        .current_dir(root)
        .env("HOME", root)
        .env("KNAPPER_NO_UPDATE_CHECK", "1")
        .output()
        .unwrap()
}

fn vault(folder: &str, engine: &str) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("knapper.yaml"),
        format!(
            r#"
vault_path: .
template_engine: {engine}
exclude: [Templates]
templates:
  Project:
    file: Templates/Project.md
    folder: '{folder}'
lint:
  rules:
    orphans: {{enabled: false}}
  paths:
    - path: '^Projects/'
      headings:
        required: [Summary, Next steps, References]
      frontmatter:
        required: [type, status, updated]
        fields:
          updated: {{type: date}}
"#
        ),
    )
    .unwrap();
    std::fs::create_dir(dir.path().join("Templates")).unwrap();
    std::fs::write(dir.path().join("Templates/Project.md"), "---\ntype: project\nstatus: active\nupdated: {{date}}\n---\n# {{title}}\n\n## Summary\n\n## Next steps\n\n## References\n").unwrap();
    dir
}

#[test]
fn creates_expanded_note_that_passes_targeted_lint() {
    for engine in ["core", "templater"] {
        let dir = vault("Projects", engine);
        if engine == "templater" {
            let p = dir.path().join("Templates/Project.md");
            let s = std::fs::read_to_string(&p)
                .unwrap()
                .replace("{{date}}", "<% tp.date.now(\"YYYY-MM-DD\") %>")
                .replace("{{title}}", "<% tp.file.title %>");
            std::fs::write(p, s).unwrap();
        }
        let title = "計画 $1 ${name}";
        let out = run(
            dir.path(),
            &["new", title, "--template", "Project", "--format", "json"],
        );
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        let path = value["path"].as_str().unwrap();
        let body = std::fs::read_to_string(dir.path().join(path)).unwrap();
        assert!(body.contains(&format!("# {title}\n")), "{body}");
        assert!(!body.contains("<%") && !body.contains("{{"));
        let lint = run(dir.path(), &["lint", path]);
        assert!(
            lint.status.success(),
            "{}{}",
            String::from_utf8_lossy(&lint.stdout),
            String::from_utf8_lossy(&lint.stderr)
        );
        let again = run(dir.path(), &["new", title, "--template", "Project"]);
        assert!(!again.status.success());
        assert_eq!(
            std::fs::read_to_string(dir.path().join(path)).unwrap(),
            body
        );
    }
}

#[test]
fn rejects_invalid_titles_and_missing_templates_without_creating_folder() {
    let dir = vault("Projects", "core");
    for title in [
        "",
        " ",
        ".",
        "..",
        "../escape",
        "/tmp/escape",
        "a/b",
        "a\\b",
        "line\nbreak",
    ] {
        assert!(!run(dir.path(), &["new", title, "--template", "Project"])
            .status
            .success());
    }
    assert!(
        !run(dir.path(), &["new", "Example", "--template", "Unknown"])
            .status
            .success()
    );
    std::fs::remove_file(dir.path().join("Templates/Project.md")).unwrap();
    assert!(
        !run(dir.path(), &["new", "Example", "--template", "Project"])
            .status
            .success()
    );
    assert!(!dir.path().join("Projects").exists());
}

#[test]
fn output_modes_and_nested_folders_work() {
    for mode in [vec![], vec!["--path-only"]] {
        let dir = vault("Projects/Nested", "core");
        let mut args = vec!["new", "Example", "--template", "Project"];
        args.extend(mode);
        let out = run(dir.path(), &args);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(String::from_utf8_lossy(&out.stdout).contains("Projects/Nested/Example.md"));
        assert!(dir.path().join("Projects/Nested/Example.md").exists());
    }
}

#[test]
fn rejects_escape_paths() {
    let outside = tempfile::tempdir().unwrap();
    for folder in ["../escape", outside.path().to_str().unwrap()] {
        let dir = vault(folder, "core");
        assert!(
            !run(dir.path(), &["new", "Example", "--template", "Project"])
                .status
                .success()
        );
    }
    assert!(!outside.path().join("Example.md").exists());
}

#[cfg(unix)]
#[test]
fn rejects_symlinked_folders_templates_and_existing_targets() {
    use std::os::unix::fs::symlink;
    let outside = tempfile::tempdir().unwrap();
    for target in ["Projects", "Templates/Project.md", "Projects/Example.md"] {
        let dir = vault("Projects", "core");
        if target == "Templates/Project.md" {
            std::fs::remove_file(dir.path().join(target)).unwrap();
            symlink(outside.path().join("source.md"), dir.path().join(target)).unwrap();
        } else if target == "Projects" {
            symlink(outside.path(), dir.path().join(target)).unwrap();
        } else {
            std::fs::create_dir(dir.path().join("Projects")).unwrap();
            symlink(outside.path().join("missing.md"), dir.path().join(target)).unwrap();
        }
        assert!(
            !run(dir.path(), &["new", "Example", "--template", "Project"])
                .status
                .success()
        );
    }
    assert_eq!(std::fs::read_dir(outside.path()).unwrap().count(), 0);
}

#[test]
fn config_check_rejects_incomplete_or_misspelled_templates() {
    for template in [
        "file: Templates/Project.md",
        "file: ''\n    folder: Projects",
        "file: Templates/Project.md\n    folder: Projects\n    typo: value",
    ] {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("knapper.yaml"),
            format!("templates:\n  Project:\n    {template}\n"),
        )
        .unwrap();
        assert!(!run(dir.path(), &["config", "check"]).status.success());
    }
}

#[test]
fn direct_template_needs_no_registration_and_requires_destination() {
    let dir = vault("Projects", "core");
    std::fs::write(
        dir.path().join("knapper.yaml"),
        "vault_path: .\ntemplate_engine: core\n",
    )
    .unwrap();
    let missing = run(
        dir.path(),
        &["new", "Example", "--template", "Templates/Project.md"],
    );
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("--folder"));
    let output = run(
        dir.path(),
        &[
            "new",
            "Example",
            "--template",
            "Templates/Project.md",
            "--folder",
            "Notes",
            "--path-only",
        ],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "Notes/Example.md"
    );
    assert!(std::fs::read_to_string(dir.path().join("Notes/Example.md"))
        .unwrap()
        .contains("# Example"));
}

#[test]
fn destination_override_works_for_registered_templates() {
    let dir = vault("Projects", "core");
    let output = run(
        dir.path(),
        &["new", "Example", "--template", "Project", "--folder", "."],
    );
    assert!(output.status.success());
    assert!(dir.path().join("Example.md").exists());
    assert!(!dir.path().join("Projects").exists());
}

#[test]
fn direct_template_and_destination_cannot_escape_vault() {
    let outside = tempfile::tempdir().unwrap();
    let dir = vault("Projects", "core");
    for (template, folder) in [
        ("../outside.md", "Notes"),
        (outside.path().to_str().unwrap(), "Notes"),
        ("Templates/Project.md", "../outside"),
        ("Templates/Project.md", outside.path().to_str().unwrap()),
    ] {
        assert!(!run(
            dir.path(),
            &["new", "Example", "--template", template, "--folder", folder]
        )
        .status
        .success());
    }
    assert!(!dir.path().join("Notes").exists());
    assert_eq!(std::fs::read_dir(outside.path()).unwrap().count(), 0);
}
