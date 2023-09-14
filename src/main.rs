use std::str::FromStr;
use anyhow::{Result, Context, anyhow};

fn get_input() -> &'static str {
    return "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let result = s.split_once(",");
        if result.is_none() {
            return Err(anyhow!("Invalid point"));
        }
        let (x, y) = result.unwrap();
        let x: i32 = str::parse(x).context("Invalid x")?;
        let y: i32 = str::parse(y).context("Invalid y")?;
        return Ok(Point {
            x, y
        });
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let result = s.split_once(" -> ");
        if result.is_none() {
            return Err(anyhow!("expected a line to contain ->"));
        }
        let (p1, p2) = result.unwrap();
        let p1 = str::parse(p1)?;
        let p2 = str::parse(p2)?;

        return Ok(Line {p1, p2});
    }
}

impl Line {
    fn is_orthogonal(&self) -> bool {
        return self.p1.x == self.p2.x || self.p1.y == self.p2.y;
    }
    fn is_horizontal(&self) -> bool {
        return self.p1.y == self.p2.y;
    }
}

fn main() {
    let lines = get_input()
        .lines()
        .flat_map(|x| str::parse(x))
        .filter(|x: &Line| x.is_orthogonal())
        .collect::<Vec<Line>>();

    let mut matrix = vec![vec![0; 10]; 10];

    for line in &lines {
        if line.is_horizontal() {
            let y = line.p1.y as usize;
            let start_x = std::cmp::min(line.p1.x, line.p2.x) as usize;
            let end_x = std::cmp::max(line.p1.x, line.p2.x) as usize;

            for x in start_x..=end_x {
                matrix[y][x] += 1;
            }
        } else {
            let x = line.p1.x as usize;
            let start_y = std::cmp::min(line.p1.y, line.p2.y) as usize;
            let end_y = std::cmp::max(line.p1.y, line.p2.y) as usize;

            for y in start_y..=end_y {
                matrix[y][x] += 1;
            }
        }
    }

    let overlaps: usize = matrix
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&count| count >= 2)
        .count();

    println!("Number of overlaps: {}", overlaps);
}
