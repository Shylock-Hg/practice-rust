use std::collections::HashMap;

fn main() {
        // create and insert
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("{:?}", scores);

        // create from keys and values
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:?}", scores);

        // ownership
        let key = String::from("Favorite color");
        let value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(key, value);  // moved
        // println!("{}-{}", key, value);  // error access moved resource

        // get item
        let team = String::from("Blue");
        let score = scores.get(&team);
        match score {
                None => println!("None key blue!"),
                Some(value) => println!("Value of key blue is {}", value),
        };

        // iterate HashMap
        for (key, value) in &scores {
                println!("Value of {} is {}!", key, value);
        }

        // update
        // overwrite
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        scores.insert(String::from("Blue"), 5);
        scores.insert(String::from("Blue"), 30);
        println!("{:?}", scores);

        // insert when key not exists
        scores.entry(String::from("Blue")).or_insert(50);  // not insert
        scores.entry(String::from("Black")).or_insert(100);  // insert new key
        println!("{:?}", scores);

        // update based on the Old value
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
                let count = map.entry(word).or_insert(0);
                *count +=1;  // count is mut& of HashMap value
        }
        println!("{:?}", map);

        // Hasher
        // cryptographically strong hash default
        // Using different hash by trait BuildHasher
}
