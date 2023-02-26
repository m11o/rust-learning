fn main() {
    let args = std::env::args();
    let mut total = 0.0;

    for (i, s) in args.enumerate() {
        if i == 0 { continue; } // 一番最初はコマンド自身

        let num: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += num;
    }

    println!("{}", total);
}