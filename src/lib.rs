use core::panic;
use std::{fmt, usize, vec};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 * - Read the Rust documentation, ask questions if you get stuck!
 */

///  Board looks like this
///  | H  G  F  E  D  C  B  A |
///  |------------------------|
/// 1| R  N  B  K  Q  B  N  R |
/// 2| P  P  P  P  P  P  P  P | WHITE
/// 3| *  *  *  *  *  *  *  * |
/// 4| *  *  *  *  *  *  *  * |
/// 5| *  *  *  *  *  *  *  * |
/// 6| *  *  *  *  *  *  *  * |
/// 7| P  P  P  P  P  P  P  P | BLACK
/// 8| R  N  B  K  Q  B   N R |

#[derive(Copy, Clone, Debug, PartialEq)]
enum Colour {
    White,
    Black,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
    King(Colour),
    Queen(Colour),
    Rook(Colour),
    Bishop(Colour),
    Knight(Colour),
    Pawn(Colour),
}

impl Piece {
    ///////////////////////////////// CHANGE THIS TO BE IN GAME AND ALSO TO WORK WITH THE NOTATION FUNCTION/////////////////////////////////////
    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece.
    /* pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>> {
        // reminder: position is "<file><rank>"
        // there's probably a better solution
        let position = _position
            .split(|c| c == '<' || c == '>')
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let (file, rank) = (position[0], position[1]);
        match &self {
            // this would look better if you just subtracted by 8 instead
            Piece::King(_colour) => {
                let mut legal_moves: Vec<String> = vec![];
                if (rank - 1) * 8 + file > 0 {
                    legal_moves.push(format!("<{}><{}>", file, rank - 1));

                    if (rank - 1) * 8 + (file - 1) > (rank - 1) * 8 {
                        legal_moves.push(format!("<{}><{}>", file - 1, (rank - 1)));
                    }

                    if (rank - 1) * 8 + (file + 1) < (rank - 1) * 8 + 8 {
                        legal_moves.push(format!("<{}><{}>", file + 1, (rank - 1)));
                    }
                }

                if (rank + 1) * 8 + file < 8 * 8 + file {
                    legal_moves.push(format!("<{}><{}>", file, rank + 1));

                    if (rank + 1) * 8 + (file - 1) > (rank + 1) * 8 {
                        legal_moves.push(format!("<{}><{}>", file - 1, (rank + 1)));
                    }

                    if (rank + 1) * 8 + (file + 1) < (rank + 1) * 8 + 8 {
                        legal_moves.push(format!("<{}><{}>", file + 1, (rank + 1)));
                    }
                }

                if rank * 8 + (file - 1) > rank * 8 {
                    legal_moves.push(format!("<{}><{}>", file - 1, rank));
                }
                if rank * 8 + (file + 1) < rank * 8 + 8 {
                    legal_moves.push(format!("<{}><{}>", file + 1, rank));
                } else {
                    return None;
                }

                return Some(legal_moves);
            }
            _ => None,
            // Piece::Queen => /*...*/,
        }
    } */

    pub fn get_colour(&self) -> Colour {
        match *self {
            Piece::King(colour)
            | Piece::Queen(colour)
            | Piece::Rook(colour)
            | Piece::Knight(colour)
            | Piece::Bishop(colour)
            | Piece::Pawn(colour) => colour,
        }
    }
}

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    /* black: u64,
    white: u64, */
    colour: Colour,
    board: [Option<Piece>; 64],
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let mut bboard: [Option<Piece>; 64] = [None; 64]; // needs better name

        let board_template = "RNBKQBNRPPPPPPPP********************************PPPPPPPPRNBKQBNR "; // imagine a new line every 8 characters

        let mut current_colour = Colour::White; // 0 = black, 1 = white, might need to change this to include starting position idk

        for i in 0..64 {
            bboard[i] = match board_template.chars().nth(i).unwrap() {
                'R' => Some(Piece::Rook(current_colour)),
                'N' => Some(Piece::Knight(current_colour)),
                'B' => Some(Piece::Bishop(current_colour)),
                'K' => Some(Piece::King(current_colour)),
                'Q' => Some(Piece::Queen(current_colour)),
                'P' => Some(Piece::Pawn(current_colour)),
                _ => {
                    current_colour = Colour::White;
                    None
                }
            }
        }
        /*

        for (rank, rank_str) in board_template.iter().enumerate() {
            // get the rank intself and the index of the rank
            for (file, piece_char) in rank_str.chars().enumerate() {
                // same thing but for each character in the rank
                let piece = match piece_char {
                    'R' => Some(Piece::Rook(current_colour)),
                    'N' => Some(Piece::Knight(current_colour)),
                    'B' => Some(Piece::Bishop(current_colour)),
                    'K' => Some(Piece::King(current_colour)),
                    'Q' => Some(Piece::Queen(current_colour)),
                    'P' => Some(Piece::Pawn(current_colour)),
                    _ => {
                        current_colour = Colour::White;
                        None
                    }
                };
                if let Some(piece) = piece {
                    let index = rank * 8 + file; // since bboard is a 1D array rank * 8 is used to denote which row is being written
                    bboard[index] = Some(piece);
                }
            }
        }*/

        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            /* black: 0,
            white: 1, */
            colour: Colour::White,
            board: bboard,
        }
    }
    /// Converts chess notation to position on the board
    /// input should be should be standard chess notation for a single space on the board e.g. "e1" or "e2"
    pub fn convert_from_notation(notation: &str) -> (i32, i32) {
        /* let notation_vector = notation
            .split_inclusive(char::is_numeric)
            .collect::<Vec<&str>>();

        let (from, to) = (notation_vector[0], notation_vector[1]);

        return (from, to); */

        let notation = notation
            .split_inclusive(char::is_alphabetic)
            .collect::<Vec<&str>>();
        let rank = notation[1].parse::<i32>().unwrap() - 1;
        let file = match notation[0] {
            // board is backwards
            "a" => 7,
            "b" => 6,
            "c" => 5,
            "d" => 4,
            "e" => 3,
            "f" => 2,
            "g" => 1,
            "h" => 0,
            _ => panic!(),
        };

        return (file, rank);
    }

    pub fn convert_to_notation(numeric_position: i32) -> String {
        let rank = (numeric_position / 8) + 1;
        let file = match numeric_position % 8 {
            // board is backwards
            7 => "a",
            6 => "b",
            5 => "c",
            4 => "d",
            3 => "e",
            2 => "f",
            1 => "g",
            0 => "h",
            _ => panic!(),
        };
        return format!("{}{}", file, rank);
    }

    /// If the current game state is `InProgress` and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// notation should be <from position><to position> e.g. e1e2 moves the piece at e1 to e2
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        /* let (from, to) = (
            Game::convert_from_notation(_from),
            Game::convert_from_notation(_to),
        ); */

        let from = {
            let (file, rank) = Game::convert_from_notation(_from);
            rank * 8 + file
        };

        let to = {
            let (file, rank) = Game::convert_from_notation(_to);
            rank * 8 + file
        };

        match Game::get_possible_moves(&self, _from) {
            Some(vector) => {
                if vector.contains(&_to.to_string()) {
                    self.board[to as usize] = self.board[from as usize];
                    self.board[from as usize] = None;
                } else {
                    panic!("illegal move")
                }
            }
            None => panic!("No moves"), // really need to get rid of the panics
        }

        None
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>> {
        // reminder: position is "<file><rank>"
        // there's probably a better solution
        /* let position = _position
        .split(|c| c == '<' || c == '>')
        .filter(|s| !s.is_empty())
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>(); */

        /* let _position = _position
            .split_inclusive(char::is_alphabetic)
            .collect::<Vec<&str>>();
        let rank = _position[1].parse::<i32>().unwrap() - 1;
        let file = match _position[0] {
            // board is backwards
            "a" => 7,
            "b" => 6,
            "c" => 5,
            "d" => 4,
            "e" => 3,
            "f" => 2,
            "g" => 1,
            "h" => 0,
            _ => panic!(),
        };

        let position = rank * 8 + file; // formula for getting position in the 1D array */

        let (file, rank) = Game::convert_from_notation(_position);
        let position = rank * 8 + file; // formula for getting position in the 1D array

        let mut legal_moves: Vec<String> = vec![];

        let mut new_pos: i32 = 0;

        match self.board[position as usize] {
            // this would look better if you just subtracted by 8 instead

            ////// change to be current colour later ///////
            Some(Piece::Pawn(Colour::White)) => {
                if 8 < position && position < 16 {
                    new_pos = position + 8;
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));

                    new_pos = position + 16;
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                } else {
                    new_pos = position + 8;
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                }
            }
            Some(Piece::King(Colour::White)) => {
                // 1 step back
                if position - 1 * 8 > 0 {
                    new_pos = position - 8;
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));

                    // 1 step back-left
                    if (rank - 2) * 8 < position - 1 * 8 - 1 {
                        new_pos = position - 8 - 1;
                        // check if target position is within the previous line
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                    // 1 step back-right
                    if position - 8 + 1 > (rank - 1) * 8 {
                        new_pos = position - 8 + 1;
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
                // 1 step forward
                if position + 8 < 8 * 8 {
                    new_pos = position + 8;
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));

                    // 1 step forward-left
                    if position + 8 - 1 > (rank + 1) * 8 {
                        new_pos = position + 8 - 1;
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                    // 1 step forward-right
                    if position + 8 + 1 < (rank + 2) * 8 - 1 {
                        new_pos = position + 8 + 1;
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // 1 step left
                if position - 1 > (rank - 1) * 8 {
                    new_pos = position - 1;
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                }
                // 1 step right
                if position + 1 < (rank + 1) * 8 {
                    new_pos = position + 1;
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                } else {
                    return None;
                }
            }
            _ => panic!("{:?}", self.board[position as usize]),
            // Piece::Queen => /*...*/,
        }
        return Some(legal_moves);

        /* let (file, rank) = (position[0], position[1]);
        match &self {
            // this would look better if you just subtracted by 8 instead
            Piece::King(_colour) => {
                let mut legal_moves: Vec<String> = vec![];
                if (rank - 1) * 8 + file > 0 {
                    legal_moves.push(format!("<{}><{}>", file, rank - 1));

                    if (rank - 1) * 8 + (file - 1) > (rank - 1) * 8 {
                        legal_moves.push(format!("<{}><{}>", file - 1, (rank - 1)));
                    }

                    if (rank - 1) * 8 + (file + 1) < (rank - 1) * 8 + 8 {
                        legal_moves.push(format!("<{}><{}>", file + 1, (rank - 1)));
                    }
                }

                if (rank + 1) * 8 + file < 8 * 8 + file {
                    legal_moves.push(format!("<{}><{}>", file, rank + 1));

                    if (rank + 1) * 8 + (file - 1) > (rank + 1) * 8 {
                        legal_moves.push(format!("<{}><{}>", file - 1, (rank + 1)));
                    }

                    if (rank + 1) * 8 + (file + 1) < (rank + 1) * 8 + 8 {
                        legal_moves.push(format!("<{}><{}>", file + 1, (rank + 1)));
                    }
                }

                if rank * 8 + (file - 1) > rank * 8 {
                    legal_moves.push(format!("<{}><{}>", file - 1, rank));
                }
                if rank * 8 + (file + 1) < rank * 8 + 8 {
                    legal_moves.push(format!("<{}><{}>", file + 1, rank));
                } else {
                    return None;
                }

                return Some(legal_moves);
            }
            _ => None,
            // Piece::Queen => /*...*/,
        } */
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  N  B  K  Q  B  N  R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  N  B  K  Q  B   N R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        let game = self;

        let mut board_string = String::new();
        board_string += "\n";

        for rank in 0..game.board.len() {
            // for piece in game.board[rank] {
            let piece = game.board[rank];
            match piece {
                // get better variable names
                Some(Piece::King(_)) => board_string += "♔ ",
                Some(Piece::Queen(_)) => board_string += "♕ ",
                Some(Piece::Rook(_)) => board_string += "♖ ",
                Some(Piece::Knight(_)) => board_string += "♘ ",
                Some(Piece::Bishop(_)) => board_string += "♗ ",
                Some(Piece::Pawn(_)) => board_string += "♙ ",
                None => board_string += "* ",
            };
            // }
            if (rank + 1) % 8 == 0 && rank != 0 {
                println!("{}", rank);
                board_string += "\n";
            }
        }

        write!(f, "{}", board_string)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use crate::Colour;
    use crate::Piece;

    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?} babagaboosh", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    #[test]
    fn colour_test() {
        let bpawn = Piece::Pawn(crate::Colour::Black);

        assert_eq!(bpawn.get_colour(), Colour::Black);
    }

    #[test]
    fn legal_move_king() {
        let game = Game::new();

        let placeholder = true;

        // let king = Piece::King(crate::Colour::White);

        let king_moves = game.get_possible_moves("e1");

        println!("{:?}", king_moves);

        assert_eq!(placeholder, true)
    }

    #[test]
    fn try_notation() {
        println!("{:?}", Game::convert_from_notation("e1"));
    }

    #[test]
    fn try_make_move() {
        let mut game = Game::new();

        game.make_move("e2", "e4");

        println!("{:?}", game)
    }
}
