//! Check definitions and tips, embedded from `data/*.toml`, plus language handling.

use serde::Deserialize;
use std::process::Command;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    En,
    Vi,
}

impl Lang {
    /// `--lang` or MACGUARD_LANG, then the first macOS preferred language, then LANG.
    pub fn detect(flag: Option<&str>) -> Lang {
        let pick = |s: &str| {
            if s.to_lowercase().starts_with("vi") {
                Lang::Vi
            } else {
                Lang::En
            }
        };
        if let Some(f) = flag {
            return pick(f);
        }
        if let Ok(v) = std::env::var("MACGUARD_LANG") {
            if !v.is_empty() {
                return pick(&v);
            }
        }
        if let Ok(out) = Command::new("defaults")
            .args(["read", "-g", "AppleLanguages"])
            .output()
        {
            let text = String::from_utf8_lossy(&out.stdout);
            if let Some(first) = text.lines().nth(1) {
                let code: String = first
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic() || *c == '-')
                    .collect();
                if !code.is_empty() {
                    return pick(&code);
                }
            }
        }
        pick(&std::env::var("LANG").unwrap_or_default())
    }

    /// Picks the text for this language.
    pub fn t<'a>(self, en: &'a str, vi: &'a str) -> &'a str {
        match self {
            Lang::En => en,
            Lang::Vi => vi,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Text {
    pub en: String,
    pub vi: String,
}

impl Text {
    pub fn get(&self, lang: Lang) -> &str {
        lang.t(&self.en, &self.vi)
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    High,
    Med,
    Low,
    Info,
}

impl Severity {
    pub const ALL: [Severity; 4] = [Severity::High, Severity::Med, Severity::Low, Severity::Info];

    pub fn name(self, lang: Lang) -> &'static str {
        match self {
            Severity::High => lang.t("high", "cao"),
            Severity::Med => lang.t("medium", "vừa"),
            Severity::Low => lang.t("low", "thấp"),
            Severity::Info => lang.t("info", "tham khảo"),
        }
    }

    /// Group heading used in the menu and the report.
    pub fn heading(self, lang: Lang) -> &'static str {
        match self {
            Severity::High => lang.t("High priority", "Mức cao"),
            Severity::Med => lang.t("Medium priority", "Mức vừa"),
            Severity::Low => lang.t("Low priority", "Mức thấp"),
            Severity::Info => lang.t("For review (not scored)", "Tham khảo (không tính điểm)"),
        }
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    System,
    Accounts,
    Network,
    Sharing,
    Software,
    Privacy,
}

impl Category {
    pub fn name(self, lang: Lang) -> &'static str {
        match self {
            Category::System => lang.t("System", "Hệ thống"),
            Category::Accounts => lang.t("Accounts and login", "Tài khoản và đăng nhập"),
            Category::Network => lang.t("Network and firewall", "Mạng và tường lửa"),
            Category::Sharing => lang.t("Sharing and open ports", "Chia sẻ và cổng mở"),
            Category::Software => lang.t("Software", "Phần mềm"),
            Category::Privacy => lang.t("Privacy", "Quyền riêng tư"),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Def {
    pub id: String,
    pub severity: Severity,
    pub category: Category,
    #[serde(default)]
    pub extra: bool,
    pub settings: Option<String>,
    pub title: Text,
    pub why: Text,
    pub skip: Text,
    pub risk: Text,
    pub how: Text,
    pub fix: Option<String>,
}

impl Def {
    /// The fix as shown to the user: `as_user` is an implementation detail.
    pub fn fix_display(&self) -> Option<String> {
        self.fix
            .as_ref()
            .map(|f| f.trim_end().replace("as_user ", ""))
    }

    pub fn source(&self, lang: Lang) -> &'static str {
        if self.extra {
            lang.t(
                "Not in the guide; common hardening added by macguard",
                "Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến",
            )
        } else {
            lang.t(
                "drduh macOS Security and Privacy Guide",
                "Tài liệu macOS Security and Privacy Guide của drduh",
            )
        }
    }
}

#[derive(Deserialize)]
struct ChecksFile {
    check: Vec<Def>,
}

#[derive(Deserialize, Debug)]
pub struct TipSection {
    pub title: Text,
    pub tips: Vec<Text>,
}

#[derive(Deserialize)]
struct TipsFile {
    section: Vec<TipSection>,
}

pub fn defs() -> Vec<Def> {
    toml::from_str::<ChecksFile>(include_str!("../data/checks.toml"))
        .expect("data/checks.toml is valid")
        .check
}

pub fn tips() -> Vec<TipSection> {
    toml::from_str::<TipsFile>(include_str!("../data/tips.toml"))
        .expect("data/tips.toml is valid")
        .section
}
