use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    /* 직접 호출
    panic!("큰일남!!!");
    */

    /* panic을 일으키는 상황
    let v = vec![1, 2, 3];
    v[999];
    */

    /* match를 이용한 예제
    let file_result = File::open("HelloRust.txt");
    let file_result = match file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("HelloRust.txt") {
                Ok(file) => file,
                Err(err) => panic!("Can`t create the file: {err}"),
            },
            other_error => panic!("Can`t open file: {err}"),
        }
    };
    */

    let file_result = File::open("HelloRust.txt").unwrap_or_else(|err| {
       if err.kind() == ErrorKind::NotFound {
           File::create("HelloRust.txt").unwrap_or_else(|err| {
               panic!("Can`t create the file: {err}")
           })
       } else {
           panic!("Can`t open file: {err}");
       }
    });
    println!("{:?}", file_result);

    read_username_from_file();

    /* main의 반환값은 () 이므로 ? 연산자를 할 수 없다.
    let greeting_file = File::open("hello.txt")?;
    */

    last_char_of_first_line("Hello");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // ? 연산자로 위의 match의 기능을 대체할 수 있다.
    let username_file_result = File::open("hello.txt")?;

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Option도 ? 연산자가 가능
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
