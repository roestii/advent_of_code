#[derive(Debug)]
enum BoardPiece {
    Unmarked(u32),
    Marked(u32),
} 

#[derive(Debug)]
struct Bingo {
    seq: Vec<u32>,
    boards: Vec<Board>,
    count: usize,
}

#[derive(Debug)]
struct Board {
    pieces: Vec<Vec<BoardPiece>>,
    ready: bool,
}

struct Winner {
    value: u32, 
    sum: u32
}

impl Winner {
    fn new(value: u32, sum: u32) -> Self {
        Self {
            value,
            sum,
        }
    }

    fn calculate_result(&self) -> u32 {
        self.value * self.sum
    }
}

impl Iterator for Bingo {
    type Item = Winner;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.boards.len() {
            if let Some(winner) = self.boards[i].handle_next_value(self.seq[self.count]) {
                return Some(winner);
            }
        }
        self.count += 1;
        None
    }
}

impl Board {
    fn new(pieces: Vec<Vec<BoardPiece>>) -> Self {
        Self {
            pieces,
            ready: false,
        }
    }

    fn handle_next_value(&mut self, value: u32) -> Option<Winner> {
        if self.ready { return None; }
        for row in 0..self.pieces.len() {
            for col in 0..self.pieces[row].len() {
                if let BoardPiece::Unmarked(inner) = self.pieces[row][col] {
                    if inner == value {
                        self.pieces[row][col] = BoardPiece::Marked(inner);
                        if self.check(row, col) {
                            self.ready = true;
                            return Some(Winner::new(value, self.count_unmarked()));
                        }
                    }
                }
            }
        }
        None
    }

    fn count_unmarked(&self) -> u32 {
        self.pieces.iter()
            .flatten()
            .fold(0, |mut acc, piece| {
                if let BoardPiece::Unmarked(value) = piece {
                    acc += value;
                }
                acc
            })
    }

    fn check(&self, row: usize, col: usize) -> bool {
        let horizontal = self.pieces[row].iter()
            .fold(Vec::new(), |mut acc, piece| {
                if let BoardPiece::Marked(value) = piece {
                    acc.push(value);
                }
                acc
            });

        let vertical = self.pieces.iter()
            .fold(Vec::new(), |mut acc, row| {
                if let BoardPiece::Marked(value) = row[col] {
                    acc.push(value);
                }
                acc
            });

        return horizontal.len() == 5 || vertical.len() == 5;
    }
}

impl Bingo {
    fn new(seq: Vec<u32>, boards: Vec<Board>) -> Self {
        Self {
            seq,
            boards,
            count: 0,
        }
    }
}

pub fn handle_input(input: Vec<&str>) -> Option<()> {
    let seq: Vec<u32> = input.first()?
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    
    let boards = parse_boards(input[1..].to_vec());
    let mut bingo = Bingo::new(seq, boards); 
    let mut winners = Vec::new();    
    loop {
        if winners.len() == bingo.boards.len() {
            break;
        }
        if let Some(winner) = bingo.next() {
            winners.push(winner);
        }
    }

    let l = winners.last().unwrap();
    println!("v: {}, sum: {} => {}", l.value, l.sum, l.calculate_result());

    Some(())
}

fn parse_boards(input: Vec<&str>) -> Vec<Board> {
    let filtered = input.iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let boards = filtered.chunks(5)
        .fold(Vec::new(), |mut acc, board| {
            let mut pieces = Vec::new();
            board.iter().for_each(|line| {
                pieces.push(line.split(" ")
                    .filter(|v| !v.is_empty())
                    .map(|v| {
                        BoardPiece::Unmarked(v.parse::<u32>().unwrap())
                    })
                    .collect::<Vec<_>>());
            });
            acc.push(Board::new(pieces));
            acc
        });
    boards
}
