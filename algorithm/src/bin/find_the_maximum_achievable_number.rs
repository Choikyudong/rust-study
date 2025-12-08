fn main() {
    // 6
    println!("{}", the_maximum_achievable_x(4, 1));

    // 7
    println!("{}", the_maximum_achievable_x(3, 2));
}

fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
    num + (t * 2)
}