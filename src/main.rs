mod day6;

fn main() {
    let input: Vec<&str> = include_str!("day6.input").lines().collect();
    day6::handle_input(input);
}
