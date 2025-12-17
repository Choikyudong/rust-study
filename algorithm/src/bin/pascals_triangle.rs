fn main() {
    // [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
    println!("{:?}", generate(5));

    // [[1]]
    println!("{:?}", generate(1));
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
    for i in 1..=num_rows {
        let mut row: Vec<i32> = vec![1; i as usize];
        if i > 2 {
            let prev_row = &ans[(i - 2) as usize];
            for j in 1..i - 1 {
                let num = prev_row[(j - 1) as usize] + prev_row[j as usize];
                row[j as usize] = num;
            }
        }
        ans.push(row);
    }

    ans
}