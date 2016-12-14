use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let mut b = Button { n: 5 };
    let f = File::open("../input2.txt").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        for c in line.unwrap().chars() {
            b.make_move(match c {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            });
        }
        print!("{:x}", b.n);
    }

}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Button {
    n: u8,
}



impl Button {
    pub fn make_move(&mut self, d: Direction) {
        match d {
            Direction::Up => {
                self.n = match self.n {
                    0xD => 0xB,
                    6...8 | 0xA...0xC => self.n - 4,
                    3 => 1,
                    _ => self.n,
                }
            }
            Direction::Down => {
                self.n = match self.n {
                    1 => 3,
                    2...4 | 6...8 => self.n + 4,
                    0xB => 0xD,
                    _ => self.n,
                }
            }
            Direction::Left => {
                self.n = match self.n {
                    3 | 4 | 6...9 | 0xB | 0xC => self.n - 1,
                    _ => self.n,
                }
            }
            Direction::Right => {
                self.n = match self.n {
                    2 | 3 | 5...8 | 0xA | 0xB => self.n + 1,
                    _ => self.n,
                }
            }
        };
    }
}
