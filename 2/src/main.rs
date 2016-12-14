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
        print!("{}", b.n);
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
        // print!("Moving from {} ", self.n);
        match d {
            Direction::Up => {
                if self.n > 3 {
                    self.n = self.n - 3
                }
            }
            Direction::Down => {
                if self.n < 7 {
                    self.n = self.n + 3
                }
            }
            Direction::Left => {
                if [2, 3, 5, 6, 8, 9].contains(&self.n) {
                    self.n = self.n - 1
                }
            }
            Direction::Right => {
                if [1, 2, 4, 5, 7, 8].contains(&self.n) {
                    self.n = self.n + 1
                }
            }
        };
        // println!(" in direction {:?} ==> {} ", d, self.n);
    }
}
