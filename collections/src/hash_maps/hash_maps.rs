use std::collections::HashMap;


pub fn explore() {
    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Red", 15);

    // Using collect to build the Hash Map
    let teams = vec![String::from("Los Angeles Lakers"), String::from("Perspolis")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Los Angeles Lakers");
    let another_team_name = String::from("FC Barcelona");
    let team_score = scores.get(&team_name);
    let another_team_score = scores.get(&another_team_name);

    println!("team_score: {:?}", team_score);
//    println!("another_team_score: {}", another_team_score);

    match another_team_score {
        Some(value) => println!("team_score: {:?}", value),
        None => println!("No element found"),
    }

    let mut all_scores = HashMap::new();
    all_scores.insert(String::from("Blue"), 10);
    all_scores.insert(String::from("Red"), 115);

    for (key, value) in &all_scores {
        println!("{}: {}", key, value);
    }
}
