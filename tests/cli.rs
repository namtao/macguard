//! End-to-end tests of the command-line interface. Read-only: no fix is ever applied,
//! because without an interactive terminal every confirmation is answered "no".

use std::process::{Command, Output, Stdio};

fn macguard(args: &[&str], config: &std::path::Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_macguard"))
        .args(args)
        .env("MACGUARD_CONFIG_DIR", config)
        .env("NO_COLOR", "1")
        .stdin(Stdio::null())
        .output()
        .expect("run macguard")
}

fn text(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned() + &String::from_utf8_lossy(&o.stderr)
}

fn temp_config(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("macguard-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn check_reports_by_severity_with_exit_code() {
    let cfg = temp_config("check");
    let o = macguard(&["--lang", "en", "check"], &cfg);
    let out = text(&o);
    assert!(matches!(o.status.code(), Some(0 | 1)), "{out}");
    let high = out.find("High priority").expect("high group");
    let med = out.find("Medium priority").expect("medium group");
    let low = out.find("Low priority").expect("low group");
    assert!(high < med && med < low, "groups out of order:\n{out}");
    assert!(out.contains("Score"));
}

#[test]
fn no_command_without_terminal_prints_report() {
    let cfg = temp_config("default");
    let out = text(&macguard(&["--lang", "en"], &cfg));
    assert!(out.contains("Score"), "{out}");
}

#[test]
fn help_in_both_languages() {
    let cfg = temp_config("help");
    assert!(text(&macguard(&["--lang", "en", "help"], &cfg)).contains("Usage"));
    assert!(text(&macguard(&["--lang", "vi", "help"], &cfg)).contains("Cách dùng"));
}

#[test]
fn unknown_item_is_rejected() {
    let cfg = temp_config("unknown");
    assert_eq!(
        macguard(&["explain", "no_such_item"], &cfg).status.code(),
        Some(2)
    );
    assert_eq!(
        macguard(&["ignore", "no_such_item"], &cfg).status.code(),
        Some(2)
    );
}

#[test]
fn explain_shows_command_and_both_risks() {
    let cfg = temp_config("explain");
    let out = text(&macguard(&["--lang", "en", "explain", "firewall"], &cfg));
    for label in [
        "What it does",
        "Command that will run",
        "Risk if you do not fix it",
        "Risk when you fix it",
        "Source",
    ] {
        assert!(out.contains(label), "missing {label}:\n{out}");
    }
    assert!(out.contains("socketfilterfw --setglobalstate on"));
}

#[test]
fn ignore_and_unignore() {
    let cfg = temp_config("ignore");
    assert!(macguard(&["ignore", "wifi", "bluetooth"], &cfg)
        .status
        .success());
    let out = text(&macguard(&["--lang", "en", "check"], &cfg));
    assert!(out.contains("Ignored: "), "{out}");
    assert!(!out
        .lines()
        .any(|l| l.trim_start().starts_with("✓ wifi") || l.trim_start().starts_with("! wifi")));
    assert!(macguard(&["unignore", "wifi", "bluetooth"], &cfg)
        .status
        .success());
    let list = std::fs::read_to_string(cfg.join("ignored")).unwrap_or_default();
    assert!(list.trim().is_empty(), "{list}");
}

#[test]
fn fix_without_terminal_changes_nothing() {
    let cfg = temp_config("fix");
    let o = macguard(&["--lang", "en", "fix"], &cfg);
    assert!(o.status.success(), "{}", text(&o));
    assert!(text(&o).contains("Fixed 0 of"));
}

#[test]
fn list_has_every_item() {
    let cfg = temp_config("list");
    let out = text(&macguard(&["list"], &cfg));
    assert!(
        out.lines().filter(|l| l.starts_with("  ")).count() >= 44,
        "{out}"
    );
}

#[test]
fn menu_refuses_without_terminal() {
    let cfg = temp_config("menu");
    assert_eq!(macguard(&["menu"], &cfg).status.code(), Some(2));
}
