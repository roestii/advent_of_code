mod day2;
mod day3;

fn main() {
    let input: Vec<&str> = include_str!("day3.input").lines().collect();
    input.iter().for_each(|x| {
        assert_eq!(x.len(), 12);
    });
    assert_eq!(input.len(), 1000);
    day3::handle_input(input);
}
