//! The check logic for every item in data/checks.toml, and parallel scanning.

use crate::data::{Def, Lang};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Pass,
    Fail,
    /// Not applicable here; shown only when it has a detail (e.g. "needs sudo").
    Skip,
    Ignored,
}

#[derive(Clone, Debug)]
pub struct Outcome {
    pub status: Status,
    pub detail: String,
}

impl Outcome {
    fn new(status: Status, detail: impl Into<String>) -> Self {
        Outcome {
            status,
            detail: detail.into(),
        }
    }
    fn pass() -> Self {
        Self::new(Status::Pass, "")
    }
    fn fail() -> Self {
        Self::new(Status::Fail, "")
    }
    fn skip() -> Self {
        Self::new(Status::Skip, "")
    }
    fn when(ok: bool) -> Self {
        if ok {
            Self::pass()
        } else {
            Self::fail()
        }
    }
    /// Hidden from lists: not applicable and nothing to say.
    pub fn hidden(&self) -> bool {
        self.status == Status::Skip && self.detail.is_empty()
    }
}

/// Who is being audited. Under sudo, per-user settings belong to SUDO_USER.
#[derive(Clone)]
pub struct Ctx {
    pub lang: Lang,
    pub root: bool,
    pub user: String,
    pub home: PathBuf,
}

impl Ctx {
    pub fn detect(lang: Lang) -> Ctx {
        let root = run("id", &["-u"]).trim() == "0";
        let user = match std::env::var("SUDO_USER") {
            Ok(u) if root && !u.is_empty() => u,
            _ => run("id", &["-un"]).trim().to_string(),
        };
        let home = run(
            "dscl",
            &[".", "-read", &format!("/Users/{user}"), "NFSHomeDirectory"],
        )
        .split_whitespace()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(std::env::var("HOME").unwrap_or_default()));
        Ctx {
            lang,
            root,
            user,
            home,
        }
    }

    fn t(&self, en: &str, vi: &str) -> String {
        self.lang.t(en, vi).to_string()
    }

    /// Runs a command as the audited user (via sudo -u when running as root).
    pub(crate) fn as_user_status(&self, prog: &str, args: &[&str]) -> (bool, String) {
        if self.root && self.user != "root" {
            let mut all = vec!["-H", "-u", self.user.as_str(), prog];
            all.extend_from_slice(args);
            run_status("sudo", &all)
        } else {
            run_status(prog, args)
        }
    }

    fn as_user(&self, prog: &str, args: &[&str]) -> String {
        self.as_user_status(prog, args).1
    }

    /// `defaults read` of a per-user setting; None when the key is not set.
    fn user_default(&self, args: &[&str]) -> Option<String> {
        let mut all = vec!["read"];
        all.extend_from_slice(args);
        let (ok, out) = self.as_user_status("defaults", &all);
        ok.then(|| out.trim().to_string())
    }
}

/// Runs a command and returns stdout followed by stderr (macOS tools mix both).
pub fn run(prog: &str, args: &[&str]) -> String {
    run_status(prog, args).1
}

fn run_status(prog: &str, args: &[&str]) -> (bool, String) {
    match Command::new(prog).args(args).output() {
        Ok(o) => {
            let mut s = String::from_utf8_lossy(&o.stdout).into_owned();
            s.push_str(&String::from_utf8_lossy(&o.stderr));
            (o.status.success(), s)
        }
        Err(_) => (false, String::new()),
    }
}

fn default(domain: &str, key: &str) -> Option<String> {
    let (ok, out) = run_status("defaults", &["read", domain, key]);
    ok.then(|| out.trim().to_string())
}

fn is_true(v: Option<&str>) -> bool {
    matches!(v, Some("1" | "YES" | "yes" | "TRUE" | "true"))
}

fn exists(p: &str) -> bool {
    Path::new(p).exists()
}

fn join(items: &[String]) -> String {
    items.join(", ")
}

/// Value after "key : " or "key: " on the first matching line.
fn field(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|l| {
        let l = l.trim();
        l.strip_prefix(key).and_then(|rest| {
            let rest = rest.trim_start();
            rest.strip_prefix(':').map(|v| v.trim().to_string())
        })
    })
}

/// Days since 1970-01-01 for a civil date (Howard Hinnant's algorithm).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn listen_sockets() -> Vec<(String, String)> {
    run("lsof", &["-nP", "-iTCP", "-sTCP:LISTEN"])
        .lines()
        .skip(1)
        .filter_map(|l| {
            let f: Vec<&str> = l.split_whitespace().collect();
            (f.len() >= 9).then(|| (f[0].to_string(), f[8].to_string()))
        })
        .collect()
}

