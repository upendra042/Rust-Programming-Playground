fn main() {
    let x: i32 = 10;
    let y: f64 = 20.5;

    println!("After declaration: x = {} y = {}", x, y);

    {
        let x = 100;
        let y: f64 = 3.14;

        println!("Inside inner scope: x = {} y = {}", x, y);
    }

    println!("Outside inner scope: x = {} y = {}", x, y);

    let x = "I am now a string";

    println!("After shadowing with different type: x = {}", x);

    let z = true;
    let a = 42;
    let b = 3.14;

    println!("Implicit types -> z: {}, a: {}, b: {}", z, a, b);
}
