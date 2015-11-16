#[macro_use]
extern crate nom;

mod maze_parser;
mod maze;

use maze::{Maze, Arrow};
use maze::Color::*;
use maze::Direction::*;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("inputs/larger.txt").expect("File Not Found");
    let mut contents: Vec<u8> = Vec::new();

    let result = file.read_to_end(&mut contents).unwrap();

    let large_maze = maze_parser::parse(&contents[..]).unwrap();

    println!("{:?}", large_maze.solve())
}