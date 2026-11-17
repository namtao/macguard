//! Consistency tests over the embedded data, the generated docs and the repository.

use crate::checks::IDS;
use crate::commands::docs_markdown;
use crate::data::{defs, tips, Lang};
use std::collections::BTreeSet;
use std::path::Path;

#[test]
fn every_definition_has_check_logic_and_back() {
    let defined: BTreeSet<String> = defs().into_iter().map(|d| d.id).collect();
    let implemented: BTreeSet<String> = IDS.iter().map(|s| s.to_string()).collect();
    assert_eq!(defined, implemented);
    assert_eq!(defs().len(), 44);
}

#[test]
fn every_text_exists_in_both_languages() {
    for d in defs() {
        for (name, text) in [
            ("title", &d.title),
            ("why", &d.why),
            ("skip", &d.skip),
            ("risk", &d.risk),
            ("how", &d.how),
        ] {
            assert!(!text.en.trim().is_empty(), "{}: {name}.en is empty", d.id);
            assert!(!text.vi.trim().is_empty(), "{}: {name}.vi is empty", d.id);
        }
        if let Some(fix) = &d.fix {
            assert!(!fix.trim().is_empty(), "{}: empty fix", d.id);
        }
    }
    for s in tips() {
        assert!(!s.title.en.is_empty() && !s.title.vi.is_empty());
        assert!(s.tips.iter().all(|t| !t.en.is_empty() && !t.vi.is_empty()));
    }
}

#[test]
fn ids_are_unique() {
    let ids: Vec<String> = defs().into_iter().map(|d| d.id).collect();
    let unique: BTreeSet<&String> = ids.iter().collect();
    assert_eq!(ids.len(), unique.len());
}

#[test]
fn generated_docs_are_up_to_date() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for (lang, name) in [(Lang::En, "checks.md"), (Lang::Vi, "checks.vi.md")] {
        let committed = std::fs::read_to_string(root.join("docs").join(name)).unwrap_or_default();
        assert!(
            committed == docs_markdown(&defs(), lang),
            "docs/{name} is out of date: run `cargo run -- gen-docs docs`"
        );
    }
}

/// The project is open source: no personal paths or e-mail addresses in tracked files.
#[test]
fn no_personal_data_in_repository() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut bad = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for e in std::fs::read_dir(&dir).unwrap().flatten() {
            let p = e.path();
            let name = e.file_name().to_string_lossy().into_owned();
            if matches!(name.as_str(), ".git" | "target" | "tests.rs" | "Cargo.lock") {
                continue;
            }
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&p) else {
                continue;
            };
            for word in text.split(|c: char| c.is_whitespace() || "\"'`()<>,;".contains(c)) {
                let email = word.contains('@')
                    && word
                        .split('@')
                        .nth(1)
                        .is_some_and(|d| d.contains('.') && !d.starts_with('.'))
                    && word.split('@').next().is_some_and(|u| !u.is_empty());
                let home = word.contains("/Users/")
                    && !word.contains("/Users/{")
                    && !word.ends_with("/Users/");
                if email || home {
                    bad.push(format!("{}: {word}", p.display()));
                }
            }
        }
    }
    assert!(bad.is_empty(), "personal data found: {bad:?}");
}
