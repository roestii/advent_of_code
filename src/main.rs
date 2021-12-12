mod day9;

fn main() {
    let input: Vec<&str> = include_str!("day9.input").lines().collect();
    day9::handle_input(input);
}
