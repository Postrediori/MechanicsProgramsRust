mod bessel_func;

fn print_table(n: i64, x1: f64, x2: f64) {
    println!("       X -   Integr -   Series -    Delta");

    let dx = (x2 - x1) / (n as f64);
    for i in 1..n+1 {
        let x = x1 + dx * (i as f64);

        let y1 = bessel_func::y0_1(x);
        let y2 = bessel_func::y0_2(x);

        println!("{:.6} - {:.6} - {:.6} - {:.6}", x, y1, y2, (y2 - y1).abs());
    }
}

fn main() {
    print_table(20, 0.0, 5.0);
}
