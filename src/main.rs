use std::io::{self, Write};

struct Tile {
    type: Tile_type;
    possiblities: Vec<Tile_type>
}

enum Tile_type {
    None,
    Black,
    White
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
           Tile::None => write!(f, " "),
           Tile::Black => write!(f, "|"),
           Tile::White => write!(f, "-"),
       }
    }
}

fn terminal_clear() {
    print!("{esc}c", esc = 27 as char);
}

fn terminal_print(screen: Vec<Vec<Tile>>) {
    for i in screen {
        for j in i {
            print!("{}", j);
        }
        print!("\n")
    }
    io::stdout().flush().unwrap();
}


fn create_screen(res: usize) -> Vec<Vec<Tile>> {
    let mut screen: Vec<Vec<Tile>> = vec!();
    for i in 0..res {
        screen.push(vec!());
        for j in 0..res {
            screen[i].push(Tile{});
        }
    }
    screen
}

fn main() {
    println!("Hello, world!");
    let screen = create_screen(20);
    terminal_clear();
    terminal_print(screen);
}
