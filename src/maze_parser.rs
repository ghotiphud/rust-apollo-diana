use maze::{Maze, Arrow};
use maze::Color;
use maze::Color::*;
use maze::Direction;
use maze::Direction::*;

use nom::{IResult, digit, space, alpha, multispace};

use std::str;
use std::str::FromStr;

named!(number<u32>, 
    map_res!(
      map_res!(
        delimited!(opt!(multispace), digit, opt!(multispace)),
        str::from_utf8
      ),
      FromStr::from_str
    )
);

named!(array_size<(u32, u32)>, 
    chain!(
        x: number ~
        y: number,

        ||{(x,y)}
    )
);

named!(arrow<(&str, &str)>,
    chain!(
        space? ~
        color: map_res!(alpha, str::from_utf8) ~
        tag!("-") ~
        direction: map_res!(alpha, str::from_utf8) ~
        space?,

        ||{(color, direction)}
    )
);

named!(arrow_line<Vec<(&str, &str)> >, 
    chain!(
        arrows: many0!(arrow) ~
        multispace?,

        ||{arrows}
    )
);

named!(maze_board<Vec<Vec<(&str, &str)> > >,
    chain!(
        array_size ~
        lines: many0!(arrow_line),

        ||{lines}
    )
);

pub fn parse(input: &[u8]) -> Option<Maze> {
    if let IResult::Done(_, board_str) = maze_board(input) {
        let mut board = vec![];

        for r in board_str {
            let mut row = vec![];

            for a in r {
                row.push(
                    Arrow::new(
                        a.0.parse::<Color>().expect("Color"), 
                        a.1.parse::<Direction>().expect("Direction")
                    )
                );
            }

            board.push(row);
        }

        return Some(Maze::new(board));
    }

    None
}


#[test]
fn parse_array_size(){
    assert_eq!(array_size(&b"3 3 "[..]), IResult::Done(&b""[..], (3,3)));
}

#[test]
fn parse_arrow(){
    let arrow_line_str = &b"B-E R-SE R-SW"[..];
    let arrow_multiline_str = &b"B-E R-SE R-SW
R-SE B-N R-E
B-N B-E O"[..];

    let arrow_multiline_str_remaining = &b"R-SE B-N R-E
B-N B-E O"[..];

    assert_eq!(arrow(&b"B-E  "[..]), IResult::Done(&b""[..], ("B", "E")));
    assert_eq!(arrow(&arrow_line_str), IResult::Done(&b"R-SE R-SW"[..], ("B", "E")));

    assert_eq!(arrow_line(arrow_line_str), 
        IResult::Done(&b""[..], vec![("B", "E"), ("R", "SE"), ("R", "SW")]));

    assert_eq!(arrow_line(&arrow_multiline_str), 
        IResult::Done(arrow_multiline_str_remaining, vec![("B", "E"), ("R", "SE"), ("R", "SW")]));
}

#[test]
fn parse_board(){
    let input = &b"3 3
B-E R-SE R-SW
R-SE B-N R-E
B-N B-E O-O"[..];

    let board = maze_board(input);
    assert_eq!(board, IResult::Done(&b""[..], 
        vec![
            vec![("B", "E"), ("R", "SE"), ("R", "SW")],
            vec![("R", "SE"), ("B", "N"), ("R", "E")],
            vec![("B", "N"), ("B", "E"), ("O", "O")],
        ])
    );
}

#[test]
fn parse_maze(){
    let input = &b"3 3
B-E R-SE R-SW
R-SE B-N R-E
B-N B-E O-O"[..];

    let board = vec![
        vec![Arrow::new(Blue, E), Arrow::new(Red, SE), Arrow::new(Red, SW)],
        vec![Arrow::new(Red, SE), Arrow::new(Blue, N), Arrow::new(Red, E)],
        vec![Arrow::new(Blue, N), Arrow::new(Blue, E), Arrow::new(White, NA)],
    ];

    let expected_maze = Maze::new(board);

    let maze = parse(input);

    assert_eq!(format!("{:?}", maze), format!("{:?}", Some(expected_maze)));
}