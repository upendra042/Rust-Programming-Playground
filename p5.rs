fn main() {
    let x: i32 = 100;
    let y: f64 = 45.6;
    let name: &str = "Rust";

    println!("Explicit types -> x: {}, y: {}, name: {}", x, y, name);

    let a = 200;
    let b = 3.14;
    let is_active = true;
    let lang = "Rustacean";

    println!("Implicit types -> a: {}, b: {}, is_active: {}, lang: {}", a, b, is_active, lang);
}
