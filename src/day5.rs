#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn from_str(x: &str, y: &str) ->  Self {
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Self {
            x,
            y,
        }
    }
}

pub fn handle_input(input: Vec<&str>) -> Result<(), ()> {
    let points = input.iter()
        .fold(Vec::new(), |mut acc, line| {
            let (start, end) = line.trim().split_once("->").unwrap();
            let (x1, y1) = start.trim().split_once(",").unwrap();
            let (x2, y2) = end.trim().split_once(",").unwrap();

            acc.push((Point::from_str(x1, y1), Point::from_str(x2, y2)));
            acc
        });

    let horizontals = points.iter()
        .filter(|(start, end)| start.y == end.y)
        .collect();
    let verticals = points.iter()
        .filter(|(start, end)| start.x == end.x)
        .collect();
    let diagonals = points.iter()
        .filter(|(start, end)| is_diag(start, end))
        .collect();

    let lines = parse_to_lines(horizontals, verticals, diagonals);
    let (x, y) = get_max_range(&lines);
    let (x, y) = (x as usize, y as usize);
    let mut board = vec![vec![0; x+1]; y+1];

    lines.iter()
        .flatten()
        .for_each(|point| {
            let (x, y) = (point.x as usize, point.y as usize);
            board[y][x] += 1; 
        });

    //print_board(&board);
    println!("{}", board.iter().flatten().filter(|x| **x > 1).count());
    Ok(())
}

fn is_diag(start: &Point, end: &Point) -> bool {
    let xdiff = start.x as i32 - end.x as i32;
    let ydiff = start.y as i32 - end.y as i32;

    xdiff.abs() == ydiff.abs()
}

fn print_board(board: &Vec<Vec<u32>>) {
    board.iter().for_each(|line| {
        println!("{:?}", line);
    });
}

fn get_max_range(lines: &Vec<Vec<Point>>) -> (u32, u32) {
    let max = lines.iter()
        .flatten()
        .fold((0, 0), |mut max, point| {
            if point.x > max.0 {
                max.0 = point.x;
            }

            if point.y > max.1 {
                max.1 = point.y;
            }
            max
        });
    max
}


fn parse_to_lines(horizontals: Vec<&(Point, Point)>, verticals: Vec<&(Point, Point)>, diagonals: Vec<&(Point, Point)>) -> Vec<Vec<Point>> {
    let mut lines = parse_horizontal(horizontals);
    lines.extend(parse_vertical(verticals));
    let diagonals = parse_diagonal(diagonals);
    lines.extend(diagonals);
    lines
}

fn create_range(start: u32, end: u32) -> Box<dyn Iterator<Item=u32>> {
    if start > end {
        return Box::new((end..=start).rev());
    }

    Box::new(start..=end)
}

fn parse_diagonal(diagonals: Vec<&(Point, Point)>) -> Vec<Vec<Point>> {
    let dlines = diagonals.iter()
        .fold(Vec::new(), |mut acc, (dstart, dend)| {
            //PROBLEM
            let xstep = create_range(dstart.x, dend.x);
            let ystep = create_range(dstart.y, dend.y);

            let line = xstep.zip(ystep)
                .fold(Vec::new(), |mut line, (x, y)| {
                    line.push(Point::new(x, y));
                    line
                });
            acc.push(line);
            acc
        });
    dlines
}

fn parse_vertical(verticals: Vec<&(Point, Point)>) -> Vec<Vec<Point>> {
    let vlines = verticals.iter()
        .fold(Vec::new(), |mut acc, (vstart, vend)| {
            let ystep = create_range(vstart.y, vend.y);

            let line = ystep.fold(Vec::new(), |mut points, i| {
                points.push(Point::new(vstart.x, i)); 
                points
            });
            acc.push(line);
            acc
        });
    vlines
}

fn parse_horizontal(horizontals: Vec<&(Point, Point)>) -> Vec<Vec<Point>> {
    let hlines = horizontals.iter()
        .fold(Vec::new(), |mut acc, (hstart, hend)| {
            let xstep = create_range(hstart.x, hend.x);

            let line = xstep.fold(Vec::new(), |mut points, i| {
                points.push(Point::new(i, hstart.y)); 
                points
            });
            acc.push(line);
            acc
        });

    hlines
}