fn brew_installed() -> bool {
    exists("/opt/homebrew/bin/brew") || exists("/usr/local/bin/brew")
}

fn brew_path() -> &'static str {
    if exists("/opt/homebrew/bin/brew") {
        "/opt/homebrew/bin/brew"
    } else {
        "/usr/local/bin/brew"
    }
}

/// VAR=1 exported now or set in a shell startup file of the audited user.
fn env_set(ctx: &Ctx, var: &str) -> bool {
    if std::env::var(var).as_deref() == Ok("1") {
        return true;
    }
    [
        ".zprofile",
        ".zshrc",
        ".zshenv",
        ".bash_profile",
        ".bashrc",
        ".profile",
    ]
    .iter()
    .any(|f| {
        std::fs::read_to_string(ctx.home.join(f)).is_ok_and(|s| {
            s.lines().any(|l| {
                let l = l.trim().trim_start_matches("export").trim();
                l == format!("{var}=1")
            })
        })
    })
}

const FW: &str = "/usr/libexec/ApplicationFirewall/socketfilterfw";
const SWU: &str = "/Library/Preferences/com.apple.SoftwareUpdate";
const LOGIN: &str = "/Library/Preferences/com.apple.loginwindow";
const DIAG: &str = "/Library/Application Support/CrashReporter/DiagnosticMessagesHistory.plist";

/// Every id with check logic; tests verify it matches data/checks.toml.
#[cfg(test)]
pub const IDS: &[&str] = &[
    "filevault",
    "sip",
    "gatekeeper",
    "firmware",
    "updates",
    "pending_updates",
    "xprotect",
    "backup",
    "lockdown",
    "autologin",
    "screenlock",
    "guest",
    "hint",
    "admin_user",
    "hostname",
    "sudo_home",
    "firewall",
    "stealth",
    "signed",
    "outbound_fw",
    "dns",
    "hosts_blocklist",
    "certs",
    "wifi",
    "bonjour",
    "handoff",
    "bluetooth",
    "ssh",
    "ssh_password",
    "screensharing",
    "airplay",
    "listeners",
    "launch_items",
    "sysext",
    "app_runtime",
    "brew_analytics",
    "brew_redirect",
    "diagnostics",
    "crash_dialog",
    "icloud_save",
    "extensions",
    "hidden_files",
    "umask",
    "terminal_keyboard",
    "file_sharing",
    "printer_sharing",
    "internet_sharing",
    "remote_management",
    "remote_apple_events",
    "media_sharing",
    "sudo_password",
    "ssh_key_passphrase",
    "ssh_key_strength",
    "ssh_host_keys",
    "ssh_agent_forwarding",
];

fn ssh_on() -> bool {
    run_status("launchctl", &["print", "system/com.openssh.sshd"]).0
}

/// True when something accepts TCP connections on this port of this Mac. Unlike
/// lsof, this also sees services that launchd listens for, without root.
fn port_open(port: u16) -> bool {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(300)).is_ok()
}

fn port_check(port: u16) -> Outcome {
    if port_open(port) {
        Outcome::new(Status::Fail, format!("port {port}"))
    } else {
        Outcome::pass()
    }
}

/// Private key files in ~/.ssh (files that start with a PRIVATE KEY header).
fn ssh_private_keys(ctx: &Ctx) -> Vec<PathBuf> {
    let mut keys: Vec<PathBuf> = std::fs::read_dir(ctx.home.join(".ssh"))
        .map(|rd| {
            rd.flatten()
                .map(|e| e.path())
                .filter(|p| p.is_file())
                .collect()
        })
        .unwrap_or_default();
    keys.retain(|p| {
        use std::io::Read;
        let mut head = [0u8; 64];
        let n = std::fs::File::open(p)
            .and_then(|mut f| f.read(&mut head))
            .unwrap_or(0);
        let head = String::from_utf8_lossy(&head[..n]);
        head.starts_with("-----BEGIN") && head.contains("PRIVATE KEY")
    });
    keys.sort();
    keys
}

