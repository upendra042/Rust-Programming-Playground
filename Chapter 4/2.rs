fn chk_div(n: i32) -> i32 {
    if n % 3 == 0 && n % 4 == 0 {
        0
    } else if n % 3 == 0 {
        1
    } else if n % 4 == 0 {
        2
    } else {
        -1
    }
}
