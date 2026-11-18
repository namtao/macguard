//! Interactive menu, opened by `macguard` in a terminal. Items are grouped by
//! severity (high first, failing items first in each group). Enter opens the
//! analysis, f fixes, i ignores. Quits only on q or Ctrl-C.

use crate::checks::{self, Outcome, Status};
use crate::commands::{self, App, FixResult};
use crate::data::{Lang, Severity};
use crate::ui;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::io::{BufRead, Write};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

enum View {
    List,
    Detail { item: usize, scroll: usize },
}

struct Menu<'a> {
    app: &'a App,
    header: String,
    results: Vec<Option<Outcome>>,
    rx: Option<Receiver<(usize, Outcome)>>,
    /// Definition index of the selected item.
    sel: usize,
    top: usize,
    only_fail: bool,
    msg: String,
    view: View,
}

enum Key {
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Open,
    Back,
    Char(char),
    Quit,
    Other,
}

fn key(k: KeyEvent) -> Key {
    if k.modifiers.contains(KeyModifiers::CONTROL) && k.code == KeyCode::Char('c') {
        return Key::Quit;
    }
    match k.code {
        KeyCode::Up | KeyCode::Char('k') => Key::Up,
        KeyCode::Down | KeyCode::Char('j') => Key::Down,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown | KeyCode::Char(' ') => Key::PageDown,
        KeyCode::Home | KeyCode::Char('g') => Key::Home,
        KeyCode::End | KeyCode::Char('G') => Key::End,
        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => Key::Open,
        KeyCode::Left | KeyCode::Esc | KeyCode::Char('h') | KeyCode::Char('b') => Key::Back,
        KeyCode::Char('q') => Key::Quit,
        KeyCode::Char(c) => Key::Char(c),
        _ => Key::Other,
    }
}

fn enter_screen() {
    let _ = terminal::enable_raw_mode();
    let _ = execute!(std::io::stdout(), EnterAlternateScreen, Hide);
}

fn leave_screen() {
    let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
    let _ = terminal::disable_raw_mode();
}

impl<'a> Menu<'a> {
    fn new(app: &'a App) -> Self {
        let mut m = Menu {
            app,
            header: app.header(),
            results: vec![None; app.defs.len()],
            rx: None,
            sel: 0,
            top: 0,
            only_fail: false,
            msg: String::new(),
            view: View::List,
        };
        m.rescan();
        m
    }

    fn lang(&self) -> Lang {
        self.app.lang
    }

    fn rescan(&mut self) {
        self.results = vec![None; self.app.defs.len()];
        let (tx, rx) = mpsc::channel();
        let defs = self.app.defs.clone();
        let ctx = self.app.ctx.clone();
        let ignored = self.app.ignored();
        std::thread::spawn(move || checks::scan(&defs, &ignored, &ctx, tx));
        self.rx = Some(rx);
    }

    /// Collects finished checks. Returns true when something changed.
    fn drain(&mut self) -> bool {
        let Some(rx) = &self.rx else { return false };
        let mut changed = false;
        for (i, o) in rx.try_iter() {
            // An item ignored or fixed during the scan already has a newer result.
            if self.results[i].is_none() {
                self.results[i] = Some(o);
                changed = true;
            }
        }
        if self.results.iter().all(Option::is_some) {
            self.rx = None;
        }
        changed
    }

    fn scanning(&self) -> Option<usize> {
        self.rx
            .as_ref()
            .map(|_| self.results.iter().filter(|r| r.is_some()).count())
    }

    /// Items shown in the list, in display order.
    fn visible(&self) -> Vec<usize> {
        ui::order(&self.app.defs, &self.results)
            .into_iter()
            .filter(|&i| match &self.results[i] {
                None => !self.only_fail,
                Some(o) if o.hidden() => false,
                // Ignored items stay visible in the filter so they can be un-ignored.
                Some(o) => !self.only_fail || matches!(o.status, Status::Fail | Status::Ignored),
            })
            .collect()
    }

    fn outcome(&mut self, i: usize) -> Outcome {
        if self.results[i].is_none() {
            self.results[i] = Some(self.app.check_one(i));
        }
        self.results[i].clone().unwrap()
    }

