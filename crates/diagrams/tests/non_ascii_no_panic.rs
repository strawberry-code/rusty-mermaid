//! Un'istruzione che inizia con un carattere non ASCII (accento, emoji) non
//! deve mandare in panic nessun parser: lo slicing per byte cadeva dentro
//! il carattere.

use rusty_mermaid_diagrams::{class, er, flowchart, gantt, sequence, state};

const RIGHE: [&str; 8] = [
    "è così",
    "é",
    "\u{1F600}",
    "A --> B\n  è",
    ":é ",
    "\u{1F600} --> \u{1F600}",
    "è : è",
    "A \"è\" --> \u{1F600} : \u{1F600}",
];

fn senza_panic(intestazione: &str, prefisso: &str, parse: impl Fn(&str) -> bool) {
    for riga in RIGHE {
        for src in [
            format!("{intestazione}\n{prefisso}{riga}\n"),
            format!("{intestazione}\n  {riga}\n  {riga}"),
            format!("{intestazione}\n{riga}"),
        ] {
            let _ = parse(&src);
        }
    }
}

#[test]
fn sequence() {
    senza_panic("sequenceDiagram", "  ", |s| {
        sequence::parser::parse(s).is_ok()
    });
}

#[test]
fn class() {
    senza_panic("classDiagram", "  ", |s| class::parser::parse(s).is_ok());
}

#[test]
fn state() {
    senza_panic("stateDiagram-v2", "  ", |s| state::parser::parse(s).is_ok());
}

#[test]
fn er() {
    senza_panic("erDiagram", "  ", |s| er::parser::parse(s).is_ok());
}

#[test]
fn gantt() {
    senza_panic("gantt\n  dateFormat YYYY-MM-DD", "", |s| {
        gantt::parser::parse(s).is_ok()
    });
}

#[test]
fn flowchart() {
    senza_panic("flowchart TD", "  ", |s| {
        flowchart::parser::parse(s).is_ok()
    });
}

/// Inserisce un carattere multibyte in ogni posizione dei sorgenti d'esempio
/// delle sei famiglie: nessun parser deve andare in panic.
#[test]
fn inserimento_in_ogni_posizione() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/golden/mmd");
    let mut visti = 0;
    let mut file = Vec::new();
    for sotto in std::fs::read_dir(dir).unwrap().flatten() {
        if let Ok(d) = std::fs::read_dir(sotto.path()) {
            file.extend(d.flatten().map(|v| v.path()));
        }
    }
    for voce in file {
        let Ok(src) = std::fs::read_to_string(voce) else {
            continue;
        };
        let famiglia = src
            .lines()
            .map(str::trim)
            .find(|l| !l.is_empty() && !l.starts_with("%%"))
            .unwrap_or("");
        let parse: fn(&str) -> bool = if famiglia.starts_with("sequenceDiagram") {
            |s| sequence::parser::parse(s).is_ok()
        } else if famiglia.starts_with("classDiagram") {
            |s| class::parser::parse(s).is_ok()
        } else if famiglia.starts_with("stateDiagram") {
            |s| state::parser::parse(s).is_ok()
        } else if famiglia.starts_with("erDiagram") {
            |s| er::parser::parse(s).is_ok()
        } else if famiglia.starts_with("gantt") {
            |s| gantt::parser::parse(s).is_ok()
        } else if famiglia.starts_with("flowchart") || famiglia.starts_with("graph") {
            |s| flowchart::parser::parse(s).is_ok()
        } else {
            continue;
        };
        visti += 1;
        for (i, _) in src.char_indices() {
            for c in ["è", "\u{1F600}"] {
                let mut mutato = src.clone();
                mutato.insert_str(i, c);
                let _ = parse(&mutato);
            }
        }
    }
    assert!(visti > 10, "pochi sorgenti visti: {visti}");
}

#[test]
fn la_direzione_con_suffisso_e_rifiutata() {
    assert!(flowchart::parser::parse("flowchart TDX\n A-->B").is_err());
    assert!(flowchart::parser::parse("flowchart TD\n A-->B").is_ok());
    assert!(flowchart::parser::parse("graph LR\n A-->B").is_ok());
}
