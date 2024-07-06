enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 15,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 둘 중 하나라도 없으면 컴파일러가 거부함
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => remove_function(),
        7 => add_function(),
        other => move_function(other),
    }
}

fn add_function() { /* 코드 */ }
fn remove_function() { /* 코드 */ }
fn move_function(other: u8) { /* 코드 */ }