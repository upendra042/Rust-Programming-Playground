fn val_pass(mut x: i32) {
    x += 1;
    println!("Inside val_pass: {}", x);
}

fn ref_pass(x: &mut i32) {
    *x += 1;
    println!("Inside ref_pass: {}", x);
}

fn main() {
    let mut a = 9;
    ref_pass(&mut a);  // Pass by reference

    val_pass(5);       // Pass by value
}
