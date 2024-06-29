fn main() {
    let mut str = String::from("Hello Rust");
    let len = first_word(&str); // 이렇게 단어를 가져오는데 문제는 없다.
    str.clear(); // 하지만 이렇게 str의 값에 변화가 생기면 버그를 유발할 수 있다.
    println!("{}", len);
    
    let my_word = String::from("Hello Kyu");

    let hello = &my_word[0..5];
    let kyu = &my_word[6..9];
    println!("{} {}", hello, kyu);

    let hello_kyu = &my_word[..9]; // start_index 생략
    let only_kyu = &my_word[6..]; // end_index 생략
    println!("{}, {}", hello_kyu, only_kyu);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[2..3]; // 문자열 외에도 배열도 슬라이스가 가능하다.
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter(반복자)를 이용한 반환으로 튜플의 첫번째는 인덱스, 두번째는 요소의 참조자
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b를 붙이면 바이트 리터럴로 변경한다.
            return i;
        }
    }
    
    s.len()
}