use std::fmt;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut screen = Screen::new();

    for line in reader.lines() {
        let line = line.unwrap();
        parse_instructions(&mut screen, &line);
    }

    println!("{}", screen.count_trues());

}

fn parse_instructions(screen: &mut Screen, s: &str) {
    if s.starts_with("rect") {
        let args: Vec<usize> = s.split(" ")
            .nth(1)
            .unwrap()
            .split('x')
            .map(|s| s.parse().unwrap())
            .collect();
        screen.rect(args[0], args[1]);
    } else {
        let args = s.split(" ");
        let x_or_y: usize =
            args.clone().skip(2).nth(0).unwrap().split('=').nth(1).unwrap().parse().unwrap();
        let shift: usize = args.clone().skip(2).nth(2).unwrap().parse().unwrap();
        match args.clone().nth(1) {
            Some("row") => screen.rotate_row(x_or_y, shift),
            Some("column") => screen.rotate_column(x_or_y, shift),
            _ => unreachable!(),
        }
    }
}

struct Screen {
    pub pixels: [[bool; 50]; 6],
}

impl Screen {
    pub fn new() -> Screen {
        Screen { pixels: [[false; 50]; 6] }
    }

    pub fn rect(&mut self, a: usize, b: usize) {
        for row_index in 0..b {
            for col_index in 0..a {
                self.pixels[row_index][col_index] = true;
            }
        }
    }

    pub fn rotate_row(&mut self, row_index: usize, shift: usize) {
        let mut row = self.pixels[row_index];
        let mut new_row = [false; 50];
        for index in 0..row.len() {
            new_row[(index + shift) % row.len()] = row[index];
        }
        self.pixels[row_index] = new_row;
    }

    pub fn rotate_column(&mut self, col_index: usize, shift: usize) {
        let mut v = Vec::new();
        let length = self.pixels.len();
        for depth in 0..length {
            v.push(self.pixels[depth][col_index]);
        }
        v.reverse();
        for depth in 0..length {
            self.pixels[(depth + shift) % length][col_index] = v.pop().unwrap();
        }
    }

    pub fn count_trues(&self) -> u32 {
        self.pixels
            .iter()
            .flat_map(|row| row.iter())
            .fold(0, |count, &b| if b { count + 1 } else { count })
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::with_capacity(self.pixels.len());
        for row in self.pixels.iter() {
            for b in row.iter() {
                match *b {
                    true => buffer.push('#'),
                    false => buffer.push('.'),
                }
            }
            buffer.push('\n');
        }
        write!(f, "{}", buffer)
    }
}
