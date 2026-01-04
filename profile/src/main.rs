// use profile::kinds::PrimaryColor;
use profile::PrimaryColor;

fn main() {
    let result = add(1, 2);
    println!("{}", result);

    let red = PrimaryColor::Red;
    let blue = PrimaryColor::Blue;
}

/// Adds Number <br/>
/// 두 개의 숫자를 더하고 반환합니다!
/// # Examples
/// ```
/// let a = 1;
/// let b = 2;
/// let result = add(a, b);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
