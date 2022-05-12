use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::io::{self, Write};

#[derive(EnumIter, Clone)]
enum TileType {
    None,
    Black,
    White
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
           TileType::None => write!(f, " "),
           TileType::Black => write!(f, "|"),
           TileType::White => write!(f, "-"),
       }
    }
}

pub struct Screen {
    content: Vec<Vec<TileType>>,
    options: Vec<Vec<Vec<TileType>>>,
}

impl Screen {
    pub fn new(size: usize) -> Self {
        let all_options: TileTypeIter = TileType::iter();
        Self {
            content: vec![vec![TileType::None; size]; size],
            //options: vec![vec![vec![all_options.next().unwrap(); all_options.count()]; size]; size],
            options: vec![vec![all_options.collect(); size]; size],
        }
    }

    pub fn print(&self) {
        for i in &self.content {
            for j in i {
                print!("{}", j);
            }
            print!("\n")
        }
        io::stdout().flush().unwrap();
    }

    pub fn clear() {
        print!("{esc}c", esc = 27 as char);
    }
}

fn main() {
    let screen = Screen::new(20);
    screen.print();
    screen.print();
}
