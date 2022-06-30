use num_complex::Complex;

#[allow(dead_code)]
pub fn solve_quadratic_by_real(_a: i64, _b: i64, _c: i64) -> (f64, f64) {
    /* Does not solve all quadratic equations. Can you complete it to handle e.g. negative inputs? */
    let d: i64 = (_b.pow(2)) - 4 * _a * _c;
    println!("Discriminant is {}", d);
    let mut s1: f64 = 1.0f64;
    let mut s2: f64 = 1.0f64;
    if d >= 0 {
        println!("2 solutions or 1 solutions.");
        s1 = (- (_b as f64) + (d as f64).sqrt()) / ((2 * _a) as f64);
        s2 = (- (_b as f64) - (d as f64).sqrt()) / ((2 * _a) as f64);
    } else if d < 0 {
        panic!("2 complex solutions.");
    }
    // https://docs.replit.com/tutorials/introduction-to-the-repl-it-ide
    return (s1, s2);
}

#[allow(dead_code)]
pub fn solve_quadratic_by_imaginary(_a: i64, _b: i64, _c: i64) -> (Complex<f64>, Complex<f64>) {
    /* Does not solve all quadratic equations. Can you complete it to handle e.g. negative inputs? */
    let mut d: i64 = (_b.pow(2)) - 4 * _a * _c;
    println!("Discriminant is {}", d);
    let mut s1: Complex<f64> = Complex::new(1.0f64, 1.0f64);
    let mut s2: Complex<f64> = Complex::new(1.0f64, 1.0f64);
    if d >= 0 {
        panic!("2 solutions or 1 solutions.");
    } else if d < 0 {
        println!("2 complex solutions.");
        d = d.abs();
        let re: f64 = - (_b as f64) / ((2 * _a) as f64);
        let im: f64 = (d as f64).sqrt() / ((2 * _a) as f64);
        s1 = Complex::new(re, im);
        s2 = Complex::new(re, -im);
        println!("{} + {}i", s1.re, s1.im);
        println!("{} + {}i", s2.re, s2.im);
    }
    // https://docs.replit.com/tutorials/introduction-to-the-repl-it-ide
    return (s1, s2);
}

#[allow(dead_code)]
pub fn solve_sum(_a: i64, _b: i64, _c: i64) -> i64 {
    /* Does not solve all quadratic equations. Can you complete it to handle e.g. negative inputs? */
    return _a + _b + _c;
}
