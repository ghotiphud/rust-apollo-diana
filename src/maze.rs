use std::ops::Index;
use std::str::FromStr;

use self::Color::*;
use self::Direction::*;

#[derive(Debug)]
pub struct Maze {
    height: usize,
    width: usize,
    board: Vec<Vec<Arrow>>,
}

impl Maze {
    pub fn new(board: Vec<Vec<Arrow>>) -> Maze {
        Maze {
            height: board.len(),
            width: board[0].len(),
            board: board,
        }
    }

    pub fn solve(&self) -> Vec<(usize, usize)> {
        let start = vec![(0,0)];
        let end = (self.width - 1, self.height - 1);

        self.solve_recursive(start, end).unwrap()
    }

    fn solve_recursive(&self, path: Vec<(usize, usize)>, end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let position = path.last().unwrap().clone();

        for m in self.possible_moves(position) {
            let mut new_path = path.clone();
            new_path.push(m);

            if m == end {
                println!("win! ");
                return Some(new_path);
            } 
            else if path.contains(&m) {
                continue;
            }

            let solution = self.solve_recursive(new_path, end);

            if solution.is_some() {
                return solution;
            }
        }

        return None;
    }

    fn possible_moves(&self, loc: (usize, usize)) -> Vec<(usize, usize)> {
        let ref arrow = self[loc];
        let mut moves = Vec::new();

        for l in arrow.follow(loc).take_while(|&(r, c)| r < self.height && c < self.width) {
            if self[l].color != arrow.color {
                moves.push(l);
            }
        }

        moves
    }
}

impl Index<(usize, usize)> for Maze {
    type Output = Arrow;
    fn index(&self, loc: (usize, usize)) -> &Arrow {
        &self.board[loc.0][loc.1]
    }
}

#[derive(Debug)]
pub struct Arrow {
    color: Color,
    direction: Direction,
}

impl Arrow {
    pub fn new(color: Color, dir: Direction) -> Arrow {
        Arrow {color: color, direction: dir}
    }

    fn follow(&self, start_loc:(usize, usize)) -> DirectionIterator {
        DirectionIterator{ loc: start_loc, direction: self.direction }
    }
}

struct DirectionIterator {
    loc: (usize, usize),
    direction: Direction,
}

impl Iterator for DirectionIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize,usize)> { 
        let (mut r, mut c) = self.loc;

        r = match self.direction {
            N | NE | NW => if r == 0 { return None; } else { r - 1 },
            S | SE | SW => r + 1,
            _ => r
        };

        c = match self.direction {
            E | NE | SE => c + 1,
            W | NW | SW => if c == 0 { return None; } else { c - 1 },
            _ => c
        };

        self.loc = (r, c);
        Some(self.loc)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Red,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Color, ()> {
        let res = match s {
            "O" => White,
            "R" => Red,
            "B" => Blue,
            _ => return Err(()),
        };

        Ok(res)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    NA,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, ()> {
        let res = match s {
            "O" => NA,
            "N" => N,
            "NE" => NE,
            "E" => E,
            "SE" => SE,
            "S" => S,
            "SW" => SW,
            "W" => W,
            "NW" => NW,
            _ => return Err(()),
        };

        Ok(res)
    }
}