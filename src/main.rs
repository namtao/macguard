//! macguard: audit and harden macOS following
//! https://github.com/drduh/macOS-Security-and-Privacy-Guide

mod checks;
mod commands;
mod data;
mod menu;
mod ui;
mod update;

#[cfg(test)]
mod tests;

use commands::App;
use data::Lang;
use std::io::IsTerminal;
use std::process::exit;
use ui::Colors;

fn main() {
    if !cfg!(target_os = "macos") {
        eprintln!("macguard only runs on macOS.");
        exit(2);
    }

    // Options may appear anywhere; everything else is the command and its ids.
    let mut lang_flag: Option<String> = None;
    let mut args: Vec<String> = Vec::new();
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        if a == "--lang" {
            match it.next() {
                Some(v) => lang_flag = Some(v),
                None => {
                    eprintln!("--lang needs en or vi");
                    exit(2);
                }
            }
        } else if let Some(v) = a.strip_prefix("--lang=") {
            lang_flag = Some(v.to_string());
        } else {
            args.push(a);
        }
    }

    let lang = Lang::detect(lang_flag.as_deref());
    let interactive = std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
    let cmd = args.first().cloned().unwrap_or_else(|| {
        if interactive {
            "menu".into()
        } else {
            "check".into()
        }
    });
    let rest: Vec<String> = args.iter().skip(1).cloned().collect();

    // Commands that need no system context.
    match cmd.as_str() {
        "help" | "-h" | "--help" => {
            println!("{}", commands::usage(lang));
            return;
        }
        "version" | "-v" | "--version" => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            return;
        }
        "update" => {
            if let Err(e) = update::run(lang) {
                eprintln!("{e}");
                exit(1);
            }
            return;
        }
        // Development helper: regenerate the check reference in docs/.
        "gen-docs" => {
            let dir = rest.first().map(String::as_str).unwrap_or("docs");
            let defs = data::defs();
            for (l, name) in [(Lang::En, "checks.md"), (Lang::Vi, "checks.vi.md")] {
                let path = std::path::Path::new(dir).join(name);
                if let Err(e) = std::fs::write(&path, commands::docs_markdown(&defs, l)) {
                    eprintln!("{}: {e}", path.display());
                    exit(1);
                }
            }
            return;
        }
        _ => {}
    }

    let colors = Colors {
        on: std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none(),
    };
    let app = App {
        defs: data::defs(),
        ctx: checks::Ctx::detect(lang),
        lang,
        colors,
    };

    let result = match cmd.as_str() {
        "menu" => {
            if !interactive {
                Err(lang
                    .t(
                        "The menu needs an interactive terminal.",
                        "Menu cần một terminal tương tác.",
                    )
                    .to_string())
            } else {
                menu::run(&app).map_err(|e| e.to_string())
            }
        }
        "check" => exit(commands::check(&app)),
        "explain" => commands::explain(&app, &rest),
        "fix" => commands::fix(&app, &rest),
        "ignore" => commands::ignore(&app, &rest, true),
        "unignore" => commands::ignore(&app, &rest, false),
        "list" => {
            commands::list(&app);
            Ok(())
        }
        "tips" => {
            commands::tips(&app);
            Ok(())
        }
        _ => {
            println!("{}", commands::usage(lang));
            exit(2);
        }
    };
    if let Err(e) = result {
        eprintln!("{}", colors.red(&e));
        exit(2);
    }
}
