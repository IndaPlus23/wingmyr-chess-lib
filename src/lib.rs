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
    black: u64,
    white: u64,
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
            black: 0,
            white: 1,
            board: bboard,
        }
    }

    pub fn convert_notation(notation: &str) -> (&str, &str) {
        // notation should be <from position><to position> e.g. e1e2 moves the piece at e1 to e2
        let notation_vector = notation
            .split_inclusive(char::is_numeric)
            .collect::<Vec<&str>>();

        let (from, to) = (notation_vector[0], notation_vector[1]);

        return (from, to);
    }

    /// If the current game state is `InProgress` and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
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

        let _position = _position
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

        let position = rank * 8 + file;

        let position = rank * 8 + file;
        match self.board[position as usize] {
            // this would look better if you just subtracted by 8 instead
            Some(Piece::King(Colour::White)) => {
                let mut legal_moves: Vec<String> = vec![];
                // 1 step back
                if position - 1 * 8 > 0 {
                    legal_moves.push(format!("{}", position - 8));

                    // 1 step back-left
                    if (rank - 2) * 8 < position - 1 * 8 - 1 {
                        // check if target position is within the previous line
                        legal_moves.push(format!("{}", position - 8 - 1));
                    }
                    // 1 step back-right
                    if position - 8 + 1 > (rank - 1) * 8 {
                        legal_moves.push(format!("{}", position - 8 + 1));
                    }
                }
                // 1 step forward
                if position + 8 < 8 * 8 {
                    legal_moves.push(format!("{}", position + 8));

                    // 1 step forward-left
                    if position + 8 - 1 > (rank + 1) * 8 {
                        legal_moves.push(format!("{}", position + 8 - 1));
                    }
                    // 1 step forward-right
                    if position + 8 + 1 < (rank + 2) * 8 - 1 {
                        legal_moves.push(format!("{}", position + 8 + 1));
                    }
                }

                // 1 step left
                if position - 1 > (rank - 1) * 8 {
                    legal_moves.push(format!("{}", position - 1));
                }
                // 1 step right
                if position + 1 < (rank + 1) * 8 {
                    legal_moves.push(format!("{}", position + 1));
                } else {
                    return None;
                }

                return Some(legal_moves);
            }
            _ => panic!("{:?}", self.board[position as usize]),
            // Piece::Queen => /*...*/,
        }

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
        println!("{:?}", Game::convert_notation("e1e2"));
    }
}
