mod day2;
mod day3;

fn main() {
    let input: Vec<&str> = include_str!("day3.input").lines().collect();
    day3::handle_input(input);
}
