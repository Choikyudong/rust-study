fn main() {
    let str1 = String::from("Hi there");
    let len = return_str_len(&str1);
    println!("{}의 길이는 : {}", str1, len);

    let mut str2 = String::from("Hello ");
    dosomething_to_reference(&mut str2);
    println!("{}", str2);

    let ex1 = &mut str2;
    // let ex2 = &mut str2; <-- 참조자를 할당 후 
    // println!("{}, {}", ex1, ex2); <-- 사용할 경우 ex2에 E0499 에러가 발생한다.
    println!("{}", ex1); // <-- ex1의 참조자가 사라짐
    
    let ex2 = &mut str2;
    {
        // let ex3 = &mut str2; <-- 위에 ex2가 이미 str2를 빌렸기 때문에 스코프가 달라져도 대여가 불가능
        // println!("{}", ex3);
    }
    println!("{}", ex2);

    {
        let ex3 = &mut str2;
    } // ex3는 스코프와 함께 사라져 새 참조자를 만드는데 문제가 없다.
    let ex4 = &mut str2;

    let mut i = 10;
    let r1 = &i;
    let r2 = &i;
    let r3 = &mut i; // 문제 발생
    // println!("{}, {}, and {}", r1, r2, r3);

    // let dangling_pointer = dangle();
}

fn return_str_len(s: &String) -> usize { // s는 참조자
    s.len()
} // s가 스코프밖으로 나가지만 참조하는 것을 소유하지 않아서 안사라진다.

/* 에러코드
fn dosomething_to_reference(s: &String) {
    s.push_str("try~");
}
*/

fn dosomething_to_reference(s: &mut String) {
    s.push_str("World");
}

/*
러스트는 해당 함수 자체가 존재하는한 컴파일을 막아버린다.
fn dangle() -> &String {
    let s = String::from("dangle~");
    &s <-- 값의 소유권 이전이 아닌 참조자를 반환한다.
} s는 스코프를 벗어나기 때문에 메모리에서 해제가 된다.
메모리에서 사라진 s의 주소를 반환하게 된다;;
*/