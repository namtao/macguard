//! Shared presentation: colors, wrapping, marks, ordering, scoring and the
//! analysis text used by both the menu and the plain commands.

use crate::checks::{Outcome, Status};
use crate::data::{Def, Lang, Severity};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Clone, Copy)]
pub struct Colors {
    pub on: bool,
}

impl Colors {
    fn wrap(self, code: &str, s: &str) -> String {
        if self.on {
            format!("\x1b[{code}m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    pub fn red(self, s: &str) -> String {
        self.wrap("31", s)
    }
    pub fn yellow(self, s: &str) -> String {
        self.wrap("33", s)
    }
    pub fn green(self, s: &str) -> String {
        self.wrap("32", s)
    }
    pub fn dim(self, s: &str) -> String {
        self.wrap("2", s)
    }
    pub fn bold(self, s: &str) -> String {
        self.wrap("1", s)
    }
    pub fn reverse(self, s: &str) -> String {
        self.wrap("7", s)
    }
}

/// Pads plain text with spaces to a display width.
pub fn pad(s: &str, w: usize) -> String {
    let cur = UnicodeWidthStr::width(s);
    if cur >= w {
        s.to_string()
    } else {
        format!("{s}{}", " ".repeat(w - cur))
    }
}

/// Word-wraps plain text to a display width; words longer than the width are split.
pub fn wrap(text: &str, max: usize) -> Vec<String> {
    let max = max.max(10);
    let mut lines = Vec::new();
    for para in text.split('\n') {
        let mut line = String::new();
        let mut lw = 0;
        for word in para.split(' ') {
            let ww = UnicodeWidthStr::width(word);
            if lw > 0 && lw + 1 + ww > max {
                lines.push(std::mem::take(&mut line));
                lw = 0;
            }
            if lw > 0 {
                line.push(' ');
                lw += 1;
            }
            for c in word.chars() {
                let cw = c.width().unwrap_or(0);
                if lw + cw > max {
                    lines.push(std::mem::take(&mut line));
                    lw = 0;
                }
                line.push(c);
                lw += cw;
            }
        }
        lines.push(line);
    }
    lines
}

pub fn mark(status: Option<Status>, sev: Severity, c: Colors) -> String {
    match (status, sev) {
        (None, _) => c.dim("…"),
        (Some(Status::Pass), _) => c.green("✓"),
        (Some(Status::Skip), _) => c.dim("-"),
        (Some(Status::Ignored), _) => c.dim("⊘"),
        (Some(Status::Fail), Severity::High) => c.red("✗"),
        (Some(Status::Fail), Severity::Med) => c.yellow("✗"),
        (Some(Status::Fail), Severity::Low) => c.yellow("!"),
        (Some(Status::Fail), Severity::Info) => c.dim("i"),
    }
}

/// Display order: by severity, failing first, then passing, not applicable, ignored.
pub fn order(defs: &[Def], results: &[Option<Outcome>]) -> Vec<usize> {
    let rank = |o: &Option<Outcome>| match o.as_ref().map(|o| o.status) {
        Some(Status::Fail) => 0,
        None | Some(Status::Pass) => 1,
        Some(Status::Skip) => 2,
        Some(Status::Ignored) => 3,
    };
    let mut idx: Vec<usize> = (0..defs.len()).collect();
    idx.sort_by_key(|&i| (defs[i].severity, rank(&results[i]), i));
    idx
}

pub struct Score {
    pub pass: usize,
    pub total: usize,
    pub serious: usize,
}

pub fn score(defs: &[Def], results: &[Option<Outcome>]) -> Score {
    let mut s = Score {
        pass: 0,
        total: 0,
        serious: 0,
    };
    for (d, o) in defs.iter().zip(results) {
        let Some(o) = o else { continue };
        if d.severity == Severity::Info {
            continue;
        }
        match o.status {
            Status::Pass => {
                s.pass += 1;
                s.total += 1;
            }
            Status::Fail => {
                s.total += 1;
                if d.severity <= Severity::Med {
                    s.serious += 1;
                }
            }
            _ => {}
        }
    }
    s
}

pub fn score_line(s: &Score, lang: Lang, c: Colors) -> String {
    let mut out = format!(
        "{} {}",
        lang.t("Score", "Điểm"),
        c.bold(&format!("{}/{}", s.pass, s.total))
    );
    if s.serious > 0 {
        let msg = match lang {
            Lang::En => format!("{} high/medium failing", s.serious),
            Lang::Vi => format!("{} mục cao/vừa chưa đạt", s.serious),
        };
        out.push_str(&format!(" · {}", c.red(&msg)));
    }
    out
}

pub fn status_text(status: Status, lang: Lang) -> &'static str {
    match status {
        Status::Pass => lang.t("passing", "đạt"),
        Status::Fail => lang.t("not passing", "chưa đạt"),
        Status::Ignored => lang.t("ignored (not checked)", "đang bỏ qua (không kiểm)"),
        Status::Skip => lang.t("not applicable", "không áp dụng"),
    }
}

/// The full analysis of one item, wrapped to a width.
pub fn analysis(def: &Def, o: &Outcome, lang: Lang, c: Colors, w: usize) -> Vec<String> {
    let mut lines = vec![format!(
        "{} {} {}",
        mark(Some(o.status), def.severity, c),
        c.bold(def.title.get(lang)),
        c.dim(&format!(
            "({}, {}, {})",
            def.id,
            def.severity.name(lang),
            def.category.name(lang)
        ))
    )];
    let mut field = |label: &str, body: &str| {
        if body.is_empty() {
            return;
        }
        lines.push(format!("  {}", c.bold(label)));
        for raw in body.lines() {
            for l in wrap(raw, w.saturating_sub(4)) {
                lines.push(format!("    {l}"));
            }
        }
    };
    let status = if o.detail.is_empty() {
        status_text(o.status, lang).to_string()
    } else {
        format!("{} ({})", status_text(o.status, lang), o.detail)
    };
    field(lang.t("Status", "Trạng thái"), &status);
    field(lang.t("What it does", "Tác dụng"), def.why.get(lang));
    match def.fix_display() {
        Some(cmd) => field(lang.t("Command that will run", "Lệnh sẽ chạy"), &cmd),
        None => field(
            lang.t("How to fix (manual)", "Cách sửa (làm tay)"),
            def.how.get(lang),
        ),
    }
    field(
        lang.t("Risk if you do not fix it", "Rủi ro nếu không sửa"),
        def.skip.get(lang),
    );
    field(
        lang.t("Risk when you fix it", "Rủi ro khi sửa"),
        def.risk.get(lang),
    );
    field(lang.t("Source", "Nguồn"), def.source(lang));
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_by_display_width() {
        let lines = wrap("Đòi mật khẩu ngay khi ngủ hoặc bật màn hình chờ", 20);
        assert!(
            lines
                .iter()
                .all(|l| UnicodeWidthStr::width(l.as_str()) <= 20),
            "{lines:?}"
        );
        assert_eq!(
            lines.join(" "),
            "Đòi mật khẩu ngay khi ngủ hoặc bật màn hình chờ"
        );
    }

    #[test]
    fn splits_long_words() {
        let lines = wrap("com.example.a-very-long-identifier", 10);
        assert!(lines.iter().all(|l| l.chars().count() <= 10));
        assert_eq!(lines.concat(), "com.example.a-very-long-identifier");
    }
}
