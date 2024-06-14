fn main() {
    let test1 = 9;
    let test2 = 30;
    let test3 = test1 | test2;
    println!("{test3}");

    let signed_8bit: i8 = -128; 
    let unsigned_8bit: u8 = 255;

    let x = 2.0;
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;
 
    let c = 'c';
    let 가: char = '가';
    let imoge = '😂';
    println!("{imoge}");

    let tup: (char, u64, bool) = ('😙', 235235235235622, false);
    let (a, b, c) = tup; // 구조 해체
    println!("튜플 값 해체 하기 : {a}, {b}, {c}");
    let tup_0 = tup.0; // 튜플 요소 접근
    println!("{tup_0}");

    let array1 = [1, 2, 3, 4, 5];
    let array1 = array1[2]; // 배열에도 섀도잉 가능
    println!("{array1}");
    let array2 : [u8; 5] = [1, 2, 3, 4, 5];
    let array2_e: u8 = array2[3]; // 배열 요소 접근
    println!("{array2_e}");
}
