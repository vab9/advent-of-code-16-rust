use std::fmt;

fn main() {
    let mut screen = Screen::new();
    println!("{}", screen);
    screen.rect(3, 5);
    println!("{}", screen);

}

#[derive(Debug)]
struct Screen {
    // pub pixels: [[bool; 50]; 6],
    pub pixels: [[bool; 10]; 6],
}

impl Screen {
    pub fn new() -> Screen {
        // Screen { pixels: [[false; 50]; 6] }
        Screen { pixels: [[false; 10]; 6] }
    }

    pub fn rect(&mut self, a: usize, b: usize) {
        for row_index in 0..b {
            for col_index in 0..a {
                self.pixels[row_index][col_index] = true;
            }
        }
    }

    pub fn rotate_row(&mut self, row: u8, shift: u8) {
        unimplemented!();
    }

    pub fn rotate_column(&mut self, col: u8, shift: u8) {
        unimplemented!();
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
