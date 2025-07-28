fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = 7;
    println!("Fibonacci({}) = {}", n, fib(n));
}
