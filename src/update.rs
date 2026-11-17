//! `macguard update`: replaces this binary with the latest GitHub release.
//! Uses the system curl and shasum, verifies the SHA-256 checksum published with
//! the release, and swaps the file atomically. It is the only network access
//! macguard ever makes, and only when you run this command.

use crate::data::Lang;
use std::path::{Path, PathBuf};
use std::process::Command;

const REPO: &str = "namtao/macguard";
const ASSET: &str = "macguard-macos-universal.tar.gz";

fn curl(url: &str, out: Option<&Path>) -> Result<String, String> {
    let mut cmd = Command::new("curl");
    cmd.args([
        "-fsSL",
        "--proto",
        "=https",
        "--tlsv1.2",
        "-H",
        "User-Agent: macguard-updater",
    ]);
    if let Some(p) = out {
        cmd.arg("-o").arg(p);
    }
    let o = cmd.arg(url).output().map_err(|e| format!("curl: {e}"))?;
    if !o.status.success() {
        return Err(format!(
            "{url}: {}",
            String::from_utf8_lossy(&o.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&o.stdout).into_owned())
}

/// Latest release tag, e.g. "v1.2.0", from the GitHub API.
fn latest_tag() -> Result<String, String> {
    let json = curl(
        &format!("https://api.github.com/repos/{REPO}/releases/latest"),
        None,
    )?;
    let start = json
        .find("\"tag_name\"")
        .ok_or("no tag_name in the GitHub response")?;
    let rest = &json[start + 10..];
    let open = rest.find('"').ok_or("bad GitHub response")? + 1;
    let close = rest[open..].find('"').ok_or("bad GitHub response")? + open;
    Ok(rest[open..close].to_string())
}

/// Compares dotted versions numerically ("1.10.0" > "1.9.2").
fn newer(candidate: &str, current: &str) -> bool {
    let parse = |v: &str| -> Vec<u64> {
        v.trim_start_matches('v')
            .split('.')
            .map(|p| p.parse().unwrap_or(0))
            .collect()
    };
    parse(candidate) > parse(current)
}

pub fn run(lang: Lang) -> Result<(), String> {
    let t = |en: &'static str, vi: &'static str| lang.t(en, vi);
    let current = env!("CARGO_PKG_VERSION");
    println!(
        "{} {current}",
        t("Installed version:", "Phiên bản đang cài:")
    );
    let tag = latest_tag().map_err(|e| {
        if e.contains("404") {
            t(
                "No release has been published yet.",
                "Chưa có bản phát hành nào.",
            )
            .to_string()
        } else {
            e
        }
    })?;
    println!(
        "{} {}",
        t("Latest release:", "Bản phát hành mới nhất:"),
        tag.trim_start_matches('v')
    );
    if !newer(&tag, current) {
        println!(
            "{}",
            t("macguard is up to date.", "macguard đã là bản mới nhất.")
        );
        return Ok(());
    }

    let exe: PathBuf = std::env::current_exe()
        .and_then(|p| p.canonicalize())
        .map_err(|e| e.to_string())?;
    let dir = exe.parent().ok_or("cannot locate the installed binary")?;
    let tmp = std::env::temp_dir().join(format!("macguard-update-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
    let result = (|| -> Result<(), String> {
        let base = format!("https://github.com/{REPO}/releases/download/{tag}");
        let archive = tmp.join(ASSET);
        println!("{} {base}/{ASSET}", t("Downloading", "Đang tải"));
        curl(&format!("{base}/{ASSET}"), Some(&archive))?;
        let expected = curl(&format!("{base}/{ASSET}.sha256"), None)?;
        let expected = expected
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_lowercase();
        let actual = Command::new("shasum")
            .args(["-a", "256"])
            .arg(&archive)
            .output()
            .map_err(|e| e.to_string())?;
        let actual = String::from_utf8_lossy(&actual.stdout)
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_lowercase();
        if expected.len() != 64 || expected != actual {
            return Err(t(
                "Checksum mismatch; the download was discarded.",
                "Sai checksum; bản tải về đã bị huỷ.",
            )
            .into());
        }
        let ok = Command::new("tar")
            .arg("-xzf")
            .arg(&archive)
            .arg("-C")
            .arg(&tmp)
            .status()
            .map_err(|e| e.to_string())?;
        let new_bin = tmp.join("macguard");
        if !ok.success() || !new_bin.exists() {
            return Err(t(
                "The release archive does not contain macguard.",
                "Gói phát hành không có file macguard.",
            )
            .into());
        }
        // Copy next to the current binary, then rename over it, so the swap is atomic.
        let staged = dir.join(".macguard.new");
        std::fs::copy(&new_bin, &staged).map_err(|e| permission_hint(e, dir, lang))?;
        let _ = Command::new("chmod").arg("755").arg(&staged).status();
        std::fs::rename(&staged, &exe).map_err(|e| permission_hint(e, dir, lang))?;
        Ok(())
    })();
    let _ = std::fs::remove_dir_all(&tmp);
    result?;
    println!(
        "✓ {} {}",
        t("Updated to", "Đã cập nhật lên"),
        tag.trim_start_matches('v')
    );
    Ok(())
}

fn permission_hint(e: std::io::Error, dir: &Path, lang: Lang) -> String {
    if e.kind() == std::io::ErrorKind::PermissionDenied {
        match lang {
            Lang::En => format!(
                "Cannot write to {}. Run 'sudo macguard update'.",
                dir.display()
            ),
            Lang::Vi => format!(
                "Không ghi được vào {}. Hãy chạy 'sudo macguard update'.",
                dir.display()
            ),
        }
    } else {
        e.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::newer;

    #[test]
    fn compares_versions() {
        assert!(newer("v1.10.0", "1.9.2"));
        assert!(newer("1.0.1", "1.0.0"));
        assert!(!newer("v1.0.0", "1.0.0"));
        assert!(!newer("0.9.0", "1.0.0"));
    }
}
