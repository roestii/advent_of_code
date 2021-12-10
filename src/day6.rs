pub fn handle_input(input: Vec<&str>) {
    let mut fishes = input.first()
        .unwrap()
        .split(",")
        .fold(vec![0; 9], |mut fishes, state| {
            fishes[state.parse::<usize>().unwrap()] += 1; 
            fishes
        });

    (0..256).for_each(|_| fishes.grow());
    println!("{:?}", fishes.iter().sum::<usize>());
}

trait Grow {
    fn grow(&mut self);
}

impl Grow for Vec<usize> {
    fn grow(&mut self) {
        let clone = self.clone();
        let grown = clone.first().unwrap();
        for (i, amount) in clone.iter().skip(1).enumerate() {
            self[i] = *amount;
        }

        self[8] = *grown;
        self[6] += grown;
    }
}
