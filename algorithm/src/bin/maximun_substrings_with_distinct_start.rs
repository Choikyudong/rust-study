fn main() {
    // [0,0,1,1]
    println!("{:?}", max_distinct(String::from("abab")));

    // [0,0,1,1,1]
    println!("{:?}", max_distinct(String::from("abcd")));
}


pub fn max_distinct(s: String) -> i32 {
    let mut arr: [bool; 26] = [false; 26];

    let mut count: i32 = 0;

    for i in s.bytes() {
        let index = (i - 97) as usize;
        if !arr[index] {
            arr[index] = true;
            count += 1;
        }
    }

    count
}
