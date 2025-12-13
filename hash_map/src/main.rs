use std::collections::HashMap;

fn main() {

    let mut scores:HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 7);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let new_team_name = String::from("RED");
    let new_team_score = 5;
    scores.insert(new_team_name, new_team_score);

    /*
    println!("{new_team_name}"); <-- 소유권이 scores로 넘어감
    */

    println!("{new_team_score}");
}

