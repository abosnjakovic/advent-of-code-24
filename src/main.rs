mod day1;
mod utils;

fn main() {
    match utils::read_file("src/day1.input") {
        Ok(contents) => {
            println!("Day 1 input loaded successfully");
            let answer = day1::answer(&contents);
            println!("Day 1 answer: {}", answer);
        }
        Err(error) => {
            println!("Error reading day1 input: {}", error);
        }
    }
}
