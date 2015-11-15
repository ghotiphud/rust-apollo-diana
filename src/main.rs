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

    //println!("{:?}", large_maze.possible_moves((5,5)));
    println!("{:?}", large_maze.solve())

    // let board = vec![
    //     vec![Arrow::new(Blue, E), Arrow::new(Red, SE), Arrow::new(Red, SW)],
    //     vec![Arrow::new(Red, SE), Arrow::new(Blue, N), Arrow::new(Red, E)],
    //     vec![Arrow::new(Blue, N), Arrow::new(Blue, E), Arrow::new(Red, N)],
    // ];

    // let maze = Maze::new(board);

    // println!("{:?}", maze.solve());
}