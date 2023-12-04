use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn main() {
    let file = File::open("input").unwrap();

    let reader = io::BufReader::new(file).lines();

    let mut total: i32 = 0;
    
    for result in reader {
        let line = result.unwrap();

        let tokens = line.split(|c: char| c.is_whitespace());
        let token_list = tokens.collect::<Vec<&str>>();

        let mut cube_counts: HashMap<&str, i32> = HashMap::new();
        cube_counts.insert("red",   0);
        cube_counts.insert("green", 0);
        cube_counts.insert("blue",  0);

        for index in (2..token_list.len()).step_by(2) {
            let cubes_shown: i32 = token_list[index].parse().unwrap();
            
            let cube_color = strip_string(token_list[index + 1]);

            if &cubes_shown > cube_counts.get(cube_color).unwrap() {
                cube_counts.insert(cube_color, cubes_shown);
            }
        }

        total += cube_counts.get("red").unwrap() *
            cube_counts.get("green").unwrap() *
            cube_counts.get("blue").unwrap();

    }

    println!("part 2 answer: {}", total);
}

fn strip_string(string: &str) -> &str {
    let suffix_tokens = [",", ";", ":"];

    for suffix in suffix_tokens {
        if let Some(stripped) = string.strip_suffix(suffix) {
            return stripped;
        }
    }

    string
}