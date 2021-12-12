mod day8;

fn main() {
    let input: Vec<&str> = include_str!("day8.input").lines().collect();
    day8::handle_input(input);
}
