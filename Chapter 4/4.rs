fn calc_ar_pr(x: f64, y: f64) -> (f64, f64) {
    let area = x * y;
    let peri = 2.0 * (x + y);
    (area, peri)
}

fn main() {
	let a=10.0;
    let p=8.0;
    println!("{:?}",calc_ar_pr(a,p));
    
}