    fn toggle_ignore(&mut self, i: usize) {
        let id = self.app.defs[i].id.clone();
        let now = !self.app.ignored().contains(&id);
        if let Err(e) = self.app.set_ignored(&id, now) {
            self.msg = e.to_string();
            return;
        }
        self.msg = match (now, self.lang()) {
            (true, Lang::En) => format!("⊘ {id} ignored; it will not be checked next time."),
            (true, Lang::Vi) => format!("⊘ Đã bỏ qua {id}; lần sau không kiểm nữa."),
            (false, Lang::En) => format!("{id} will be checked again."),
            (false, Lang::Vi) => format!("{id} sẽ được kiểm lại."),
        };
        self.results[i] = Some(self.app.check_one(i));
    }

    fn fix(&mut self, i: usize) {
        let o = self.outcome(i);
        let id = &self.app.defs[i].id;
        let lang = self.lang();
        match o.status {
            Status::Fail => {}
            Status::Ignored => {
                self.msg = match lang {
                    Lang::En => format!("{id} is ignored. Press i to check it again first."),
                    Lang::Vi => format!("{id} đang bị bỏ qua. Bấm i để kiểm lại trước."),
                };
                return;
            }
            _ => {
                self.msg = format!("{id}: {}", lang.t("nothing to fix.", "không cần sửa."));
                return;
            }
        }
        leave_screen();
        print!("\x1b[2J\x1b[H");
        let res = commands::fix_one(self.app, i);
        print!(
            "\n{}",
            self.app
                .colors
                .dim(lang.t("Press Enter to go back.", "Nhấn Enter để quay lại."))
        );
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().lock().read_line(&mut String::new());
        enter_screen();
        self.results[i] = Some(self.app.check_one(i));
        self.msg = match (res, lang) {
            (FixResult::Fixed, Lang::En) => self.app.colors.green(&format!("✓ {id} fixed.")),
            (FixResult::Fixed, Lang::Vi) => self.app.colors.green(&format!("✓ Đã sửa {id}.")),
            (_, Lang::En) => format!("{id} not changed."),
            (_, Lang::Vi) => format!("{id} chưa thay đổi."),
        };
    }

