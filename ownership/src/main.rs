fn main() {
    // 변수의 스코프
    { // str는 유효하지 않다.
      let str = "Hello World~"; // str은 현재 유효하다.
    } // 스코프 종료와 함께 str은 유효하지 않다.
    
    // 문자열 리터럴로 고정된값
    let mut str = "Hello World~";

    // String::from으로 str을 힙 영역에 추가한다.
    let mut str = String::from("Hello"); // 입력한만큼 메모리가 요청됨
    str.push_str(" World~"); // 문자열을 추가한다.
    println!("{}",str);

    {
      let mut str = String::from("Hello");
    } // str의 스코프 종료로 str은 메모리 해제

    let x = 5;
    let y = x; // x, y 의 값은 스택에 푸시가 된다.

    let s1 = String::from("Hi~");
    /*
    let s2 = s1; // 같은 포인터를 참조한다.
    */
    let s2 = s1.clone(); // clone을 통해 깊은 복사를 하자
    println!("{}", s1);

    let s = String::from("Mymy~"); // s는 현재 main 스코프안에 유효하다.
    give_me_ownership(s); // s는 함수로 이동된다.
    // 이동된 s는 더 이상 유효하지 않다.

    let myStr = return_str(); // 함수의 반환값이 myStr로 이동됨

    let num = 100; // num는 main 스코프안에 유효하다.
    it_is_copy(num); // num는 함수로 이동한다.
    // num는 [Copy]로 num는 유효하다.

    let param = String::from("Hi, Rust~");
    let (res, len) = param_and_result_return(param); // String을 함수에 주고도 String을 다시 쓸 수 있다.
    print!("파라미터는 : '{}'의 길이는 '{}'", res, len);
} // num가 스코프를 벗어나며 유효하지 않게된다.

fn give_me_ownership(str: String) { // str이 스코프로 들어옴
    println!("{}", str);
} // str이 스코프를 벗어남과 동시에 [drop]이 호출됨

fn it_is_copy(num: isize) { // 변수 num이 스코프로 들어옴
    println!("{}", num);
} // 별일없음

fn return_str() -> String {
    let some_string = String::from("return Str"); // some_string이 스코프안으로 들어옴
    some_string // 호출자쪽으로 이동
}

fn param_and_result_return(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}