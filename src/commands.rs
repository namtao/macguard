//! Ignore list, applying fixes, and the non-interactive commands.

use crate::checks::{self, Ctx, Outcome, Status};
use crate::data::{self, Def, Lang, Severity};
use crate::ui::{self, Colors};
use std::io::{BufRead, IsTerminal, Write};
use std::path::PathBuf;
use std::process::Command;

pub struct App {
    pub defs: Vec<Def>,
    pub ctx: Ctx,
    pub lang: Lang,
    pub colors: Colors,
}

// ---------- Ignore list ----------
// Ignored items are not checked, scored or offered for fixing until un-ignored.
// Stored per user in ~/.config/macguard/ignored, one id per line.

impl App {
    pub fn ignore_file(&self) -> PathBuf {
        match std::env::var("MACGUARD_CONFIG_DIR") {
            Ok(d) if !d.is_empty() => PathBuf::from(d).join("ignored"),
            _ => self.ctx.home.join(".config/macguard/ignored"),
        }
    }

    pub fn ignored(&self) -> Vec<String> {
        std::fs::read_to_string(self.ignore_file())
            .map(|s| {
                s.lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty())
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn set_ignored(&self, id: &str, on: bool) -> std::io::Result<()> {
        let mut list: Vec<String> = self.ignored().into_iter().filter(|x| x != id).collect();
        if on {
            list.push(id.to_string());
        }
        let path = self.ignore_file();
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let body = if list.is_empty() {
            String::new()
        } else {
            list.join("\n") + "\n"
        };
        std::fs::write(&path, body)?;
        // Under sudo, keep the file owned by the user it belongs to.
        if self.ctx.root && self.ctx.user != "root" {
            for p in [path.parent().map(PathBuf::from), Some(path.clone())]
                .into_iter()
                .flatten()
            {
                let _ = Command::new("chown").arg(&self.ctx.user).arg(p).status();
            }
        }
        Ok(())
    }

    pub fn find(&self, id: &str) -> Option<usize> {
        self.defs.iter().position(|d| d.id == id)
    }

    pub fn t<'a>(&self, en: &'a str, vi: &'a str) -> &'a str {
        self.lang.t(en, vi)
    }

    pub fn header(&self) -> String {
        let ver = checks::run("sw_vers", &["-productVersion"]);
        let mut h = format!(
            "{} · macOS {}",
            self.colors
                .bold(&format!("macguard {}", env!("CARGO_PKG_VERSION"))),
            ver.trim()
        );
        if !self.ctx.root {
            h.push_str(&format!(
                " · {}",
                self.t(
                    "run with sudo for the full audit",
                    "chạy với sudo để kiểm đủ"
                )
            ));
        }
        h
    }

    pub fn check_one(&self, i: usize) -> Outcome {
        checks::check_one(&self.defs[i], &self.ignored(), &self.ctx)
    }

    pub fn open_settings(&self, def: &Def) -> bool {
        match &def.settings {
            Some(url) => Command::new("open").arg(url).status().is_ok(),
            None => false,
        }
    }
}

// ---------- Fixing ----------

/// Asks a yes/no question. Without an interactive terminal the answer is always no,
/// so piping or scripting macguard can never apply a fix by accident.
pub fn confirm(question: &str) -> bool {
    print!("{question} [y/N] ");
    let _ = std::io::stdout().flush();
    if !std::io::stdin().is_terminal() {
        println!();
        return false;
    }
    let mut line = String::new();
    if std::io::stdin().lock().read_line(&mut line).is_err() {
        return false;
    }
    matches!(line.trim().chars().next(), Some('y' | 'Y'))
}

/// Runs a fix script with bash. `as_user` runs a command as the audited user even under sudo.
fn run_fix(app: &App, script: &str) {
    let prelude = r#"as_user() { if [ "$(id -u)" = 0 ] && [ "$MACGUARD_USER" != root ]; then sudo -H -u "$MACGUARD_USER" "$@"; else "$@"; fi; }"#;
    let _ = Command::new("/bin/bash")
        .arg("-c")
        .arg(format!("{prelude}\n{script}"))
        .env("MACGUARD_USER", &app.ctx.user)
        .env("TARGET_HOME", &app.ctx.home)
        .status();
}

pub enum FixResult {
    Fixed,
    NotFixed,
    NothingToFix,
}

/// Shows the analysis of a failing item, asks, applies the fix and checks again.
pub fn fix_one(app: &App, i: usize) -> FixResult {
    let def = &app.defs[i];
    let o = app.check_one(i);
    if o.status != Status::Fail {
        return FixResult::NothingToFix;
    }
    let c = app.colors;
    for l in ui::analysis(def, &o, app.lang, c, 100) {
        println!("{l}");
    }
    let Some(script) = &def.fix else {
        if def.settings.is_some()
            && confirm(&format!(
                "  {}",
                app.t("Open System Settings?", "Mở Cài đặt hệ thống?")
            ))
        {
            app.open_settings(def);
        }
        return FixResult::NotFixed;
    };
    if !confirm(&format!("  {}", app.t("Apply this fix?", "Sửa mục này?"))) {
        return FixResult::NotFixed;
    }
    run_fix(app, script);
    if app.check_one(i).status == Status::Pass {
        println!(
            "  {}",
            c.green(&format!("✓ {}", app.t("Fixed.", "Đã sửa.")))
        );
        return FixResult::Fixed;
    }
    println!(
        "  {}",
        c.red(&format!(
            "✗ {}",
            app.t("Still not passing.", "Vẫn chưa đạt.")
        ))
    );
    println!(
        "  {}",
        c.bold(app.t("How to fix (manual)", "Cách sửa (làm tay)"))
    );
    for l in ui::wrap(def.how.get(app.lang), 96) {
        println!("    {l}");
    }
    if def.settings.is_some()
        && confirm(&format!(
            "  {}",
            app.t("Open System Settings?", "Mở Cài đặt hệ thống?")
        ))
    {
        app.open_settings(def);
    }
    FixResult::NotFixed
}

// ---------- Commands ----------

fn select(app: &App, ids: &[String]) -> Result<Vec<usize>, String> {
    if ids.is_empty() {
        return Ok((0..app.defs.len()).collect());
    }
    ids.iter()
        .map(|id| {
            app.find(id).ok_or_else(|| match app.lang {
                Lang::En => format!("Unknown item '{id}'. See 'macguard list'."),
                Lang::Vi => format!("Không có mục '{id}'. Xem 'macguard list'."),
            })
        })
        .collect()
}

/// Plain report grouped by severity. Returns the exit code: 1 when a high or medium item fails.
pub fn check(app: &App) -> i32 {
    let c = app.colors;
    let results: Vec<Option<Outcome>> = checks::scan_all(&app.defs, &app.ignored(), &app.ctx)
        .into_iter()
        .map(Some)
        .collect();
    println!("{}", app.header());
    let mut last = None;
    let mut ignored = Vec::new();
    for i in ui::order(&app.defs, &results) {
        let (d, o) = (&app.defs[i], results[i].as_ref().unwrap());
        if o.status == Status::Ignored {
            ignored.push(d.id.clone());
            continue;
        }
        if o.hidden() {
            continue;
        }
        if last != Some(d.severity) {
            println!("\n{}", c.bold(d.severity.heading(app.lang)));
            last = Some(d.severity);
        }
        let detail = if o.detail.is_empty() {
            String::new()
        } else {
            format!(" {}", c.dim(&format!("({})", o.detail)))
        };
        println!(
            " {} {} {}{}",
            ui::mark(Some(o.status), d.severity, c),
            c.dim(&ui::pad(&d.id, ui::ID_WIDTH)),
            d.title.get(app.lang),
            detail
        );
    }
    println!();
    if !ignored.is_empty() {
        let list = ignored.join(", ");
        println!(
            "{}",
            c.dim(&match app.lang {
                Lang::En => format!("Ignored: {list}. Undo with 'macguard unignore <id>'."),
                Lang::Vi =>
                    format!("Đang bỏ qua: {list}. Bỏ chế độ này bằng 'macguard unignore <mã>'."),
            })
        );
    }
    let s = ui::score(&app.defs, &results);
    println!("{}", ui::score_line(&s, app.lang, c));
    if s.serious > 0 {
        println!(
            "{}",
            app.t(
                "Run 'macguard' in a terminal to open the menu, or 'macguard fix' to fix step by step.",
                "Chạy 'macguard' trong terminal để mở menu, hoặc 'macguard fix' để sửa từng bước."
            )
        );
        return 1;
    }
    0
}

pub fn explain(app: &App, ids: &[String]) -> Result<(), String> {
    for i in select(app, ids)? {
        let o = app.check_one(i);
        for l in ui::analysis(&app.defs[i], &o, app.lang, app.colors, 100) {
            println!("{l}");
        }
        println!();
    }
    Ok(())
}

pub fn fix(app: &App, ids: &[String]) -> Result<(), String> {
    let (mut fixed, mut seen) = (0, 0);
    let ignored = app.ignored();
    for i in select(app, ids)? {
        let d = &app.defs[i];
        if ids.is_empty() {
            // Without explicit ids, informational and ignored items are skipped.
            if d.severity == Severity::Info || ignored.contains(&d.id) {
                continue;
            }
        } else if ignored.contains(&d.id) {
            app.set_ignored(&d.id, false).map_err(|e| e.to_string())?;
        }
        println!();
        match fix_one(app, i) {
            FixResult::NothingToFix => {
                if !ids.is_empty() {
                    println!("{}: {}", d.id, app.t("nothing to fix", "không cần sửa"));
                }
            }
            FixResult::Fixed => {
                fixed += 1;
                seen += 1;
            }
            FixResult::NotFixed => seen += 1,
        }
    }
    println!();
    println!(
        "{}",
        match app.lang {
            Lang::En => format!("Fixed {fixed} of {seen} failing item(s)."),
            Lang::Vi => format!("Đã sửa {fixed} trên {seen} mục chưa đạt."),
        }
    );
    Ok(())
}

pub fn ignore(app: &App, ids: &[String], on: bool) -> Result<(), String> {
    if ids.is_empty() {
        return Err(app
            .t("Give at least one item id.", "Hãy nêu ít nhất một mã mục.")
            .into());
    }
    for i in select(app, ids)? {
        let id = &app.defs[i].id;
        app.set_ignored(id, on).map_err(|e| e.to_string())?;
        if on {
            println!("⊘ {id}: {}", app.t("ignored", "đã bỏ qua"));
        } else {
            println!("{id}: {}", app.t("checked again", "sẽ được kiểm lại"));
        }
    }
    Ok(())
}

pub fn list(app: &App) {
    let ignored = app.ignored();
    for sev in Severity::ALL {
        println!("{}", app.colors.bold(sev.heading(app.lang)));
        for d in app.defs.iter().filter(|d| d.severity == sev) {
            let flag = if ignored.contains(&d.id) { " ⊘" } else { "" };
            println!(
                "  {} {}{}",
                ui::pad(&d.id, ui::ID_WIDTH),
                d.title.get(app.lang),
                flag
            );
        }
    }
}

pub fn tips(app: &App) {
    let c = app.colors;
    println!(
        "{}",
        app.t(
            "Guide recommendations that macguard cannot check automatically",
            "Khuyến nghị trong tài liệu mà macguard không tự kiểm được"
        )
    );
    for s in data::tips() {
        println!("\n{}", c.bold(s.title.get(app.lang)));
        for tip in &s.tips {
            for (n, l) in ui::wrap(tip.get(app.lang), 94).iter().enumerate() {
                println!("{}{l}", if n == 0 { "  • " } else { "    " });
            }
        }
    }
    println!(
        "\n{} https://github.com/drduh/macOS-Security-and-Privacy-Guide",
        app.t("Full guide:", "Tài liệu đầy đủ:")
    );
}

pub fn usage(lang: Lang) -> String {
    let v = env!("CARGO_PKG_VERSION");
    match lang {
        Lang::Vi => format!(
            "macguard {v}: kiểm tra và gia cố bảo mật macOS theo drduh/macOS-Security-and-Privacy-Guide

Cách dùng:
  macguard                    Mở menu tương tác (trong terminal); thoát bằng q hoặc Ctrl-C
  macguard check              Chấm điểm máy (chỉ đọc, không thay đổi gì)
  macguard explain [mã...]    Phân tích từng mục: tác dụng, lệnh, rủi ro khi sửa và khi không sửa
  macguard fix [mã...]        Sửa các mục chưa đạt, phân tích và hỏi trước từng mục
  macguard ignore <mã...>     Bỏ qua mục, lần sau không kiểm nữa
  macguard unignore <mã...>   Kiểm lại mục đã bỏ qua
  macguard list               Liệt kê mã các mục theo mức (⊘ = đang bỏ qua)
  macguard tips               Khuyến nghị trong tài liệu không tự kiểm được
  macguard help | version

Tuỳ chọn:
  --lang en|vi                Chọn ngôn ngữ (hoặc biến MACGUARD_LANG)

Chạy 'sudo macguard' để kiểm cả các mục cần quyền root.
Ký hiệu: ✓ đạt · ✗ đỏ mức cao · ✗ vàng mức vừa · ! mức thấp · i tham khảo · - không áp dụng · ⊘ bỏ qua
Mã thoát của check: 0 khi không còn mục cao/vừa nào chưa đạt, 1 khi còn."
        ),
        Lang::En => format!(
            "macguard {v}: audit and harden macOS following drduh/macOS-Security-and-Privacy-Guide

Usage:
  macguard                    Open the interactive menu (in a terminal); quit with q or Ctrl-C
  macguard check              Score this Mac (read-only, changes nothing)
  macguard explain [id...]    Analyse each item: effect, command, risk of fixing and of not fixing
  macguard fix [id...]        Fix failing items, explaining and asking before each one
  macguard ignore <id...>     Ignore items so they are no longer checked
  macguard unignore <id...>   Check ignored items again
  macguard list               List item ids by severity (⊘ = ignored)
  macguard tips               Guide recommendations that cannot be checked automatically
  macguard help | version

Options:
  --lang en|vi                Output language (or the MACGUARD_LANG variable)

Run 'sudo macguard' to include items that need root.
Marks: ✓ pass · red ✗ high · yellow ✗ medium · ! low · i info · - not applicable · ⊘ ignored
Exit code of check: 0 when no high/medium item fails, 1 otherwise."
        ),
    }
}

/// Markdown reference of every check, kept in docs/ and verified by tests.
pub fn docs_markdown(defs: &[Def], lang: Lang) -> String {
    let t = |en, vi| lang.t(en, vi);
    let mut s = String::new();
    s.push_str(t(
        "# macguard checks\n\n",
        "# Các mục kiểm của macguard\n\n",
    ));
    s.push_str(t(
        "Generated by `cargo run -- gen-docs docs` from `data/checks.toml`. Do not edit by hand.\n\n",
        "Được sinh bởi `cargo run -- gen-docs docs` từ `data/checks.toml`. Đừng sửa tay.\n\n",
    ));
    s.push_str(t(
        "Items are grouped by severity. **High** and **medium** items affect the exit code of `macguard check`; **low** items are recommended hardening; items **for review** are not scored.\n",
        "Các mục được nhóm theo mức độ. Mục **cao** và **vừa** ảnh hưởng mã thoát của `macguard check`; mục **thấp** là gia cố nên làm; mục **tham khảo** không tính điểm.\n",
    ));
    for sev in Severity::ALL {
        s.push_str(&format!("\n## {}\n\n", sev.heading(lang)));
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n|---|---|---|---|---|\n",
            t("Id", "Mã"),
            t("Check", "Mục kiểm"),
            t("Area", "Nhóm"),
            t("Fix", "Cách sửa"),
            t("Source", "Nguồn")
        ));
        for d in defs.iter().filter(|d| d.severity == sev) {
            s.push_str(&format!(
                "| [`{0}`](#{0}) | {1} | {2} | {3} | {4} |\n",
                d.id,
                d.title.get(lang),
                d.category.name(lang),
                if d.fix.is_some() {
                    t("automatic, asks first", "tự động, hỏi trước")
                } else {
                    t("manual", "làm tay")
                },
                if d.extra {
                    t("added by macguard", "macguard thêm")
                } else {
                    t("guide", "tài liệu")
                },
            ));
        }
    }
    s.push_str(&format!("\n## {}\n", t("Details", "Chi tiết")));
    for sev in Severity::ALL {
        for d in defs.iter().filter(|d| d.severity == sev) {
            s.push_str(&format!(
                "\n<a id=\"{0}\"></a>\n\n### `{0}`: {1}\n\n",
                d.id,
                d.title.get(lang)
            ));
            s.push_str(&format!(
                "- **{}:** {}\n",
                t("Severity", "Mức"),
                d.severity.name(lang)
            ));
            s.push_str(&format!(
                "- **{}:** {}\n",
                t("Area", "Nhóm"),
                d.category.name(lang)
            ));
            s.push_str(&format!(
                "- **{}:** {}\n",
                t("What it does", "Tác dụng"),
                d.why.get(lang)
            ));
            match d.fix_display() {
                Some(cmd) => {
                    s.push_str(&format!(
                        "- **{}:**\n\n```bash\n{cmd}\n```\n\n",
                        t("Command `macguard fix` runs", "Lệnh `macguard fix` sẽ chạy")
                    ));
                    s.push_str(&format!(
                        "- **{}:** {}\n",
                        t("If the automatic fix fails", "Nếu sửa tự động không được"),
                        d.how.get(lang)
                    ));
                }
                None => s.push_str(&format!(
                    "- **{}:** {}\n",
                    t("How to fix (manual)", "Cách sửa (làm tay)"),
                    d.how.get(lang)
                )),
            }
            s.push_str(&format!(
                "- **{}:** {}\n",
                t("Risk if you do not fix it", "Rủi ro nếu không sửa"),
                d.skip.get(lang)
            ));
            s.push_str(&format!(
                "- **{}:** {}\n",
                t("Risk when you fix it", "Rủi ro khi sửa"),
                d.risk.get(lang)
            ));
            s.push_str(&format!(
                "- **{}:** {}\n",
                t("Source", "Nguồn"),
                d.source(lang)
            ));
        }
    }
    s
}