    fn help_list(&self) -> &'static str {
        self.lang().t(
            "↑↓ move · Enter analyse · f fix · i ignore/unignore · a failing only/all · r rescan · q quit",
            "↑↓ chọn · Enter phân tích · f sửa · i bỏ qua/bỏ bỏ qua · a chỉ mục chưa đạt/tất cả · r quét lại · q thoát",
        )
    }

    fn help_detail(&self) -> &'static str {
        self.lang().t(
            "↑↓ scroll · f fix · i ignore/unignore · o open System Settings · ← or Esc back · q quit",
            "↑↓ cuộn · f sửa · i bỏ qua/bỏ bỏ qua · o mở Cài đặt hệ thống · ← hoặc Esc quay lại · q thoát",
        )
    }

    fn render(&mut self) -> std::io::Result<()> {
        // Some terminals (and pseudo-terminals) report 0x0; fall back to 80x24.
        let (cols, rows) = match terminal::size() {
            Ok((c, r)) if c > 0 && r > 0 => (c as usize, r as usize),
            _ => (80, 24),
        };
        let c = self.app.colors;
        let mut out: Vec<String> = Vec::new();
        let rule = c.dim(&"─".repeat(cols));

        let mut footer = vec![rule.clone()];
        if !self.msg.is_empty() {
            footer.push(self.msg.clone());
        }
        let help = match self.view {
            View::List => self.help_list(),
            View::Detail { .. } => self.help_detail(),
        };
        footer.extend(ui::wrap(help, cols).into_iter().map(|l| c.dim(&l)));

        match self.view {
            View::List => {
                let mut status =
                    ui::score_line(&ui::score(&self.app.defs, &self.results), self.lang(), c);
                if let Some(n) = self.scanning() {
                    status.push_str(&c.dim(&format!(
                        " · {} {n}/{}…",
                        self.lang().t("checking", "đang kiểm"),
                        self.app.defs.len()
                    )));
                }
                if self.only_fail {
                    status.push_str(&format!(
                        " · {}",
                        self.lang().t("failing items only", "chỉ mục chưa đạt")
                    ));
                }
                out.push(self.header.clone());
                out.push(status);
                out.push(rule.clone());
                let body = rows.saturating_sub(out.len() + footer.len()).max(3);
                out.extend(self.list_lines(cols, body));
            }
            View::Detail { item, scroll } => {
                let o = self.outcome(item);
                let lines = ui::analysis(
                    &self.app.defs[item],
                    &o,
                    self.lang(),
                    c,
                    cols.saturating_sub(2).min(110),
                );
                let body = rows.saturating_sub(footer.len()).max(3);
                let max_scroll = lines.len().saturating_sub(body);
                let scroll = scroll.min(max_scroll);
                self.view = View::Detail { item, scroll };
                out.extend(lines.into_iter().skip(scroll).take(body));
                while out.len() < body {
                    out.push(String::new());
                }
            }
        }
        out.extend(footer);

        let mut stdout = std::io::stdout().lock();
        queue!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
        let n = out.len().min(rows);
        for (k, line) in out.iter().take(n).enumerate() {
            write!(stdout, "{line}")?;
            if k + 1 < n {
                write!(stdout, "\r\n")?;
            }
        }
        stdout.flush()
    }

    /// The scrolled list body: severity headings and wrapped item lines.
    fn list_lines(&mut self, cols: usize, body: usize) -> Vec<String> {
        let c = self.app.colors;
        let lang = self.lang();
        let vis = self.visible();
        if vis.is_empty() {
            return vec![format!(
                "   {}",
                lang.t("Nothing to show.", "Không có mục nào.")
            )];
        }
        if !vis.contains(&self.sel) {
            self.sel = vis[0];
        }
        let text_w = cols.saturating_sub(ui::ID_WIDTH + 7).max(20);
        // (line, item or None for a heading)
        let mut lines: Vec<(String, Option<usize>)> = Vec::new();
        let mut last: Option<Severity> = None;
        for &i in &vis {
            let d = &self.app.defs[i];
            if last != Some(d.severity) {
                let failing = vis
                    .iter()
                    .filter(|&&j| self.app.defs[j].severity == d.severity)
                    .filter(|&&j| matches!(&self.results[j], Some(o) if o.status == Status::Fail))
                    .count();
                let mut h = d.severity.heading(lang).to_string();
                if failing > 0 && d.severity != Severity::Info {
                    h.push_str(&match lang {
                        Lang::En => format!(" · {failing} failing"),
                        Lang::Vi => format!(" · {failing} chưa đạt"),
                    });
                }
                let h = match d.severity {
                    Severity::High => c.red(&h),
                    Severity::Med => c.yellow(&h),
                    _ => h,
                };
                lines.push((format!(" {}", c.bold(&h)), None));
                last = Some(d.severity);
            }
            let (status, detail) = match &self.results[i] {
                Some(o) => (Some(o.status), o.detail.as_str()),
                None => (None, ""),
            };
            let text = if detail.is_empty() {
                d.title.get(lang).to_string()
            } else {
                format!("{} ({detail})", d.title.get(lang))
            };
            let selected = i == self.sel;
            for (k, seg) in ui::wrap(&text, text_w).into_iter().enumerate() {
                let line = match (k, selected) {
                    (0, true) => format!(
                        " {} {} {}",
                        c.bold("▸"),
                        ui::mark(status, d.severity, c),
                        c.reverse(&format!(
                            "{} {}",
                            ui::pad(&d.id, ui::ID_WIDTH),
                            ui::pad(&seg, text_w)
                        ))
                    ),
                    (0, false) => format!(
                        "   {} {} {seg}",
                        ui::mark(status, d.severity, c),
                        c.dim(&ui::pad(&d.id, ui::ID_WIDTH))
                    ),
                    (_, true) => format!(
                        "{}{}",
                        " ".repeat(ui::ID_WIDTH + 6),
                        c.reverse(&ui::pad(&seg, text_w))
                    ),
                    (_, false) => format!("{}{seg}", " ".repeat(ui::ID_WIDTH + 6)),
                };
                lines.push((line, Some(i)));
            }
        }
        // Keep the whole selected item visible, with its heading when possible.
        let first = lines
            .iter()
            .position(|(_, it)| *it == Some(self.sel))
            .unwrap_or(0);
        let last_line = lines
            .iter()
            .rposition(|(_, it)| *it == Some(self.sel))
            .unwrap_or(first);
        if first < self.top {
            self.top = first;
        }
        if self.top > 0 && first == self.top && lines[first - 1].1.is_none() {
            self.top = first - 1;
        }
        if last_line >= self.top + body {
            self.top = last_line + 1 - body;
        }
        let mut shown: Vec<String> = lines
            .into_iter()
            .skip(self.top)
            .take(body)
            .map(|(l, _)| l)
            .collect();
        while shown.len() < body {
            shown.push(String::new());
        }
        shown
    }

    fn move_sel(&mut self, delta: isize) {
        let vis = self.visible();
        if vis.is_empty() {
            return;
        }
        let pos = vis.iter().position(|&i| i == self.sel).unwrap_or(0) as isize;
        let new = (pos + delta).clamp(0, vis.len() as isize - 1) as usize;
        self.sel = vis[new];
    }

    /// Handles a key. Returns false to quit.
    fn handle(&mut self, k: Key) -> bool {
        self.msg.clear();
        match (&self.view, k) {
            (_, Key::Quit) => return false,
            (View::List, Key::Up) => self.move_sel(-1),
            (View::List, Key::Down) => self.move_sel(1),
            (View::List, Key::PageUp) => self.move_sel(-10),
            (View::List, Key::PageDown) => self.move_sel(10),
            (View::List, Key::Home) => self.move_sel(-1000),
            (View::List, Key::End) => self.move_sel(1000),
            (View::List, Key::Open) => {
                if !self.visible().is_empty() {
                    self.view = View::Detail {
                        item: self.sel,
                        scroll: 0,
                    };
                }
            }
            (View::List, Key::Char('f')) => {
                if !self.visible().is_empty() {
                    self.fix(self.sel)
                }
            }
            (View::List, Key::Char('i')) => {
                if !self.visible().is_empty() {
                    self.toggle_ignore(self.sel)
                }
            }
            (View::List, Key::Char('a')) => {
                self.only_fail = !self.only_fail;
                self.top = 0;
            }
            (View::List, Key::Char('r')) => {
                self.header = self.app.header();
                self.rescan();
            }
            (View::Detail { item, scroll }, k) => {
                let (item, scroll) = (*item, *scroll);
                match k {
                    Key::Up => {
                        self.view = View::Detail {
                            item,
                            scroll: scroll.saturating_sub(1),
                        }
                    }
                    Key::Down => {
                        self.view = View::Detail {
                            item,
                            scroll: scroll + 1,
                        }
                    }
                    Key::PageUp => {
                        self.view = View::Detail {
                            item,
                            scroll: scroll.saturating_sub(10),
                        }
                    }
                    Key::PageDown => {
                        self.view = View::Detail {
                            item,
                            scroll: scroll + 10,
                        }
                    }
                    Key::Back => self.view = View::List,
                    Key::Char('f') => self.fix(item),
                    Key::Char('i') => self.toggle_ignore(item),
                    Key::Char('o') if !self.app.open_settings(&self.app.defs[item]) => {
                        self.msg = self
                            .lang()
                            .t(
                                "No System Settings pane for this item.",
                                "Mục này không có trang Cài đặt hệ thống.",
                            )
                            .to_string();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        true
    }
}

pub fn run(app: &App) -> std::io::Result<()> {
    let mut menu = Menu::new(app);
    enter_screen();
    // Restore the terminal even if something panics.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        leave_screen();
        default_hook(info);
    }));
    let result = (|| -> std::io::Result<()> {
        let mut dirty = true;
        loop {
            if menu.drain() {
                dirty = true;
            }
            if dirty {
                menu.render()?;
                dirty = false;
            }
            let wait = if menu.scanning().is_some() { 30 } else { 500 };
            if event::poll(Duration::from_millis(wait))? {
                match event::read()? {
                    Event::Key(k) if k.kind != KeyEventKind::Release => {
                        if !menu.handle(key(k)) {
                            return Ok(());
                        }
                        dirty = true;
                    }
                    Event::Resize(..) => dirty = true,
                    _ => {}
                }
            }
        }
    })();
    leave_screen();
    result
}
