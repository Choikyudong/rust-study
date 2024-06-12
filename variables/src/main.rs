fn main() {
    let mut x = 5;
    println!("변수 x의 값은 : {x}");
    x = 4;
    println!("변수 x의 값은 : {x}");

    const THREE: u32 = 3;
    println!("상수 THREE의 값은 : {THREE}");

    let y = 5;
    let y = y + 3;  // 섀도잉
    {
        let y = 15;
        println!("변수 y의 값은 : {y}");
    }
    println!("변수 y의 값은 : {y}");

    let spaces = "    ";
    let spaces = spaces.len();

    /*
    아래 코드는 에러가 발생
    let spaces = "    ";
    spaces = spaces.len();
    */
    println!("변수 spaces의 값은 : {spaces}");

}
