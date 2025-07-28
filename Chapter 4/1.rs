fn fact(n: u32) -> u32 {
    if n<=1 {
        return n;
    }
    else {
        return n*fact(n-1);
    }
}

fn main() {
	println!("{}",fact(5));

}


	