//! `self-update`, and the version a newer-release notice compares against.
//!
//! How knapper was installed decides how it is updated. A GitHub release
//! binary (built with the `self-update` feature) verifies and installs the
//! next release itself. A `cargo install` copy belongs to cargo, so knapper
//! only checks crates.io and names the cargo command. Any other build has no
//! install channel to follow and only reports the newest GitHub release.

use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context, Result};
use serde_json::Value;

#[cfg(feature = "self-update")]
mod release;

const PACKAGE: &str = env!("CARGO_PKG_NAME");
const RELEASES_URL: &str = "https://github.com/combinatrix-ai/knapper/releases";
const CRATES_INDEX_URL: &str = "https://index.crates.io";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Channel {
    /// A GitHub release binary: updates itself after verifying provenance.
    Release,
    /// Installed by `cargo install` from crates.io: cargo replaces it.
    Cargo,
    /// Anything else, such as a local source build.
    Unmanaged,
}

impl Channel {
    pub(crate) fn current() -> Self {
        let from_cargo = std::env::current_exe()
            .and_then(|exe| exe.canonicalize())
            .is_ok_and(|exe| installed_by_cargo(&exe, PACKAGE));
        if from_cargo {
            Channel::Cargo
        } else if cfg!(feature = "self-update") {
            Channel::Release
        } else {
            Channel::Unmanaged
        }
    }

    pub(crate) fn latest_version(self) -> Result<String> {
        match self {
            Channel::Cargo => latest_crate_version(PACKAGE),
            Channel::Release | Channel::Unmanaged => latest_release_version(),
        }
    }

    pub(crate) fn how_to_update(self, latest: &str) -> String {
        match self {
            Channel::Release => "run `knapper self-update`".to_owned(),
            Channel::Cargo => format!("run `cargo install {PACKAGE} --locked`"),
            Channel::Unmanaged => format!("see {RELEASES_URL}/tag/v{latest}"),
        }
    }

    /// Each channel caches its own answer, so reinstalling through another
    /// channel never reports the other source's version.
    pub(crate) fn cache_name(self) -> &'static str {
        match self {
            Channel::Cargo => "latest-crate-version",
            Channel::Release | Channel::Unmanaged => "latest-version",
        }
    }
}

pub fn run(check_only: bool, yes: bool, force: bool, no_skill: bool) -> Result<()> {
    let channel = Channel::current();
    let latest = channel.latest_version()?;

    #[cfg(feature = "self-update")]
    if channel == Channel::Release {
        return release::run(&latest, check_only, yes, force, no_skill);
    }
    let _ = (yes, force, no_skill);

    let current = env!("CARGO_PKG_VERSION");
    if version_is_newer(&latest, current) {
        println!(
            "knapper {current} -> {latest} is available; {}.",
            channel.how_to_update(&latest)
        );
    } else {
        println!("knapper {current} is up to date.");
    }
    if !check_only {
        match channel {
            Channel::Cargo => println!("This copy was installed by cargo, so cargo updates it."),
            Channel::Unmanaged => {
                println!("This build was not installed from a release and does not update itself.")
            }
            Channel::Release => {}
        }
    }
    Ok(())
}

/// Whether `exe` is a binary cargo installed from crates.io for `package`.
///
/// `cargo install` puts binaries in `<root>/bin` and records each install in
/// `<root>/.crates2.json`. Git and path installs are not crates.io releases,
/// so they are not this channel.
fn installed_by_cargo(exe: &Path, package: &str) -> bool {
    let Some(bin_dir) = exe.parent().filter(|dir| dir.ends_with("bin")) else {
        return false;
    };
    let Some(bin_name) = exe.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let Some(root) = bin_dir.parent() else {
        return false;
    };
    let Ok(record) = std::fs::read_to_string(root.join(".crates2.json")) else {
        return false;
    };
    let Ok(record) = serde_json::from_str::<Value>(&record) else {
        return false;
    };
    let prefix = format!("{package} ");
    record["installs"].as_object().is_some_and(|installs| {
        installs.iter().any(|(id, install)| {
            id.starts_with(&prefix)
                && id.contains("(registry+")
                && install["bins"]
                    .as_array()
                    .is_some_and(|bins| bins.iter().any(|bin| bin.as_str() == Some(bin_name)))
        })
    })
}

fn curl(extra: &[&str], url: &str) -> Result<String> {
    let output = Command::new("curl")
        .args([
            "--fail",
            "--silent",
            "--show-error",
            "--location",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "--tlsv1.2",
            "--max-time",
            "3",
        ])
        .args(extra)
        .arg(url)
        .output()
        .context("failed to run curl")?;
    if !output.status.success() {
        bail!("{url}: {}", String::from_utf8_lossy(&output.stderr).trim());
    }
    String::from_utf8(output.stdout).context("response was not UTF-8")
}

