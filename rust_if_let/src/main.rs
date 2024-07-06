fn main() {
    let config = Some(3u8);
    match config {
        Some(max) => println!("config : {}", max),
        _ => (), // match 표현식을 위한 보일러 플레이트 코드이다.
    }

    // 위의 match문을 이렇게 표현할 수 있다.
    let config2 = Some(3u8);
    if let Some(max) = config2 {
        println!("config : {}", max);
    }

    // 만약 위의 match에서 other도 필요하다면
    let config3 = Some(3u8);
    if let Some(max) = config3 {
        println!("config : {}", max);
    } else {
        /* 코드 */
    }
}
