use core::panic;
use std::{fmt, string, usize, vec};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    CheckMate,
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
pub enum Colour {
    White,
    Black,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    King(Colour),
    Queen(Colour),
    Rook(Colour),
    Bishop(Colour),
    Knight(Colour),
    Pawn(Colour),
}

pub struct Game {
    state: GameState,

    active_colour: Colour,
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
                    current_colour = Colour::Black;
                    None
                }
            }
        }

        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,

            active_colour: Colour::White,
            board: bboard,
        }
    }
    /// Converts chess notation to position on the board
    /// input should be should be standard chess notation for a single space on the board e.g. "e1" or "e2"
    pub fn convert_from_notation(notation: &str) -> (i32, i32) {
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
            _ => panic!("cannot assign {} to letter", numeric_position % 8),
        };
        return format!("{}{}", file, rank);
    }

    pub fn get_piece_colour(&self, position: i32) -> Option<Colour> {
        match self.board[position as usize] {
            Some(Piece::King(colour))
            | Some(Piece::Queen(colour))
            | Some(Piece::Rook(colour))
            | Some(Piece::Knight(colour))
            | Some(Piece::Bishop(colour))
            | Some(Piece::Pawn(colour)) => Some(colour),
            None => None,
        }
    }

    /// If the current game state is `InProgress` and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// notation should be <from position><to position> e.g. e1e2 moves the piece at e1 to e2
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        if self.get_game_state() == GameState::CheckMate {
            eprintln!("game is over");
            return None;
        }

        let from = {
            let (file, rank) = Game::convert_from_notation(_from);
            rank * 8 + file
        };

        let to = {
            let (file, rank) = Game::convert_from_notation(_to);
            rank * 8 + file
        };

        match Game::get_possible_moves(&self, self.board, _from, self.active_colour) {
            Some(vector) => {
                if let Some(legal_moves) = self.get_legal_moves(_from, self.active_colour, vector) {
                    if legal_moves.contains(&_to.to_string()) {
                        self.board[to as usize] = self.board[from as usize];
                        self.board[from as usize] = None;
                    } else {
                        eprintln!("illegal move");
                        return Some(self.get_game_state());
                    }
                }
            }
            None => return None,
        }

        if self.active_colour == Colour::White && self.check_checker(Colour::Black, self.board) {
            if self.checkmate_checker(Colour::Black) {
                eprintln!("game is over");
                self.state = GameState::CheckMate;
                return Some(GameState::CheckMate);
            } else {
                self.state = GameState::CheckMate;
                return Some(GameState::Check);
            }
        } else if self.active_colour == Colour::Black
            && self.check_checker(Colour::White, self.board)
        {
            println!("{}", self.checkmate_checker(Colour::White));
            if self.checkmate_checker(Colour::White) {
                eprintln!("game is over");
                self.state = GameState::CheckMate;
                return Some(GameState::CheckMate);
            } else {
                self.state = GameState::CheckMate;
                return Some(GameState::Check);
            }
        }

        if self.active_colour == Colour::White {
            self.active_colour = Colour::Black
        } else {
            self.active_colour = Colour::White
        }

        return Some(GameState::InProgress);
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _position: &str, _piece: &str) {
        let (file, rank) = Game::convert_from_notation(_position);
        let position = rank * 8 + file;

        if self.board[position as usize].is_some() {
            if self.board[position as usize] != Some(Piece::Pawn(self.active_colour)) {
                eprintln!("you can only promote pawns")
            } else {
                match _piece {
                    "q" => self.board[position as usize] = Some(Piece::Queen(self.active_colour)),
                    "r" => self.board[position as usize] = Some(Piece::Queen(self.active_colour)),
                    "n" => self.board[position as usize] = Some(Piece::Queen(self.active_colour)),
                    "b" => self.board[position as usize] = Some(Piece::Queen(self.active_colour)),
                    _ => return,
                }
            }
        }
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// takes position where king is standing or will be standing and returns true if that space is threatened
    fn check_checker(&self, checking_for: Colour, board: [Option<Piece>; 64]) -> bool {
        // check for pieces that can reach the king directly

        let mut position = 0; //needs a value for the compiler to shut up

        // if the current piece isn't the king, find king and run program

        for i in 0..board.len() {
            if board[i] == Some(Piece::King(checking_for)) {
                position = i as i32;
                break;
            }
        }
        // if there is a legal move threatening the king
        for piece in 0..board.len() {
            if board[piece].is_some() {
                if self.get_piece_colour(piece as i32) != Some(checking_for) {
                    let other_c: Colour = match checking_for {
                        Colour::White => Colour::Black,
                        Colour::Black => Colour::White,
                    };

                    if let Some(enemy_positions) = Game::get_possible_moves(
                        &self,
                        board,
                        &Game::convert_to_notation(piece as i32),
                        other_c,
                    ) {
                        if enemy_positions.contains(&Game::convert_to_notation(position)) {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    /// finds all possible moves for white/black and returns true if there are none (checkmate)
    pub fn checkmate_checker(&self, checking_for: Colour) -> bool {
        let mut all_moves: Vec<Vec<String>> = vec![];
        // all pieces
        for (index, piece) in self.board.iter().enumerate() {
            if self.get_piece_colour(index as i32) != Some(checking_for) {
                continue;
            }
            if let Some(possible_moves) = self.get_possible_moves(
                self.board,
                &Game::convert_to_notation(index as i32),
                checking_for,
            ) {
                match piece {
                    Some(piece) => {
                        if let Some(move_set) = self.get_legal_moves(
                            &Game::convert_to_notation(index as i32),
                            checking_for,
                            possible_moves,
                        ) {
                            if !move_set.is_empty() {
                                all_moves.push(move_set);
                            }
                        }
                    }
                    None => continue,
                }
            }
        }

        // return true if there are no legal moves
        if all_moves.is_empty() {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_possible_moves(
        &self,
        board: [Option<Piece>; 64],
        _position: &str,
        checking_for: Colour,
    ) -> Option<Vec<String>> {
        // reminder: position is "<file><rank>"
        // there's probably a better solution

        let (file, rank) = Game::convert_from_notation(_position);
        let position = rank * 8 + file; // formula for getting position in the 1D array

        match board[position as usize] {
            Some(Piece::King(colour))
            | Some(Piece::Queen(colour))
            | Some(Piece::Rook(colour))
            | Some(Piece::Knight(colour))
            | Some(Piece::Bishop(colour))
            | Some(Piece::Pawn(colour)) => {
                if colour != checking_for {
                    eprintln!("not your piece");
                    return None;
                }
            }
            None => {
                eprintln!("no piece");
                return None;
            }
        }

        let mut potential_moves: Vec<String> = vec![];

        let mut new_pos: i32;

        match board[position as usize] {
            Some(Piece::Pawn(Colour::White)) => {
                if 8 < position && position <= 16 {
                    // if pawn hasn't been moved
                    new_pos = position + 8;
                    if board[(position + 8) as usize].is_none() {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));

                        new_pos = position + 16; // two steps forward
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                } else {
                    new_pos = position + 8;
                    if board[new_pos as usize].is_none() {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
                // check for capturable pieces
                if board[(position + 8 + 1) as usize].is_some() {
                    if self.get_piece_colour(position + 8 - 1) != Some(checking_for) {
                        potential_moves
                            .push(format!("{}", Game::convert_to_notation(position + 8 - 1)));
                    }
                }
                if board[(position + 8 - 1) as usize].is_some() {
                    if self.get_piece_colour(position + 8 + 1) != Some(checking_for) {
                        potential_moves
                            .push(format!("{}", Game::convert_to_notation(position + 8 + 1)));
                    }
                }
            }
            Some(Piece::Pawn(Colour::Black)) => {
                if 48 < position && position <= 56 {
                    // if pawn hasn't been moved
                    new_pos = position - 8;
                    if board[(position - 8) as usize].is_none() {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));

                        new_pos = position - 16;
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    }
                } else {
                    new_pos = position - 8;
                    if board[new_pos as usize].is_none() {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                if board[(position - 8 + 1) as usize].is_some() {
                    if self.get_piece_colour(position + 8 - 1) != Some(checking_for) {
                        potential_moves
                            .push(format!("{}", Game::convert_to_notation(position - 8 - 1)));
                    }
                }

                if board[(position - 8 - 1) as usize].is_some() {
                    if self.get_piece_colour(position + 8 + 1) != Some(checking_for) {
                        potential_moves
                            .push(format!("{}", Game::convert_to_notation(position - 8 + 1)));
                    }
                }
            }
            Some(Piece::Rook(_colour)) => {
                // vertical movement
                // up
                for y in (0..rank).rev() {
                    new_pos = y * 8 + file;

                    // check if something's in the way
                    if board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if board[new_pos as usize] == Some(Piece::Rook(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            potential_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }

                    if file <= new_pos {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
                // down
                for y in rank..8 {
                    new_pos = y * 8 + file;

                    // check if something's in the way
                    if board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if board[new_pos as usize] == Some(Piece::Rook(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            potential_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }

                    if new_pos <= 7 * 8 + file {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // horizontal movement

                // left
                for x in (0..file).rev() {
                    new_pos = rank * 8 + x;

                    if board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if board[new_pos as usize] == Some(Piece::Rook(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            potential_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }
                    if rank * 8 <= new_pos {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // right
                for x in file..8 {
                    new_pos = rank * 8 + x;

                    if board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if board[new_pos as usize] == Some(Piece::Rook(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            potential_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }
                    if new_pos < (rank + 1) * 8 {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
            }
            Some(Piece::Bishop(_colour)) => {
                for i in 0..8 {
                    new_pos = (rank + i) * 8 + file + i;

                    let current_file = new_pos - (rank + i) * 8;
                    let current_rank = (new_pos - (file + i)) / 8;

                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    {
                        // check if something's in the way
                        if board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (board[new_pos as usize]) == Some(Piece::Bishop(checking_for)) {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                potential_moves
                                    .push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                for i in 0..8 {
                    new_pos = (rank + i) * 8 + file - i;

                    // check if something's in the way
                    let current_file = new_pos - (rank + i) * 8;
                    let current_rank = (new_pos - (file - i)) / 8;

                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    {
                        if board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if board[new_pos as usize] == Some(Piece::Bishop(checking_for)) {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                potential_moves
                                    .push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                for i in 0..8 {
                    new_pos = (rank - i) * 8 + file - i;

                    let current_file = new_pos - (rank - i) * 8;
                    let current_rank = (new_pos - (file - i)) / 8;
                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    {
                        // check if something's in the way
                        if board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (board[new_pos as usize]) == Some(Piece::Bishop(checking_for)) {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                potential_moves
                                    .push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                eprintln!("backward-right");
                for i in 0..8 {
                    new_pos = (rank - i) * 8 + file + i;

                    let current_file = new_pos - (rank - i) * 8;
                    let current_rank = (new_pos - (file + i)) / 8;
                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    {
                        // check if something's in the way
                        if board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (board[new_pos as usize]) == Some(Piece::Bishop(checking_for)) {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                potential_moves
                                    .push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
            }

            Some(Piece::Queen(_colour)) => {
                //Diagonal movement
                for i in 0..8 {
                    new_pos = (rank + i) * 8 + file + i;

                    let current_file = new_pos - (rank + i) * 8;
                    let current_rank = (new_pos - (file + i)) / 8;

                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    {
                        // check if something's in the way
                        if board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (board[new_pos as usize]) == Some(Piece::Queen(checking_for)) {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                potential_moves
                                    .push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file + i, rank + i)
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                for i in 0..8 {
                    new_pos = (rank + i) * 8 + file - i;

                    // check if something's in the way
                    let current_file = new_pos - (rank + i) * 8;
                    let current_rank = (new_pos - (file - i)) / 8;

                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    {
                        if board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if board[new_pos as usize] == Some(Piece::Queen(checking_for)) {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                potential_moves
                                    .push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file - i, rank + i)
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                for i in 0..8 {
                    new_pos = (rank - i) * 8 + file - i;

                    let current_file = new_pos - (rank - i) * 8;
                    let current_rank = (new_pos - (file - i)) / 8;
                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    {
                        // check if something's in the way
                        if board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (board[new_pos as usize]) == Some(Piece::Queen(checking_for)) {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                potential_moves
                                    .push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file - i, rank - i)
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                //backward-right
                for i in 0..8 {
                    new_pos = (rank - i) * 8 + file + i;

                    let current_file = new_pos - (rank - i) * 8;
                    let current_rank = (new_pos - (file + i)) / 8;
                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    {
                        // check if something's in the way
                        if board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (board[new_pos as usize]) == Some(Piece::Queen(checking_for)) {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                potential_moves
                                    .push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file + i, rank - i)
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }

                // vertucak movement

                for y in (0..rank).rev() {
                    new_pos = y * 8 + file;

                    // check if something's in the way
                    if board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if board[new_pos as usize] == Some(Piece::Queen(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            potential_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }

                    if file <= new_pos {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
                // down
                for y in rank..8 {
                    new_pos = y * 8 + file;

                    // check if something's in the way
                    if board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if board[new_pos as usize] == Some(Piece::Queen(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            potential_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }

                    if new_pos <= 7 * 8 + file {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // horizontal movement

                // left
                for x in (0..file).rev() {
                    new_pos = rank * 8 + x;

                    if board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if board[new_pos as usize] == Some(Piece::Queen(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            potential_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }
                    if rank * 8 <= new_pos {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // right
                for x in file..8 {
                    new_pos = rank * 8 + x;

                    if board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if board[new_pos as usize] == Some(Piece::Queen(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            potential_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }
                    if new_pos < (rank + 1) * 8 {
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
            }

            Some(Piece::Knight(_colour)) => {
                // forward
                if (rank + 2) * 8 < 7 * 8 + file {
                    // right
                    if file + 1 < 8 {
                        potential_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank + 2) * 8 + file + 1)
                        ));
                    }
                    // left
                    if file - 1 >= 0 {
                        potential_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank + 2) * 8 + file - 1)
                        ));
                    }
                }
                // backwards
                if (rank - 2) * 8 >= file {
                    //right
                    if file + 1 < 8 {
                        potential_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank - 2) * 8 + file + 1)
                        ));
                    }
                    // left
                    if file - 1 >= 0 {
                        potential_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank - 2) * 8 + file - 1)
                        ));
                    }
                }
                // right
                if file + 2 < 8 {
                    // forward
                    if (rank + 1) * 8 < 7 * 8 + file + 2 {
                        potential_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank + 1) * 8 + file + 2)
                        ));
                    }
                    // backward
                    if (rank - 1) * 8 + file > file + 2 {
                        potential_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank - 1) * 8 + file + 2)
                        ));
                    }
                }
                // left
                if file - 2 >= 0 {
                    //backward
                    if (rank - 1) * 8 + file - 2 > 7 * 8 + file - 2 {
                        potential_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank - 1) * 8 + file - 2)
                        ));
                    }
                    // forward
                    if (rank + 1) * 8 < 7 * 8 + file - 2 {
                        potential_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank + 1) * 8 + file - 2)
                        ));
                    }
                }

                let mut temp: Vec<String> = vec![];

                // remove moves that collide with friendly pieces
                for i in 0..potential_moves.len() {
                    let (file, rank) = Game::convert_from_notation(&potential_moves[i]);
                    // if there is a piece and it's hostile, make it capturable
                    if board[(rank * 8 + file) as usize].is_some() {
                        if self.get_piece_colour(rank * 8 + file) != Some(checking_for) {
                            temp.push((&potential_moves[i]).to_string());
                        }
                        // if there is no piece you can obviously go there
                    } else {
                        temp.push((&potential_moves[i]).to_string())
                    }
                }
                potential_moves = temp;
            }

            Some(Piece::King(_colour)) => {
                // 1 step back
                if position - 1 * 8 > 0 {
                    new_pos = position - 8;
                    potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));

                    // 1 step back-left
                    if (rank - 2) * 8 < position - 1 * 8 - 1 {
                        new_pos = position - 8 - 1;
                        // check if target position is within the previous line
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                    // 1 step back-right
                    if position - 8 + 1 > (rank - 1) * 8 {
                        new_pos = position - 8 + 1;
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
                // 1 step forward
                if position + 8 < 8 * 8 {
                    new_pos = position + 8;
                    potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));

                    // 1 step forward-left
                    if position + 8 - 1 > (rank + 1) * 8 {
                        new_pos = position + 8 - 1;
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                    // 1 step forward-right
                    if position + 8 + 1 < (rank + 2) * 8 - 1 {
                        new_pos = position + 8 + 1;
                        potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // 1 step left
                if position - 1 > (rank - 1) * 8 {
                    new_pos = position - 1;
                    potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                }
                // 1 step right
                if position + 1 < (rank + 1) * 8 {
                    new_pos = position + 1;
                    potential_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                } else {
                    return None;
                }

                let mut temp: Vec<String> = vec![]; // temporary storage of moves
                                                    // only allow taking friendly pieces
                for i in 0..potential_moves.len() {
                    let (file, rank) = Game::convert_from_notation(&potential_moves[i]);
                    // if there is a piece and it's hostile, make it capturable
                    if board[(rank * 8 + file) as usize].is_some() {
                        if self.get_piece_colour(rank * 8 + file) != Some(checking_for) {
                            temp.push((&potential_moves[i]).to_string());
                        }
                        // if there is no piece you can obviously go there
                    } else {
                        temp.push((&potential_moves[i]).to_string())
                    }
                }
                potential_moves = temp;
            }
            _ => panic!("{:?}", board[position as usize]),
        }

        return Some(potential_moves);
    }

    /// takes potential moves and removes the ones that would result in putting onself in check
    fn get_legal_moves(
        &self,
        _position: &str,
        checking_for: Colour,
        mut possible_moves: Vec<String>,
    ) -> Option<Vec<String>> {
        let mut legal_moves = vec![];

        for (index, to) in possible_moves.iter().enumerate() {
            // let mut fake_board = temp.clone();
            match self.make_fake_move(_position, to, checking_for, &possible_moves) {
                Some(fake_board) => {
                    if self.check_checker(checking_for, fake_board) == false {
                        legal_moves.push(to.to_string());
                    }
                }
                None => (),
            }
        }
        return Some(legal_moves);
    }

    // simulate move to see if it leaves the king in check
    fn make_fake_move(
        &self,
        _from: &str,
        _to: &str,
        checking_for: Colour,
        considered_moves: &Vec<String>,
    ) -> Option<[Option<Piece>; 64]> {
        if considered_moves.is_empty() {
            // return Some(self.get_game_state());
            return None;
        }
        let from = {
            let (file, rank) = Game::convert_from_notation(_from);
            rank * 8 + file
        };

        let to = {
            let (file, rank) = Game::convert_from_notation(_to);
            rank * 8 + file
        };

        let mut fake_board = self.board.clone(); // copy of the board
        match considered_moves {
            _vector => {
                fake_board[to as usize] = fake_board[from as usize];
                fake_board[from as usize] = None;
                return Some(fake_board);
            }
        }
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
                // println!("{}", rank);
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
    use core::panic;

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

    /*     #[test]
    fn colour_test() {
        let bpawn = Piece::Pawn(crate::Colour::Black);

        assert_eq!(bpawn.get_colour(), Colour::Black);
    } */

    #[test]
    fn legal_moves() {
        let game = Game::new();

        // let king = Piece::King(crate::Colour::White);

        // game.active_colour = Colour::White;

        // single space
        /*  if let Some(possible_moves) = game.get_possible_moves(game.board, "e2", game.active_colour)
        {
            let moves = game.get_legal_moves("e2", game.active_colour, possible_moves);
            println!("{:?}", moves);
        } else {
            panic!()
        } */

        let mut all_moves: Vec<Vec<String>> = vec![];
        // all pieces
        for (index, piece) in game.board.iter().enumerate() {
            if let Some(possible_moves) = game.get_possible_moves(
                game.board,
                &Game::convert_to_notation(index as i32),
                game.active_colour,
            ) {
                match piece {
                    Some(piece) => {
                        if let Some(temp) = (game.get_legal_moves(
                            &Game::convert_to_notation(index as i32),
                            game.active_colour,
                            possible_moves,
                        )) {
                            if !temp.is_empty() {
                                all_moves.push(temp);
                            }
                        }
                    }
                    None => continue,
                }
            }
        }
        println!("{:?}", all_moves);
    }

    #[test]
    fn try_notation() {
        println!("{:?}", Game::convert_to_notation(59));
    }

    #[test]
    fn try_make_move() {
        let mut game = Game::new();

        println!("{:?}", game.get_piece_colour(3));

        game.make_move("g2", "g4");
    }

    #[test]
    fn try_game_loop() {
        let mut game = Game::new();

        game.make_move("g2", "g4");
        game.make_move("e7", "e5");
        game.make_move("f2", "f3");
        game.make_move("d8", "h4");

        println!("{:?}", game);
        println!("{:?}", game.get_game_state());
    }

    #[test]
    fn promote_test() {
        let mut game = Game::new();

        game.set_promotion("e2", "q");
        println!("{:?}", game);
    }

    /* #[test]
    fn check_test() {
        let game = Game::new();

        let in_check = game.check_checker(Colour::White, game.board);

        println!("{}", in_check);

        assert_eq!(in_check, true)
    } */
}
