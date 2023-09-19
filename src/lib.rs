use std::fmt;

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
    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Implement en passant and castling.
    /* pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
            match &self {
                Piece::King => /*...*/,
                Piece::Queen => /*...*/,
        }
    }  */

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
        let mut board: [Option<Piece>; 64] = [None; 64];

        let board_template = [
            "RNBKQBNR",
            "PPPPPPPP",
            "********",
            "********",
            "********",
            "********",
            "PPPPPPPP",
            "RNBKQBNR ",
        ];

        let mut current_colour = Colour::Black; // 0 = black, 1 = white, might need to change this to include starting position idk

        for (rank, rank_str) in board_template.iter().enumerate() {
            for (file, piece_char) in rank_str.chars().enumerate() {
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
                    let index = rank * 8 + file;
                    board[index] = Some(piece);
                }
            }
        }

        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            black: 0,
            white: 1,
            board: [None; 64],
        }
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
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "")
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

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    #[test]
    fn colour_test() {
        let bpawn = Piece::Pawn(crate::Colour::Black);

        assert_eq!(bpawn.get_colour(), Colour::Black);
    }
}
