mod day7;

fn main() {
    let input: Vec<&str> = include_str!("day7.input").lines().collect();
    day7::handle_input(input);
}
