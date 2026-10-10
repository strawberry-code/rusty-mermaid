//! Un flowchart con molti incroci si dispone in tempo ragionevole.
//!
//! Il simplex di rete dei ranghi, dopo pochi scambi utili, ciclava su scambi
//! degeneri fino a `n^2` (150 nodi: 22801 scambi, 2,3 s in release). Ora si
//! ferma dopo `MAX_STALLED_PIVOTS` scambi senza miglioramento.

use std::time::Instant;

use rusty_mermaid_core::Theme;

/// Strati da 10, due archi per nodo verso lo strato dopo, ogni tanto un arco
/// all'indietro (la forma del corpus `flow-grande`).
fn flowchart(n: u64) -> String {
    let mut x: u64 = 5;
    let mut next = move |m: u64| {
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (x >> 33) % m
    };
    let mut s = String::from("flowchart TD\n");
    for i in 0..n {
        s.push_str(&format!("  N{i}[Nodo {i}]\n"));
    }
    for i in 0..n {
        for _ in 0..2 {
            let to = (i / 10 + 1) * 10 + next(20);
            if to < n {
                s.push_str(&format!("  N{i} --> N{to}\n"));
            }
        }
        if i > 10 && next(10) == 0 {
            s.push_str(&format!("  N{i} -.-> N{}\n", next(i - 10)));
        }
    }
    s
}

#[test]
fn flowchart_150_nodi_incrociati_non_cicla() {
    let src = flowchart(150);
    let t = Instant::now();
    rusty_mermaid_diagrams::render_to_scene(&src, &Theme::default()).expect("render");
    // Prima della correzione: 47 s in debug. Ora una frazione.
    assert!(t.elapsed().as_secs() < 8, "{:?}", t.elapsed());
}
