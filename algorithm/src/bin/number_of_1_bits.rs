fn main() {
    let output = hamming_weight(11);
    println!("{:?}",output);

    let output = hamming_weight(128);
    println!("{:?}",output);

    let output = hamming_weight(2147483645);
    println!("{:?}",output);
}

pub fn hamming_weight(n: i32) -> i32 {
    n.count_ones() as i32
}
