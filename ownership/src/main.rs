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

    /*
    let s1 = String::from("Hi~");
    let s2 = s1; // 같은 포인터를 참조한다.
    println!("{}", s1);
    */
}
