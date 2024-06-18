fn main() {
    println!("Hello, world!");
    second_function();
    my_function(25);
    my_function_two('🍕', 239874903);
    statement_example();
    let number = return_five();
    println!("return_five() 결과값은 ??? : {number}");
    let number = put_parmeter(30); // 섀도잉
    println!("return_five() 결과값은 ??? : {number}");
}

fn second_function() {
    println!("또 다른 함수~");
}

fn my_function(param: i8) {
    println!("입력된 숫자는 {param} 입니다.");
}

fn my_function_two(param1: char, param2: u32) {
    println!("입력된 문자는 {param1} 이고 숫자는 {param2} 입니다.");
}

fn statement_example() {
    /*
    let number = 10; // let 키워드를 만들고 값을 할당하는 것은 구문이다.
    let x = (let y = 6) => (let y = 6) 구문이 값을 반환하지 않아 x에 값을 할당할 수 없다.
    */
    let block_number = { // 표현식 예제
        let inner_number = 10;
        inner_number + 1 // <-- 표현식은 종결할 때 ;을 사용안한다. ;사용시 구문으로 변경된다.
    };
    println!("블록 영역 계산 결과값은 {block_number} 입니다.");
}

fn return_five() -> u8 { // 반환 값을 갖는 함수
    5 // 흔한 return 키워드도 입력하지 않아도 된다.
}

fn put_parmeter(num: isize) -> isize {
    num + num
}