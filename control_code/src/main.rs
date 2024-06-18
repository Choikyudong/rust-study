fn main() {
    if_example();
    loop_example();
    while_example();
    for_example();
}

fn if_example() {
    let num = 3;
    if num > 3 { // (괄호)를 사용하지 않는다.
        println!("참");
    } else {
        println!("거짓");
    }

    /*
    if num { num은 정수형 타입이기 때문에 에러
        println!("숫자가 있음");
    }
    */
    
    let num = 10;
    if num % 3 == 0 {
        println!("3으로 나누어 지나요?");
    } else if num % 2 == 0 {
        println!("2으로 나누어 지나요?");
    } else {
        println!("너는 소수인가?");
    }

    let num = 8;
    let if_state_let: char = if num == 8 { 'a' } else { 'b' };
    println!("삼항연산자 돌려줘요 ~ : {if_state_let}");

    let num = 5;
    let if_state_let: char = if num == 8 { 'a' } else if num == 5 { 'b' } else { 'c' };
    println!("삼항연산자 돌려줘요 ~ : {if_state_let}");

    /*
    let num = 8;
    let if_state_let = if num == 8 { 1 } else { 'b' };
    println!("삼항연산자 돌려줘요 ~ : {if_state_let}");
    */
}

fn loop_example() {
    /*
    loop {
        println!("무한루프~~");
    }
    */

    let mut count: u16 = 0;
    loop {
        println!("{count} 번째 작동중");
        count += 1;
        if count == 1000 {
            break;
        }
    }
    println!("loop 종료~");

    count = 0; // 재할당
    let loop_answer = loop {
        println!("{count} 번째 작동중");
        count += 1;
        if count == 1000 {
            break count; // 값을 꼭 넣어야 한다.
        }
    };
    println!("loop_answer 의 값은 ?? {loop_answer}");

    count = 0;
    'outer_loop: loop {
        println!("{count} 번째 작동중");
        let mut inner_count = 0;
        loop {
            println!("{count} 번째 작동중");
            count += 1;
            if count > 1000 {
                break 'outer_loop;
            }
            if inner_count == 10 {
                break;
            }
            inner_count += 1;
        }
        count *= count;
    }
}

fn while_example() {
    let mut count = 0;
    while 100 > count {
        println!("현재 count 는 {count} 이다");
        count += 1;
    }

    let array: [isize; 5] = [1, 6, 2, 7, 10];
    let mut index = 0;
    while array.len() > index {
        println!("array 요소 {index} 번째 값은 {} 이다", array[index]);
        index += 1;
    }
}

fn for_example() {
    let array: [usize; 10] = [1, 7, 34, 456, 14, 2746, 124, 547, 175, 634];
    for item in array {
        println!("배열 array의 각 요소들은 {item}이 들어있네요");
    }
}