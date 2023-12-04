use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


fn main() {

    let Ok(file) = File::open("input") else {
        panic!("file not found, exiting")
    };

    let lines = io::BufReader::new(file).lines();

    let mut total: i32 = 0;

    for line in lines {
        let Ok(ip) = line else{
            return println!("err")
        };

        let mut line_number: String = "".to_string();


        ip.find(|c: char| (c.is_digit(10)));

        ip.get(i)
        line_number.push_str(&find_first(ip, &numwords));
        // line_number.push_str(&find_second(ip));
        
        println!("{}", line_number);

        if let Ok(num) = line_number.parse::<i32>() {
            total += num;
        };
    }

    println!("total: {}", total);
    
}

fn find_first(str: String, num_words: &HashMap<&str, u32>) -> String {
    //let textnum = textnum.to_lowercase();
    
    str.find(|c: char| (c.is_digit(10)));

        
    for char in chars {
        if let Some(_num) = char.to_digit(10) {
            return char.to_string();
        };
    }

    panic!("parsable number not found")
}

// fn find_second(str: String) -> String {
//     for char in chars {
//         if let Some(_num) = char.to_digit(10) {
//             return char.to_string();
//         };
//     }

//     panic!("parsable number not found")
// }