fn file_name(p: &Path) -> String {
    p.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

/// Lower-case (keyword, value) pairs from the user's and the system SSH client config.
fn ssh_client_options(ctx: &Ctx) -> Vec<(String, String)> {
    [
        ctx.home.join(".ssh/config"),
        PathBuf::from("/etc/ssh/ssh_config"),
    ]
    .iter()
    .filter_map(|p| std::fs::read_to_string(p).ok())
    .flat_map(|text| {
        text.lines()
            .filter_map(|l| {
                let l = l.trim();
                if l.starts_with('#') {
                    return None;
                }
                let mut f = l.splitn(2, |c: char| c.is_whitespace() || c == '=');
                let k = f.next()?.to_lowercase();
                let v = f
                    .next()?
                    .trim()
                    .trim_start_matches('=')
                    .trim()
                    .to_lowercase();
                Some((k, v))
            })
            .collect::<Vec<_>>()
    })
    .collect()
}

/// Runs the check for one item.
pub fn check(id: &str, ctx: &Ctx) -> Outcome {
    match id {
        "filevault" => Outcome::when(run("fdesetup", &["status"]).contains("FileVault is On")),
        "sip" => Outcome::when(run("csrutil", &["status"]).contains("status: enabled")),
        "gatekeeper" => Outcome::when(run("spctl", &["--status"]).contains("assessments enabled")),
        "firmware" => {
            if !exists("/usr/bin/bputil") {
                return Outcome::skip();
            }
            if !ctx.root {
                return Outcome::new(Status::Skip, ctx.t("needs sudo", "cần sudo"));
            }
            let out = run("bputil", &["-d"]).to_lowercase();
            if out.contains("full security") {
                Outcome::pass()
            } else if out.contains("reduced security") {
                Outcome::new(Status::Fail, "Reduced Security")
            } else if out.contains("permissive security") {
                Outcome::new(Status::Fail, "Permissive Security")
            } else {
                Outcome::new(
                    Status::Skip,
                    ctx.t(
                        "could not read the security mode",
                        "không đọc được chế độ bảo mật",
                    ),
                )
            }
        }
        "updates" => {
            let keys = [
                "AutomaticCheckEnabled",
                "AutomaticDownload",
                "CriticalUpdateInstall",
                "ConfigDataInstall",
                "AutomaticallyInstallMacOSUpdates",
            ];
            // A missing key means the Apple default: on, except installing macOS updates.
            let off: Vec<String> = keys
                .iter()
                .filter(|k| match default(SWU, k).as_deref() {
                    Some("1") => false,
                    Some("0") => true,
                    _ => **k == "AutomaticallyInstallMacOSUpdates",
                })
                .map(|k| k.to_string())
                .collect();
            if off.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(
                    Status::Fail,
                    format!("{}: {}", ctx.t("off", "đang tắt"), off.join(" ")),
                )
            }
        }
        "pending_updates" => {
            let out = run("softwareupdate", &["--list", "--no-scan"]);
            if out.contains("No new software available") {
                return Outcome::pass();
            }
            let labels: Vec<String> = out
                .lines()
                .filter_map(|l| l.split_once("Label: ").map(|(_, v)| v.trim().to_string()))
                .collect();
            if labels.is_empty() {
                Outcome::new(
                    Status::Skip,
                    ctx.t("no scan result yet", "chưa có kết quả quét"),
                )
            } else {
                Outcome::new(Status::Fail, join(&labels))
            }
        }
        "xprotect" => {
            if !exists("/usr/bin/xprotect") {
                return Outcome::skip();
            }
            let out = run("xprotect", &["version"]);
            let Some(date) = out
                .split("Installed: ")
                .nth(1)
                .map(|s| s.chars().take(10).collect::<String>())
            else {
                return Outcome::new(
                    Status::Skip,
                    ctx.t("version unknown", "không đọc được phiên bản"),
                );
            };
            let p: Vec<i64> = date.split('-').filter_map(|x| x.parse().ok()).collect();
            if p.len() != 3 {
                return Outcome::new(
                    Status::Skip,
                    ctx.t("version unknown", "không đọc được phiên bản"),
                );
            }
            let today = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64 / 86400)
                .unwrap_or(0);
            let age = today - days_from_civil(p[0], p[1], p[2]);
            Outcome::new(
                if age <= 30 {
                    Status::Pass
                } else {
                    Status::Fail
                },
                format!("{} {date}", ctx.t("updated", "cập nhật")),
            )
        }
        "backup" => {
            let out = run("tmutil", &["destinationinfo"]);
            if out.contains("No destinations") {
                return Outcome::new(Status::Fail, ctx.t("no backup disk", "chưa có ổ sao lưu"));
            }
            let Some(mp) = field(&out, "Mount Point") else {
                return Outcome::new(
                    Status::Pass,
                    ctx.t(
                        "disk not connected, encryption not checked",
                        "ổ chưa cắm, chưa kiểm mã hoá",
                    ),
                );
            };
            let info = run("diskutil", &["info", &mp]);
            if field(&info, "FileVault").as_deref() == Some("Yes") {
                Outcome::pass()
            } else if field(&out, "Kind").as_deref() == Some("Network") {
                Outcome::new(
                    Status::Pass,
                    ctx.t(
                        "network disk, encryption not checked",
                        "ổ mạng, chưa kiểm mã hoá",
                    ),
                )
            } else {
                Outcome::new(
                    Status::Fail,
                    ctx.t("backup disk is not encrypted", "ổ sao lưu chưa mã hoá"),
                )
            }
        }
        "lockdown" => {
            Outcome::when(run("sysctl", &["-n", "security.mac.lockdown_mode_state"]).trim() == "1")
        }

        "autologin" => Outcome::when(default(LOGIN, "autoLoginUser").is_none()),
        "screenlock" => {
            let out = ctx.as_user("sysadminctl", &["-screenLock", "status"]);
            if out.contains("immediate") {
                Outcome::pass()
            } else if let Some(rest) = out.split("delay is ").nth(1) {
                let n: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
                if n == "0" {
                    Outcome::pass()
                } else {
                    Outcome::new(
                        Status::Fail,
                        ctx.lang
                            .t(&format!("delay {n} s"), &format!("chờ {n} giây"))
                            .to_string(),
                    )
                }
            } else {
                Outcome::new(Status::Fail, ctx.t("off", "đang tắt"))
            }
        }
        "guest" => Outcome::when(!is_true(default(LOGIN, "GuestEnabled").as_deref())),
        "hint" => Outcome::when(matches!(
            default(LOGIN, "RetriesUntilHint").as_deref(),
            None | Some("0")
        )),
        "admin_user" => Outcome::when(
            !run(
                "dsmemberutil",
                &["checkmembership", "-U", &ctx.user, "-G", "admin"],
            )
            .contains("is a member"),
        ),
        "hostname" => {
            let name = run("scutil", &["--get", "ComputerName"])
                .trim()
                .to_lowercase();
            if name.is_empty() {
                return Outcome::skip();
            }
            let full = run("id", &["-F", &ctx.user]).to_lowercase();
            let leaks = name.contains(&ctx.user.to_lowercase())
                || full
                    .split_whitespace()
                    .any(|w| w.chars().count() >= 3 && name.contains(w));
            Outcome::when(!leaks)
        }
        "sudo_home" => {
            if !ctx.root {
                return Outcome::new(Status::Skip, ctx.t("needs sudo", "cần sudo"));
            }
            let mut text = std::fs::read_to_string("/etc/sudoers").unwrap_or_default();
            if let Ok(rd) = std::fs::read_dir("/etc/sudoers.d") {
                for e in rd.flatten() {
                    text.push('\n');
                    text.push_str(&std::fs::read_to_string(e.path()).unwrap_or_default());
                }
            }
            let rules: Vec<&str> = text
                .lines()
                .map(str::trim)
                .filter(|l| {
                    l.starts_with("Defaults") && l.contains("env_keep") && l.contains("HOME")
                })
                .collect();
            Outcome::when(rules.is_empty() || rules.iter().any(|l| l.contains("-=")))
        }

        "firewall" => {
            let out = run(FW, &["--getglobalstate"]);
            Outcome::when(out.contains("State = 1") || out.contains("State = 2"))
        }
        "stealth" => Outcome::when(run(FW, &["--getstealthmode"]).contains("stealth mode is on")),
        "signed" => {
            let on: Vec<String> = run(FW, &["--getallowsigned"])
                .lines()
                .filter(|l| l.contains("ENABLED"))
                .map(|l| {
                    l.replace("Automatically allow ", "")
                        .split(" software")
                        .next()
                        .unwrap_or("")
                        .to_string()
                })
                .collect();
            if on.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, join(&on))
            }
        }
        "outbound_fw" => match ["LuLu", "Little Snitch", "Radio Silence"]
            .iter()
            .find(|a| exists(&format!("/Applications/{a}.app")))
        {
            Some(a) => Outcome::new(Status::Pass, *a),
            None => Outcome::fail(),
        },
        "dns" => {
            let ns = run("scutil", &["--dns"]).lines().find_map(|l| {
                l.split("nameserver[0] : ")
                    .nth(1)
                    .map(|s| s.trim().to_string())
            });
            if matches!(ns.as_deref(), Some("127.0.0.1" | "::1")) {
                return Outcome::new(
                    Status::Pass,
                    ctx.t("local resolver", "bộ phân giải trên máy"),
                );
            }
            if run("system_profiler", &["SPConfigurationProfileDataType"])
                .to_lowercase()
                .contains("dnssettings")
            {
                return Outcome::new(Status::Pass, "DNS profile");
            }
            Outcome::fail()
        }
        "hosts_blocklist" => {
            let hosts = std::fs::read_to_string("/etc/hosts").unwrap_or_default();
            let n = hosts
                .lines()
                .filter(|l| {
                    let f: Vec<&str> = l.split_whitespace().collect();
                    f.len() >= 2
                        && matches!(f[0], "0" | "0.0.0.0" | "127.0.0.1")
                        && !f[1].starts_with('#')
                        && f[1] != "localhost"
                })
                .count();
            Outcome::new(
                if n >= 100 { Status::Pass } else { Status::Fail },
                ctx.lang
                    .t(&format!("{n} blocked"), &format!("chặn {n}"))
                    .to_string(),
            )
        }
        "certs" => {
            let count = |args: &[&str]| {
                ctx.as_user("security", args)
                    .lines()
                    .filter(|l| {
                        l.starts_with("Cert ") && l[5..].starts_with(|c: char| c.is_ascii_digit())
                    })
                    .count()
            };
            let n = count(&["dump-trust-settings"]) + count(&["dump-trust-settings", "-d"]);
            if n == 0 {
                Outcome::pass()
            } else {
                Outcome::new(
                    Status::Fail,
                    ctx.lang
                        .t(&format!("{n} certificate(s)"), &format!("{n} chứng chỉ"))
                        .to_string(),
                )
            }
        }
        "wifi" => {
            let ports = run("networksetup", &["-listallhardwareports"]);
            let mut lines = ports.lines();
            let mut dev = None;
            while let Some(l) = lines.next() {
                if l.contains("Wi-Fi") || l.contains("AirPort") {
                    dev = lines
                        .next()
                        .and_then(|d| d.split("Device: ").nth(1))
                        .map(|d| d.trim().to_string());
                    break;
                }
            }
            let Some(dev) = dev else {
                return Outcome::skip();
            };
            match field(&run("ipconfig", &["getsummary", &dev]), "Security") {
                Some(sec) => Outcome::new(
                    if sec.to_lowercase().contains("wpa3") {
                        Status::Pass
                    } else {
                        Status::Fail
                    },
                    sec,
                ),
                None => Outcome::skip(),
            }
        }
        "bonjour" => Outcome::when(is_true(
            default(
                "/Library/Preferences/com.apple.mDNSResponder",
                "NoMulticastAdvertisements",
            )
            .as_deref(),
        )),
        "handoff" => {
            // A missing key means the default, which is on.
            let v = ctx.user_default(&[
                "-currentHost",
                "com.apple.coreservices.useractivityd",
                "ActivityAdvertisingAllowed",
            ]);
            Outcome::when(v.is_some() && !is_true(v.as_deref()))
        }
        "bluetooth" => match field(&run("system_profiler", &["SPBluetoothDataType"]), "State") {
            Some(s) => Outcome::when(s != "On"),
            None => Outcome::skip(),
        },

        "ssh" => Outcome::when(!ssh_on()),
        "ssh_password" => {
            if !ssh_on() {
                return Outcome::skip();
            }
            // sshd keeps the first value it reads; drop-in files are included first.
            let mut files: Vec<PathBuf> = std::fs::read_dir("/etc/ssh/sshd_config.d")
                .map(|rd| {
                    rd.flatten()
                        .map(|e| e.path())
                        .filter(|p| p.extension().is_some_and(|x| x == "conf"))
                        .collect()
                })
                .unwrap_or_default();
            files.sort();
            files.push(PathBuf::from("/etc/ssh/sshd_config"));
            let text: String = files
                .iter()
                .map(|p| std::fs::read_to_string(p).unwrap_or_default() + "\n")
                .collect();
            let first = |key: &str| {
                text.lines().find_map(|l| {
                    let mut f = l.split_whitespace();
                    (f.next().is_some_and(|k| k.eq_ignore_ascii_case(key)))
                        .then(|| f.next().unwrap_or("").to_lowercase())
                })
            };
            Outcome::when(
                first("PasswordAuthentication").as_deref() == Some("no")
                    && first("KbdInteractiveAuthentication").as_deref() == Some("no"),
            )
        }
        "screensharing" => {
            Outcome::when(!run_status("launchctl", &["print", "system/com.apple.screensharing"]).0)
        }
        "airplay" => Outcome::when(
            !listen_sockets()
                .iter()
                .any(|(c, n)| c.starts_with("ControlCe") && n.starts_with("*:")),
        ),
        "listeners" => {
            // Control Center is covered by the airplay item.
            let mut open: Vec<String> = listen_sockets()
                .into_iter()
                .filter(|(c, n)| {
                    !c.starts_with("ControlCe")
                        && !n.starts_with("127.")
                        && !n.starts_with("[::1]")
                        && !n.starts_with("localhost")
                })
                .map(|(c, n)| format!("{c} {n}"))
                .collect();
            open.sort();
            open.dedup();
            if open.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, join(&open))
            }
        }

        "launch_items" => {
            let dirs = [
                ctx.home.join("Library/LaunchAgents"),
                "/Library/LaunchAgents".into(),
                "/Library/LaunchDaemons".into(),
            ];
            let mut items: Vec<String> = dirs
                .iter()
                .filter_map(|d| std::fs::read_dir(d).ok())
                .flat_map(|rd| rd.flatten())
                .filter_map(|e| e.file_name().to_str().map(str::to_string))
                .filter(|n| n.ends_with(".plist") && !n.starts_with("com.apple."))
                .map(|n| n.trim_end_matches(".plist").to_string())
                .collect();
            items.sort();
            items.dedup();
            if items.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, format!("{}: {}", items.len(), join(&items)))
            }
        }
        "sysext" => {
            let mut names: Vec<String> = run("systemextensionsctl", &["list"])
                .lines()
                .filter_map(|l| {
                    let f: Vec<&str> = l.split('\t').collect();
                    (f.len() > 4 && f[0] == "*" && f[1] == "*").then(|| f[4].trim().to_string())
                })
                .collect();
            names.sort();
            names.dedup();
            if names.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, join(&names))
            }
        }
        "app_runtime" => {
            let mut apps: Vec<PathBuf> = std::fs::read_dir("/Applications")
                .map(|rd| {
                    rd.flatten()
                        .map(|e| e.path())
                        .filter(|p| p.extension().is_some_and(|x| x == "app"))
                        .collect()
                })
                .unwrap_or_default();
            apps.sort();
            // Apple's own apps (e.g. Safari) ship sealed with macOS; only third-party apps are listed.
            let missing: Vec<String> = std::thread::scope(|s| {
                let handles: Vec<_> = apps
                    .iter()
                    .map(|a| {
                        s.spawn(move || {
                            let info = run("codesign", &["-dv", &a.to_string_lossy()]);
                            let apple =
                                info.lines().any(|l| l.starts_with("Identifier=com.apple."));
                            (!apple && !info.contains("runtime")).then(|| {
                                a.file_stem()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .into_owned()
                            })
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .filter_map(|h| h.join().ok().flatten())
                    .collect()
            });
            if missing.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, join(&missing))
            }
        }
        "brew_analytics" => {
            if !brew_installed() {
                return Outcome::skip();
            }
            if env_set(ctx, "HOMEBREW_NO_ANALYTICS") {
                return Outcome::pass();
            }
            Outcome::when(
                !ctx.as_user(brew_path(), &["analytics", "state"])
                    .contains("are enabled"),
            )
        }
        "brew_redirect" => {
            if !brew_installed() {
                return Outcome::skip();
            }
            Outcome::when(env_set(ctx, "HOMEBREW_NO_INSECURE_REDIRECT"))
        }

        "diagnostics" => {
            if !exists(DIAG) {
                return Outcome::skip();
            }
            Outcome::when(
                !is_true(default(DIAG, "AutoSubmit").as_deref())
                    && !is_true(default(DIAG, "ThirdPartyDataSubmit").as_deref()),
            )
        }
        "crash_dialog" => Outcome::when(
            ctx.user_default(&["com.apple.CrashReporter", "DialogType"])
                .as_deref()
                == Some("none"),
        ),
        "icloud_save" => {
            let v = ctx.user_default(&["NSGlobalDomain", "NSDocumentSaveNewDocumentsToCloud"]);
            Outcome::when(v.is_some() && !is_true(v.as_deref()))
        }
        "extensions" => Outcome::when(is_true(
            ctx.user_default(&["NSGlobalDomain", "AppleShowAllExtensions"])
                .as_deref(),
        )),
        "hidden_files" => {
            let shown = is_true(
                ctx.user_default(&["com.apple.finder", "AppleShowAllFiles"])
                    .as_deref(),
            );
            let lib = ctx.home.join("Library");
            let lib_hidden = run("stat", &["-f", "%Sf", &lib.to_string_lossy()]).contains("hidden");
            Outcome::when(shown && !lib_hidden)
        }
        // launchd stores the umask in decimal: 077 octal is 63.
        "umask" => Outcome::when(
            default(
                "/private/var/db/com.apple.xpc.launchd/config/user.plist",
                "Umask",
            )
            .as_deref()
                == Some("63"),
        ),
        "terminal_keyboard" => Outcome::when(is_true(
            ctx.user_default(&["com.apple.Terminal", "SecureKeyboardEntry"])
                .as_deref(),
        )),

        "file_sharing" => port_check(445),
        "remote_management" => port_check(3283),
        "remote_apple_events" => port_check(3031),
        "media_sharing" => port_check(3689),
        "printer_sharing" => {
            if !exists("/usr/sbin/cupsctl") {
                return Outcome::skip();
            }
            let out = run("cupsctl", &[]);
            let on: Vec<String> = ["_share_printers", "_remote_admin", "_remote_any"]
                .iter()
                .filter(|k| out.contains(&format!("{k}=1")))
                .map(|k| k.trim_start_matches('_').to_string())
                .collect();
            if on.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, join(&on))
            }
        }
        "internet_sharing" => {
            // A missing file or key means Internet Sharing was never turned on.
            let v = run_status(
                "/usr/libexec/PlistBuddy",
                &[
                    "-c",
                    "Print :NAT:Enabled",
                    "/Library/Preferences/SystemConfiguration/com.apple.nat.plist",
                ],
            );
            Outcome::when(!(v.0 && v.1.trim() == "1"))
        }

        "sudo_password" => {
            if !ctx.root {
                return Outcome::new(Status::Skip, ctx.t("needs sudo", "cần sudo"));
            }
            let mut files = vec![PathBuf::from("/etc/sudoers")];
            if let Ok(rd) = std::fs::read_dir("/etc/sudoers.d") {
                files.extend(rd.flatten().map(|e| e.path()));
            }
            let rules: Vec<String> = files
                .iter()
                .filter_map(|p| std::fs::read_to_string(p).ok())
                .flat_map(|t| {
                    t.lines()
                        .map(str::trim)
                        .filter(|l| !l.starts_with('#') && l.contains("NOPASSWD"))
                        .map(str::to_string)
                        .collect::<Vec<_>>()
                })
                .collect();
            if rules.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, rules.join("; "))
            }
        }
        "ssh_key_passphrase" => {
            let keys = ssh_private_keys(ctx);
            if keys.is_empty() {
                return Outcome::skip();
            }
            let open: Vec<String> = keys
                .iter()
                .filter(|k| {
                    let path = k.to_string_lossy();
                    // With an empty passphrase, ssh-keygen succeeds only on unprotected keys.
                    let (ok, out) =
                        ctx.as_user_status("ssh-keygen", &["-y", "-P", "", "-f", &path]);
                    ok && !out.starts_with("sk-")
                })
                .map(|k| file_name(k))
                .collect();
            if open.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, join(&open))
            }
        }
        "ssh_key_strength" => {
            let mut weak = Vec::new();
            let mut seen = false;
            for k in ssh_private_keys(ctx) {
                let public = PathBuf::from(format!("{}.pub", k.to_string_lossy()));
                if !public.exists() {
                    continue;
                }
                seen = true;
                // e.g. "3072 SHA256:... comment (RSA)"
                let out = run("ssh-keygen", &["-l", "-f", &public.to_string_lossy()]);
                let bits: u32 = out
                    .split_whitespace()
                    .next()
                    .and_then(|b| b.parse().ok())
                    .unwrap_or(0);
                let kind = out
                    .rsplit('(')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .trim_end_matches(')')
                    .to_uppercase();
                if kind == "DSA" || (kind == "RSA" && bits < 3072) {
                    weak.push(format!("{} ({kind} {bits})", file_name(&k)));
                }
            }
            if !seen {
                Outcome::skip()
            } else if weak.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, join(&weak))
            }
        }
        "ssh_host_keys" => {
            let bad: Vec<String> = ssh_client_options(ctx)
                .into_iter()
                .filter(|(k, v)| {
                    (k == "stricthostkeychecking" && (v == "no" || v == "off"))
                        || (k == "userknownhostsfile" && v == "/dev/null")
                })
                .map(|(k, v)| format!("{k} {v}"))
                .collect();
            if bad.is_empty() {
                Outcome::pass()
            } else {
                Outcome::new(Status::Fail, join(&bad))
            }
        }
        "ssh_agent_forwarding" => Outcome::when(
            !ssh_client_options(ctx)
                .iter()
                .any(|(k, v)| k == "forwardagent" && v == "yes"),
        ),
        _ => Outcome::new(Status::Skip, format!("no check for {id}")),
    }
}

