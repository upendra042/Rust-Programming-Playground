fn sq_arr(arr: &[i32]) -> Vec<i32> {
    arr.iter().map(|x| x * x).collect()
}

fn main() {
    let nums = [1, 2, 3, 4, 5];
    let squares = sq_arr(&nums);
    println!("Original: {:?}", nums);
    println!("Squares: {:?}", squares);
}