fn latest_release_version() -> Result<String> {
    let url = curl(
        &["--output", "/dev/null", "--write-out", "%{url_effective}"],
        &format!("{RELEASES_URL}/latest"),
    )
    .context("latest release check failed")?;
    let tag = url
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or_default();
    let version = tag.strip_prefix('v').unwrap_or(tag);
    parse_version(version).context("latest release had an invalid version")?;
    Ok(version.to_owned())
}

fn latest_crate_version(package: &str) -> Result<String> {
    let index = curl(&[], &format!("{CRATES_INDEX_URL}/{}", index_path(package)))
        .context("crates.io version check failed")?;
    newest_published(&index).with_context(|| format!("crates.io has no release of {package}"))
}

/// The sparse-index path of a crate, as cargo computes it.
fn index_path(package: &str) -> String {
    let name = package.to_ascii_lowercase();
    match name.len() {
        1 => format!("1/{name}"),
        2 => format!("2/{name}"),
        3 => format!("3/{}/{name}", &name[..1]),
        _ => format!("{}/{}/{name}", &name[..2], &name[2..4]),
    }
}

/// The highest stable, unyanked version in a sparse-index file.
fn newest_published(index: &str) -> Option<String> {
    index
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|entry| entry["yanked"] != Value::Bool(true))
        .filter_map(|entry| entry["vers"].as_str().map(str::to_owned))
        .filter(|version| !version.contains(['-', '+']))
        .filter_map(|version| parse_version(&version).map(|parsed| (parsed, version)))
        .max_by_key(|(parsed, _)| *parsed)
        .map(|(_, version)| version)
}

pub(crate) fn version_is_newer(candidate: &str, current: &str) -> bool {
    parse_version(candidate)
        .zip(parse_version(current))
        .is_some_and(|(candidate, current)| candidate > current)
}

fn parse_version(version: &str) -> Option<(u64, u64, u64)> {
    let core = version.split(['-', '+']).next()?;
    let mut parts = core.split('.').map(str::parse::<u64>);
    let parsed = (
        parts.next()?.ok()?,
        parts.next()?.ok()?,
        parts.next()?.ok()?,
    );
    parts.next().is_none().then_some(parsed)
}

#[cfg(test)]
mod tests {
    use super::{
        index_path, installed_by_cargo, newest_published, parse_version, version_is_newer,
    };

    #[test]
    fn compares_release_versions_numerically() {
        assert!(version_is_newer("0.10.0", "0.9.9"));
        assert!(!version_is_newer("0.1.4", "0.1.4"));
        assert!(!version_is_newer("0.1.3", "0.1.4"));
        assert_eq!(parse_version("1.2.3-beta.1"), Some((1, 2, 3)));
        assert_eq!(parse_version("main"), None);
    }

    #[test]
    fn index_paths_follow_cargo_layout() {
        assert_eq!(index_path("a"), "1/a");
        assert_eq!(index_path("ab"), "2/ab");
        assert_eq!(index_path("abc"), "3/a/abc");
        assert_eq!(index_path("Knapper-CLI"), "kn/ap/knapper-cli");
    }

    #[test]
    fn newest_published_skips_yanked_and_prereleases() {
        let index = [
            r#"{"name":"k","vers":"0.1.10","yanked":false}"#,
            r#"{"name":"k","vers":"0.1.9","yanked":false}"#,
            r#"{"name":"k","vers":"0.2.0","yanked":true}"#,
            r#"{"name":"k","vers":"0.3.0-beta.1","yanked":false}"#,
            "not json",
        ]
        .join("\n");
        assert_eq!(newest_published(&index).as_deref(), Some("0.1.10"));
        assert_eq!(newest_published(""), None);
    }

    #[test]
    fn only_a_crates_io_install_record_makes_a_cargo_copy() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir(root.path().join("bin")).unwrap();
        let exe = root.path().join("bin/knapper");
        let record = |id: &str| {
            std::fs::write(
                root.path().join(".crates2.json"),
                format!(r#"{{"installs":{{"{id}":{{"bins":["knapper"]}}}}}}"#),
            )
            .unwrap();
        };

        assert!(!installed_by_cargo(&exe, "knapper-cli"));

        record("knapper-cli 0.1.2 (registry+https://github.com/rust-lang/crates.io-index)");
        assert!(installed_by_cargo(&exe, "knapper-cli"));
        assert!(!installed_by_cargo(
            &root.path().join("bin/other"),
            "knapper-cli"
        ));
        assert!(!installed_by_cargo(&exe, "knapper"));

        record("knapper-cli 0.1.2 (git+https://github.com/combinatrix-ai/knapper#abc)");
        assert!(!installed_by_cargo(&exe, "knapper-cli"));
    }
}
