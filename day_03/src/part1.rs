
pub fn main() {
    let file = File::open("input").unwrap();

    let reader = io::BufReader::new(file).lines();

    for result in reader {
        let line = result.unwrap();




    }
}