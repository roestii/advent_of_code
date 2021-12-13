mod day11;

fn main() {
    let input: Vec<&str> = include_str!("day11.input").lines().collect();
    day11::handle_input(input);
}
