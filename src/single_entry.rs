use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use walkdir::WalkDir;

use crate::parser::fold_name;
use crate::vault::{is_excluded, is_note_path, relative_path, Config};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    pub path: String,
    #[serde(default = "default_main")]
    pub main: String,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub entry: BTreeMap<String, serde_yaml::Value>,
}

fn default_main() -> String {
    "README.md".into()
}

fn relative(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('\\')
        && Path::new(value)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}

pub fn parse(value: &serde_yaml::Value) -> Result<Vec<Layout>> {
    let layouts: Vec<Layout> =
        serde_yaml::from_value(value.clone()).context("invalid lint.single_entry")?;
    for layout in &layouts {
        if !relative(&layout.path)
            || !relative(&layout.main)
            || Path::new(&layout.main).components().count() != 1
            || !is_note_path(&layout.main)
            || layout.exclude.iter().any(|s| !relative(s))
        {
            return Err(anyhow!("lint.single_entry requires a relative path, a single note filename for main, and relative exclusions without '..'"));
        }
        for (key, value) in &layout.entry {
            if key.is_empty()
                || !matches!(
                    value,
                    serde_yaml::Value::String(_)
                        | serde_yaml::Value::Bool(_)
                        | serde_yaml::Value::Number(_)
                )
            {
                return Err(anyhow!("lint.single_entry.entry requires nonempty field names and non-null scalar values"));
            }
        }
    }
    Ok(layouts)
}

fn matches_entry(path: &Path, layout: &Layout) -> Result<bool> {
    let text = crate::note::read_note(path)?;
    let note = crate::note::parse_note(path, &text);
    Ok(layout
        .entry
        .iter()
        .all(|(key, value)| note.frontmatter.get(key.as_str()) == Some(value)))
}

pub fn check(
    config: &Config,
    selection: Option<&BTreeSet<String>>,
    in_scope: impl Fn(&str) -> bool,
) -> Result<Vec<Value>> {
    let mut issues = Vec::new();
    for layout in &config.single_entry {
        let root = config.vault_path.join(&layout.path);
        let excluded = |path: &Path| {
            is_excluded(&relative_path(&config.vault_path, path), &config.exclude)
                || is_excluded(&relative_path(&root, path), &layout.exclude)
        };
        if excluded(&root) {
            continue;
        }
        let selected = |path: &Path| {
            let relative = relative_path(&config.vault_path, path);
            selection.is_none_or(|set| {
                set.iter()
                    .any(|s| s == &relative || s.starts_with(&format!("{relative}/")))
            })
        };
        let mut report = |path: &Path, code: &str, detail: String| {
            let file = relative_path(&config.vault_path, path);
            if in_scope(&file) {
                issues.push(json!({"type": "single-entry", "file": file, "code": code, "detail": detail, "severity": "warning"}));
            }
        };
        let mut ancestor = config.vault_path.clone();
        let mut unsafe_root = false;
        for part in Path::new(&layout.path).components() {
            ancestor.push(part);
            if std::fs::symlink_metadata(&ancestor).is_ok_and(|m| m.file_type().is_symlink()) {
                unsafe_root = true;
                break;
            }
        }
        if unsafe_root || !root.is_dir() {
            if selected(&root) {
                report(
                    &root,
                    "invalid-root",
                    "Layout root must be an existing directory without symlink ancestors".into(),
                );
            }
            continue;
        }
        let mut entries = std::fs::read_dir(&root)?.collect::<std::io::Result<Vec<_>>>()?;
        entries
            .retain(|e| !excluded(&e.path()) && !e.file_name().to_string_lossy().starts_with('.'));
        entries.sort_by_key(|e| e.file_name());
        let dirs: Vec<_> = entries
            .iter()
            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
            .collect();
        for item in &entries {
            let path = item.path();
            let kind = item.file_type()?;
            if kind.is_symlink() {
                if selected(&path) {
                    report(
                        &path,
                        "symlink",
                        "Layout entries must not be symlinks".into(),
                    );
                }
                continue;
            }
            if kind.is_file() && is_note_path(&path) {
                if item.file_name() == layout.main.as_str() {
                    continue;
                }
                if selected(&path) {
                    if !matches_entry(&path, layout)? {
                        report(
                            &path,
                            "entry-metadata",
                            "Standalone entry does not match configured entry fields".into(),
                        );
                    }
                    let stem = path.file_stem().unwrap().to_string_lossy();
                    if let Some(dir) = dirs
                        .iter()
                        .find(|d| fold_name(&d.file_name().to_string_lossy()) == fold_name(&stem))
                    {
                        report(
                            &path,
                            "duplicate-unit",
                            format!(
                                "Both standalone note and folder exist; keep one entry at {}/{}",
                                relative_path(&config.vault_path, &dir.path()),
                                layout.main
                            ),
                        );
                    }
                }
            } else if kind.is_dir() && selected(&path) {
                let main = path.join(&layout.main);
                if !excluded(&main)
                    && std::fs::symlink_metadata(&main)
                        .is_ok_and(|m| m.is_file() && !m.file_type().is_symlink())
                {
                    if !matches_entry(&main, layout)? {
                        report(
                            &main,
                            "entry-metadata",
                            "Folder entry does not match configured entry fields".into(),
                        );
                    }
                } else {
                    report(
                        &main,
                        "missing-main",
                        format!("Folder requires {} as its entry note", layout.main),
                    );
                }
                if layout.entry.is_empty() {
                    continue;
                }
                for member in WalkDir::new(&path).into_iter().filter_entry(|e| {
                    !excluded(e.path()) && !e.file_name().to_string_lossy().starts_with('.')
                }) {
                    let member = member?;
                    if member.file_type().is_file()
                        && member.path() != main
                        && is_note_path(member.path())
                        && matches_entry(member.path(), layout)?
                    {
                        report(
                            member.path(),
                            "extra-entry",
                            format!(
                                "Supporting note matches entry fields; keep the entry at {}",
                                relative_path(&config.vault_path, &main)
                            ),
                        );
                    }
                }
            }
        }
    }
    issues.sort_by(|a, b| {
        (a["file"].as_str(), a["code"].as_str()).cmp(&(b["file"].as_str(), b["code"].as_str()))
    });
    issues.dedup();
    Ok(issues)
}