/// Checks every item in parallel and sends (index, outcome) as each finishes.
/// Ignored items are reported as Ignored without being checked.
pub fn scan(defs: &[Def], ignored: &[String], ctx: &Ctx, tx: mpsc::Sender<(usize, Outcome)>) {
    std::thread::scope(|s| {
        for (i, d) in defs.iter().enumerate() {
            let tx = tx.clone();
            let skip = ignored.contains(&d.id);
            s.spawn(move || {
                let o = if skip {
                    Outcome::new(Status::Ignored, "")
                } else {
                    check(&d.id, ctx)
                };
                let _ = tx.send((i, o));
            });
        }
    });
}

/// Scans and waits for all results, in definition order.
pub fn scan_all(defs: &[Def], ignored: &[String], ctx: &Ctx) -> Vec<Outcome> {
    let (tx, rx) = mpsc::channel();
    scan(defs, ignored, ctx, tx);
    let mut out = vec![Outcome::skip(); defs.len()];
    for (i, o) in rx.try_iter() {
        out[i] = o;
    }
    out
}

/// Checks one item, honouring the ignore list.
pub fn check_one(def: &Def, ignored: &[String], ctx: &Ctx) -> Outcome {
    if ignored.contains(&def.id) {
        Outcome::new(Status::Ignored, "")
    } else {
        check(&def.id, ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_days() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(2000, 3, 1), 11017);
    }

    #[test]
    fn field_parsing() {
        assert_eq!(
            field("  Mount Point   : /Volumes/TM\n", "Mount Point").as_deref(),
            Some("/Volumes/TM")
        );
        assert_eq!(field("State: On", "State").as_deref(), Some("On"));
        assert_eq!(field("nothing", "State"), None);
    }

    #[test]
    fn detects_listening_port() {
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = l.local_addr().unwrap().port();
        assert!(port_open(port));
        // A freed ephemeral port can be reused by the probing socket itself, so test
        // the closed case on port 1 (tcpmux), which macOS never listens on.
        drop(l);
        assert!(!port_open(1));
    }

    /// Builds a throwaway home with SSH keys and a client config, and checks the SSH items.
    #[test]
    fn ssh_checks() {
        let home = std::env::temp_dir().join(format!("macguard-ssh-{}", std::process::id()));
        let ssh = home.join(".ssh");
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&ssh).unwrap();
        let ctx = Ctx {
            lang: Lang::En,
            root: false,
            user: "nobody".into(),
            home: home.clone(),
        };
        let keygen = |name: &str, kind: &str, bits: &str, pass: &str| {
            let path = ssh.join(name);
            let ok = Command::new("ssh-keygen")
                .args(["-q", "-t", kind, "-b", bits, "-N", pass, "-C", "", "-f"])
                .arg(&path)
                .status()
                .unwrap()
                .success();
            assert!(ok, "ssh-keygen {name}");
        };
        // as_user runs commands directly when not root, so the user name is irrelevant here.
        keygen("open_ed25519", "ed25519", "256", "");
        keygen("locked_ed25519", "ed25519", "256", "correct horse");
        keygen("weak_rsa", "rsa", "2048", "correct horse");

        let o = check("ssh_key_passphrase", &ctx);
        assert_eq!(o.status, Status::Fail);
        assert_eq!(o.detail, "open_ed25519");

        let o = check("ssh_key_strength", &ctx);
        assert_eq!(o.status, Status::Fail);
        assert!(o.detail.starts_with("weak_rsa (RSA 2048)"), "{}", o.detail);

        assert_eq!(check("ssh_host_keys", &ctx).status, Status::Pass);
        assert_eq!(check("ssh_agent_forwarding", &ctx).status, Status::Pass);
        std::fs::write(
            ssh.join("config"),
            "Host *\n  StrictHostKeyChecking no\n  ForwardAgent=yes\n# UserKnownHostsFile /dev/null\n",
        )
        .unwrap();
        let o = check("ssh_host_keys", &ctx);
        assert_eq!(
            (o.status, o.detail.as_str()),
            (Status::Fail, "stricthostkeychecking no")
        );
        assert_eq!(check("ssh_agent_forwarding", &ctx).status, Status::Fail);
        let _ = std::fs::remove_dir_all(&home);
    }
}
