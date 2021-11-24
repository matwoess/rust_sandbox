use std::fmt::{Display, Formatter};
use std::process::exit;

use rand::Rng;

use crate::util;
use crate::xox::Cell::{Circle, Cross, Empty};
use crate::xox::GameState::{DRAW, LOST, RUNNING, WON};

const ROWS: usize = 3;
const COLS: usize = 3;

#[derive(PartialEq)]
enum Cell {
    Cross,
    Circle,
    Empty,
}

impl Cell {
    pub const OPPONENT: Cell = Cell::Circle;
    pub const PLAYER: Cell = Cell::Cross;
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Empty => ' ',
            Cross => 'X',
            Circle => 'O',
        };
        write!(f, "{}", char)
    }
}

struct Field {
    cells: [[Cell; ROWS]; COLS],
}

impl Field {
    fn new() -> Field {
        return Field {
            cells: [
                [Empty, Empty, Empty],
                [Empty, Empty, Empty],
                [Empty, Empty, Empty],
            ],
        };
    }
}

enum GameState {
    RUNNING,
    WON,
    LOST,
    DRAW,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let letters = vec!["a", "b", "c", "d", "e", "f"];
        let header = letters[0..COLS].join(" ");
        writeln!(f, "   {}", header)?;
        writeln!(f, "  -------")?;
        for x in 0..ROWS {
            write!(f, "{} |", x)?;
            for y in 0..COLS {
                write!(f, "{}", self.cells[x][y])?;
                write!(f, "|")?;
            }
            writeln!(f, "\n  -------")?;
        }
        writeln!(f)
    }
}

pub(crate) fn main() {
    println!("> starting \"xox\" module");
    let mut field = Field::new();
    println!("Computer goes first and acts random.");
    let (x, y) = get_random_move(&field);
    field.cells[x][y] = Cell::OPPONENT;
    println!("{}", field);

    loop {
        print!("enter move coordinates: ");
        let coordinates = util::prompt_for_coordinates();
        let (x, y): (usize, usize) = match coordinates {
            Ok((x, y)) => (x, y),
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };
        if x >= ROWS || y >= COLS {
            println!("indices ({}, {}) are out of bounds!", x, y);
            continue;
        } else if field.cells[x][y] != Empty {
            println!("coordinates are already taken by '{}'", field.cells[x][y]);
            continue;
        }
        field.cells[x][y] = Cell::PLAYER;
        println!("{}", field);
        check_board(&field);
        println!("opponent's move:");
        let (x, y) = get_random_move(&field);
        field.cells[x][y] = Cell::OPPONENT;
        println!("{}", field);
        check_board(&field);
    }
}

fn check_board(field: &Field) {
    match get_game_state(field) {
        GameState::RUNNING => return,
        GameState::WON => {
            println!("Congratulations, you have won!");
            exit(0);
        }
        GameState::LOST => {
            println!("The computer has beaten you!");
            exit(0);
        }
        GameState::DRAW => {
            println!("Draw, try again!");
            exit(0);
        }
    }
}

fn get_game_state(field: &Field) -> GameState {
    let any_row_matches = |f: &Field, kind: Cell| -> bool {
        f.cells.iter().any(|row| row.iter().all(|c| c == &kind))
    };
    let any_column_matches = |f: &Field, kind: Cell| -> bool {
        return (0..COLS).into_iter().any(|col: usize| {
            f.cells
                .iter()
                .map(|row| row.get(col).expect("error getting column"))
                .all(|c| c == &kind)
        });
    };

    let any_diagonal_matches = |f: &Field, kind: Cell| -> bool {
        let max_diag_len = std::cmp::min(ROWS, COLS);
        let diag_top_down = (0..max_diag_len)
            .map(|i| &f.cells[i][i])
            .all(|c| c == &kind);
        let diag_bottom_up = (0..max_diag_len)
            .map(|i| &f.cells[ROWS-1-i][i])
            .all(|c| c == &kind);
        return diag_top_down || diag_bottom_up;
    };

    let any_row_opponent: bool = any_row_matches(field, Cell::OPPONENT);
    let any_row_player: bool = any_row_matches(field, Cell::PLAYER);
    let any_column_opponent: bool = any_column_matches(field, Cell::OPPONENT);
    let any_column_player: bool = any_column_matches(field, Cell::PLAYER);
    let any_diagonal_opponent: bool = any_diagonal_matches(field, Cell::OPPONENT);
    let any_diagonal_player: bool = any_diagonal_matches(field, Cell::PLAYER);

    if any_row_opponent || any_column_opponent || any_diagonal_opponent {
        return LOST;
    }
    if any_row_player || any_column_player  || any_diagonal_player{
        return WON;
    }

    let any_cell_empty = field
        .cells
        .iter()
        .flat_map(|columns| columns.iter())
        .any(|c| c == &Empty);
    if !any_cell_empty {
        return DRAW;
    }

    return RUNNING;
}

fn get_random_move(field: &Field) -> (usize, usize) {
    loop {
        let xy = random_coordinates();
        if field.cells[xy.0][xy.1] == Empty {
            return xy;
        }
    }
}

fn random_coordinates() -> (usize, usize) {
    let x = rand::thread_rng().gen_range(0..ROWS);
    let y = rand::thread_rng().gen_range(0..COLS);
    (x, y)
}
