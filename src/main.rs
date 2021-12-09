mod day5;

fn main() {
    let input: Vec<&str> = include_str!("day5.test").lines().collect();
    day5::handle_input(input);
}
