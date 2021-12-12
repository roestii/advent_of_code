mod day10;

fn main() {
    let input: Vec<&str> = include_str!("day10.input").lines().collect();
    day10::handle_input(input);
}
