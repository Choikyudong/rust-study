fn main() {
    let mut s = String::new();

    let str = "Hello World";

    let s = str.to_string();

    let s = String::from("Hello World");

    // UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l');
    println!("{}",s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // let s3 = s1.add(&2) 랑 동일

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    /* 러스트는 다음과 같은 접근이 불가능
    {
        let s = String::from("hello");
        let h = s[0];
    }
    */

    {
        // 범위를 지정해 문자열 슬라이스는 가능
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("{}",s); // Зд

        // 문자열 반복도 가능
        for c in hello.chars() {
            println!("{c}");
        }

        for b in hello.bytes() {
            println!("{b}");
        }
    }

}
