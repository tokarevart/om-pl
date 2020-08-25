use om_pl::*;

fn main() {
    // x^2+x*ln(2+x^2) = x*(x+ln(1+(1+x^2)))
    let f = |x: f64| x * (x + (1.0 + x * x).ln_1p());
    let range = -10.0..10.0;
    let ltz = 30.0;
    let eps = 1e-4;
    let x = search(range, ltz, eps, f);
    println!("x              : {}", x);
    println!("x^2+x*ln(2+x^2): {}", f(x));
    println!("");

    let f = |x: f64| x.sin() + (1.0 + x * x).ln_1p();
    let range = 2.0..14.0;
    let ltz = 1.3;
    let eps = 1e-4;
    let x = search(range, ltz, eps, f);
    println!("x               : {}", x);
    println!("sin(x)+ln(2+x^2): {}", f(x));
    println!("");

    let f = |x: f64| x * (5.0 * x * x - x - 4.0);
    let range = -7.0..7.0;
    let ltz = 750.0;
    let eps = 1e-4;
    let x = search(range, ltz, eps, f);
    println!("x          : {}", x);
    println!("5*x-x^2-4*x: {}", f(x));
    println!("");
}
