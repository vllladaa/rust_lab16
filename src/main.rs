fn find_solution() -> Option<(u8, u8, u8, u8, u8, u8, u8, u8)> {
    for m in 1..=8 {
        for u in 1..=8 {
            if u == m { continue; }
            for x in 1..=8 {
                if x == m || x == u { continue; }
                for a in 1..=8 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=8 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 1..=8 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 1..=8 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 1..=8 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    let muxa = m as u32 * 1000 + u as u32 * 100 + x as u32 * 10 + a as u32;
                                    let a_only = a as u32;
                                    let slon = s as u32 * 1000 + l as u32 * 100 + o as u32 * 10 + n as u32;

                                    if muxa * a_only == slon {
                                        return Some((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    match find_solution() {
        Some((m, u, x, a, s, l, o, n)) => {
            println!("Розв'язок знайдено:");
            println!(" m: {}, u: {}, x: {}, a: {}", m, u, x, a);
            println!(" s: {}, l: {}, o: {}, n: {}", s, l, o, n);
            println!();
            println!("  muxa");
            println!("x    a");
            println!("------");
            println!("  slon");
            println!();
            println!("{}{}{}{}", m, u, x, a);
            println!("x     {}", a);
            println!("------");
            println!("{}{}{}{}", s, l, o, n);
        }
        None => {
            println!("Розв'язку не знайдено.");
        }
    }
}
