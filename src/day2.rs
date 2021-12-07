pub fn handle_input(input: Vec<&str>) {
    let instructions = parse_input(input); 
    let mut submarine = Submarine::new(instructions); 
    let position = submarine.handle_instructions();
    println!("x:{}, y:{} => {}", position.0, position.1, position.0 * position.1);
}

fn parse_input(input: Vec<&str>) -> Vec<Option<Instruction>> {
    let instructions = input.iter().map(|x| {
        if let Some((instruction, value)) = x.split_once(" ") {
            let value = value.parse::<i32>().unwrap();
            return match instruction {
                "down" => Some(Instruction::Down(value)),
                "up" => Some(Instruction::Up(value)),
                "forward" => Some(Instruction::Forward(value)),
                _ => None,
            }
        }
        None
    }).collect();

    instructions
}

struct Submarine {
    position: (i32, i32),
    aim: i32,
    instructions: Vec<Option<Instruction>>,
}

impl Submarine {
    fn new(instructions: Vec<Option<Instruction>>) -> Self {
        Self {
            position: (0, 0),
            aim: 0,
            instructions,
        }
    }

    fn handle_instructions(&mut self) -> (i32, i32) {
        for instruction in &self.instructions {
            if let Some(ins) = instruction {
                use Instruction::*;
                match ins {
                    Down(value) => self.aim += value,
                    Up(value) => self.aim -= value,
                    Forward(value) => {
                        self.position.0 += value; 
                        self.position.1 += value * self.aim;
                    },
                }
            }
        }

        self.position
    }
}

enum Instruction {
    Down(i32),
    Up(i32),
    Forward(i32),
}
