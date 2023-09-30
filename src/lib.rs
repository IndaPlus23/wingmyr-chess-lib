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
enum Piece {
    King(Colour),
    Queen(Colour),
    Rook(Colour),
    Bishop(Colour),
    Knight(Colour),
    Pawn(Colour),
}

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    /* black: u64,
    white: u64, */
    active_colour: Colour,
    prev_colour: Colour,
    board: [Option<Piece>; 64],
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let mut bboard: [Option<Piece>; 64] = [None; 64]; // needs better name

        let board_template = "RNBKQBNRPPPPPPPP********************************PPPPPPPPRNBKQBNR "; // imagine a new line every 8 characters

        let mut current_colour = Colour::White; // 0 = black, 1 = white, might need to change this to include starting position idk

        /* for i in 0..64 {
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
        } */

        // in check scenario

        bboard[3] = Some(Piece::Rook(Colour::Black));
        bboard[11] = Some(Piece::Rook(Colour::White));
        bboard[27] = Some(Piece::King(Colour::White));

        // bboard[45] = Some(Piece::Pawn(Colour::White));

        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            /* black: 0,
            white: 1, */
            active_colour: Colour::/* Black, */ White,
            prev_colour: Colour::Black, // start value
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
            _ => panic!(),
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
        let from = {
            let (file, rank) = Game::convert_from_notation(_from);
            rank * 8 + file
        };

        let to = {
            let (file, rank) = Game::convert_from_notation(_to);
            rank * 8 + file
        };

        if self.board[from as usize] == Some(Piece::King(self.active_colour)) {
            // if you're trying to move the king
            if self.check_checker(/* _from, */ self.active_colour, self.board) == true
                && self.check_checker(/* _to, */ self.active_colour, self.board) == true
            {
                // if the king is in check and is not moving out of check
                eprintln!("you need to get out of check check");
                return Some(GameState::Check);
            } else if self.check_checker(/* _to, */ self.active_colour, self.board) == true {
                // if the king is not in check and moving into check
                eprintln!("you can't put yourself in check");
                return Some(GameState::Check);
            }
        }

        /* if self.get_game_state() == GameState::Check && self.check_checker(_to) == true {
            eprintln!("still in check");
            return Some(GameState::Check);
        } */

        match Game::get_possible_moves(&self, _from, self.active_colour) {
            Some(vector) => {
                if let Some(legal_moves) = self.get_legal_moves(_from, self.active_colour, vector) {
                    if legal_moves.contains(&_to.to_string()) {
                        self.board[to as usize] = self.board[from as usize];
                        self.board[from as usize] = None;
                    } else {
                        eprintln!("illegal move");
                        return Some(GameState::InProgress);
                    }
                }
            }
            None => return None,
        }

        /* if self.board[to as usize] == Some(Piece::Pawn(self.active_colour)) {
            if self.get_piece_colour(to) == Some(Colour::Black) && 55 < to && to <= 63{

                let mut line = String::new();
                let input = std::io::stdin().read_line(&mut line).unwrap();
            }
        } */

        let enemy_in_check: bool;

        if self.active_colour == Colour::White {
            enemy_in_check = self.check_checker(/* _to, */ Colour::Black, self.board);
        } else {
            enemy_in_check = self.check_checker(/* _to, */ Colour::White, self.board);
        }

        if self.active_colour == Colour::White {
            self.prev_colour = Colour::White;
            self.active_colour = Colour::Black
        } else {
            self.prev_colour = Colour::Black;
            self.active_colour = Colour::White
        }

        if enemy_in_check {
            self.state = GameState::Check;
            return Some(GameState::Check);
        }

        self.state = GameState::Check;
        return Some(GameState::InProgress);
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// takes position where king is standing or will be standing and returns true if that space is threatened
    fn check_checker(
        &self,
        /* _position: &str, */ checking_for: Colour,
        board: [Option<Piece>; 64],
    ) -> bool {
        // check for pieces that can reach the king directly

        // let file: i32; // = Game::convert_from_notation(_position);
        // let rank: i32;
        let mut position = 0; //needs a value for the compiler to shut up //= rank * 8 + file;

        // figure this out

        // if the current piece isn't the king, find king and run program
        // if board[position as usize] != Some(Piece::King(checking_for)) {
        for i in 0..board.len() {
            if board[i] == Some(Piece::King(checking_for)) {
                // let rank = ((i as i32) - file) / 8;
                // let file = (i;i as i32) - rank * 8;
                position = i as i32; /* rank * 8 + file; */
                break;
            }
            // }
            // position = rank * 8 + file;
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
                        &Game::convert_to_notation(piece as i32),
                        other_c,
                    ) {
                        println!(
                            "enemy {:?} currpos {:?}",
                            enemy_positions,
                            Game::convert_to_notation(position)
                        );
                        // println!("{:?} {:?}", enemy_positions, other_c);
                        if enemy_positions.contains(&Game::convert_to_notation(position)) {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    /* if self.board[position as usize] == Some(Piece::King(Colour::White)) {
        if self.board[(position + 8 - 1) as usize] == Some(Piece::Pawn(Colour::Black))
            || self.board[(position + 8 + 1) as usize] == Some(Piece::Pawn(Colour::Black))
        {
            return true; // at least one pawn is a threat
        }
    }

    if self.board[position as usize] == Some(Piece::King(Colour::Black)) {
        if self.board[(position - 8 - 1) as usize] == Some(Piece::Pawn(Colour::White))
            || self.board[(position - 8 + 1) as usize] == Some(Piece::Pawn(Colour::White))
        {
            return true; // at least one pawn is a threat
        }
    }

    //Diagonal movement
    // forward-right
    for i in 0..8 {
        position = (rank + i) * 8 + file + i;

        let current_file = position - (rank + i) * 8;
        let current_rank = (position - (file + i)) / 8;

        if (0 <= current_file && current_file <= 7) && (0 <= current_rank && current_rank <= 7)
        {
            // check if something can reach
            if self.board[position as usize].is_some() {
                if self.get_piece_colour(position) != Some(checking_for) {
                    match self.board[position as usize] {
                        Some(Piece::Bishop(_colour)) | Some(Piece::Queen(_colour)) => {
                            return true
                        } // space is threatened
                        _ => continue, // keep looking for threats on this diagonal
                    }
                }
            }
        } else {
            // no threat on this diagonal
            break;
        }
    }
    //forward-left
    for i in 0..8 {
        position = (rank + i) * 8 + file - i;

        let current_file = position - (rank + i) * 8;
        let current_rank = (position - (file - i)) / 8;

        if (0 <= current_file && current_file <= 7) && (0 <= current_rank && current_rank <= 7)
        {
            // check if something can reach
            if self.board[position as usize].is_some() {
                if self.get_piece_colour(position) != Some(checking_for) {
                    match self.board[position as usize] {
                        Some(Piece::Bishop(_colour)) | Some(Piece::Queen(_colour)) => {
                            return true
                        } // space is not threatened
                        _ => continue, // keep looking for threats on this diagonal
                    }
                }
            }
        } else {
            // no threat on this diagonal
            break;
        }
    }
    //backward-left
    for i in 0..8 {
        position = (rank - i) * 8 + file - i;

        let current_file = position - (rank - i) * 8;
        let current_rank = (position - (file - i)) / 8;

        if (0 <= current_file && current_file <= 7) && (0 <= current_rank && current_rank <= 7)
        {
            // check if something can reach
            if self.board[position as usize].is_some() {
                if self.get_piece_colour(position) != Some(checking_for) {
                    match self.board[position as usize] {
                        Some(Piece::Bishop(_colour)) | Some(Piece::Queen(_colour)) => {
                            return true
                        } // space is not threatened
                        _ => continue, // keep looking for threats on this diagonal
                    }
                }
            }
        } else {
            // no theat on this diagonal
            break;
        }
    }
    //backward-right
    for i in 0..8 {
        position = (rank - i) * 8 + file + i;

        let current_file = position - (rank - i) * 8;
        let current_rank = (position - (file + i)) / 8;

        if (0 <= current_file && current_file <= 7) && (0 <= current_rank && current_rank <= 7)
        {
            // check if something can reach
            if self.board[position as usize].is_some() {
                if self.get_piece_colour(position) != Some(checking_for) {
                    match self.board[position as usize] {
                        Some(Piece::Bishop(_colour)) | Some(Piece::Queen(_colour)) => {
                            return true
                        } // space is not threatened
                        _ => continue, // keep looking for threats on this diagonal
                    }
                }
            }
        } else {
            // no threat here
            break;
        }
    }
    // up
    let mut new_pos: i32;
    for y in (0..rank).rev() {
        new_pos = y * 8 + file;

        // check if something is threatening tile
        if self.board[new_pos as usize].is_some() {
            if self.get_piece_colour(new_pos) == Some(checking_for) {
                // break if friendly piece
                if self.board[new_pos as usize] == Some(Piece::King(checking_for)) {
                    continue;
                } // ignore self
                break; // only friends
            } else {
                match self.board[new_pos as usize] {
                    Some(Piece::Queen(_colour)) | Some(Piece::Rook(_colour)) => return true, // rook or queen is threat
                    None => continue,
                    _ => break, // non-threatening piece in the way
                }
            }
        } */
    /////////////////////////////////
    /*  if file <= new_pos /* && new_pos <= 7 * 8 + file  */{
            legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
        }
    }*/
    /* // down
        for y in rank..8{
            new_pos = y*8+file;

            // check if something's in the way
            if self.board[new_pos as usize].is_some() {
                if self.get_piece_colour(new_pos) == Some(self.active_colour) {
                    // break if friendly piece
                    if self.board[new_pos as usize] == Some(Piece::King(self.active_colour)){continue}
                    break;
                } else {
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                    break;
                }
            }

            if /*file <= new_pos  &&*/ new_pos <= 7 * 8 + file  {
                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
            }

        }

        // horizontal movement

        // left
        for x in (0..file).rev(){
            new_pos = rank*8+x;

            if self.board[new_pos as usize].is_some() {
                if self.get_piece_colour(new_pos) == Some(self.active_colour) {
                    // break if friendly piece
                    if self.board[new_pos as usize] == Some(Piece::King(self.active_colour)){continue}
                    break;
                } else {
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                    break;
                }
            }
            if rank * 8 <= new_pos /* && x_left < left_file_range */ {
                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));}
        }

        // right
        for x in file..8{
            new_pos = rank*8+x;

            if self.board[new_pos as usize].is_some() {
                if self.get_piece_colour(new_pos) == Some(self.active_colour) {
                    // break if friendly piece
                    if self.board[new_pos as usize] == Some(Piece::King(self.active_colour)){continue}
                    break;
                } else {
                    legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                    break;
                }
            }
            if /*rank * 8 <= new_pos  &&*/ new_pos < (rank+1)*8  {
                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));}
        }

        return false; // no threat was found
    }*/

    pub fn get_possible_moves(&self, _position: &str, checking_for: Colour) -> Option<Vec<String>> {
        // reminder: position is "<file><rank>"
        // there's probably a better solution

        let (file, rank) = Game::convert_from_notation(_position);
        let position = rank * 8 + file; // formula for getting position in the 1D array

        match self.board[position as usize] {
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

        let mut legal_moves: Vec<String> = vec![];

        let mut new_pos: i32;

        match self.board[position as usize] {
            ////// change to be current colour later ///////
            Some(Piece::Pawn(Colour::White)) => {
                if 8 < position && position <= 16 {
                    // if pawn hasn't been moved
                    new_pos = position + 8;
                    if self.board[(position + 8) as usize].is_none() {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));

                        new_pos = position + 16; // two steps forward
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                } else {
                    new_pos = position + 8;
                    if self.board[new_pos as usize].is_none() {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
                // check for capturable pieces
                if self.board[(position + 8 + 1) as usize].is_some() {
                    if self.get_piece_colour(position + 8 - 1) != Some(checking_for) {
                        legal_moves
                            .push(format!("{}", Game::convert_to_notation(position + 8 - 1)));
                    }
                }
                if self.board[(position + 8 - 1) as usize].is_some() {
                    if self.get_piece_colour(position + 8 + 1) != Some(checking_for) {
                        legal_moves
                            .push(format!("{}", Game::convert_to_notation(position + 8 + 1)));
                    }
                }
            }
            Some(Piece::Pawn(Colour::Black)) => {
                if 48 < position && position <= 56 {
                    // if pawn hasn't been moved
                    new_pos = position - 8;
                    if self.board[(position - 8) as usize].is_none() {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));

                        new_pos = position - 16;
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    }
                } else {
                    new_pos = position - 8;
                    if self.board[new_pos as usize].is_none() {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                if self.board[(position - 8 + 1) as usize].is_some() {
                    if self.get_piece_colour(position + 8 - 1) != Some(checking_for) {
                        legal_moves
                            .push(format!("{}", Game::convert_to_notation(position - 8 - 1)));
                    }
                }

                if self.board[(position - 8 - 1) as usize].is_some() {
                    if self.get_piece_colour(position + 8 + 1) != Some(checking_for) {
                        legal_moves
                            .push(format!("{}", Game::convert_to_notation(position - 8 + 1)));
                    }
                }
            }
            Some(Piece::Rook(_colour)) => {
                /* let rank_range = 64 / 8;

                let left_file_range = 8 - position - rank * 8;
                let right_file_range = 8 - left_file_range; */

                // vertical movement
                // up
                for y in (0..rank).rev() {
                    new_pos = y * 8 + file;

                    // check if something's in the way
                    if self.board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if self.board[new_pos as usize] == Some(Piece::Rook(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }

                    if file <= new_pos
                    /* && new_pos <= 7 * 8 + file  */
                    {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
                // down
                for y in rank..8 {
                    new_pos = y * 8 + file;

                    // check if something's in the way
                    if self.board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if self.board[new_pos as usize] == Some(Piece::Rook(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }

                    if
                    /*file <= new_pos  &&*/
                    new_pos <= 7 * 8 + file {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // horizontal movement

                // left
                for x in (0..file).rev() {
                    new_pos = rank * 8 + x;

                    if self.board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if self.board[new_pos as usize] == Some(Piece::Rook(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }
                    if rank * 8 <= new_pos
                    /* && x_left < left_file_range */
                    {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // right
                for x in file..8 {
                    new_pos = rank * 8 + x;

                    if self.board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if self.board[new_pos as usize] == Some(Piece::Rook(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }
                    if
                    /*rank * 8 <= new_pos  &&*/
                    new_pos < (rank + 1) * 8 {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
            }
            // painful
            Some(Piece::Bishop(_colour)) => {
                // probably a better way to do this
                for i in 0..8 {
                    new_pos = (rank + i) * 8 + file + i;

                    /* if 0 <= new_pos && new_pos <= 63{

                    } */
                    let current_file = new_pos - (rank + i) * 8;
                    let current_rank = (new_pos - (file + i)) / 8;

                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    /* (0 <= new_pos && new_pos <= 7)
                        || (56 <= new_pos && new_pos <= 63)
                        || (new_pos % 8 == 7 || new_pos % 8 == 0) */
                    /* && ((rank + i) * 8 + (file + i)) / 8 > 8  */
                    {
                        // check if something's in the way
                        if self.board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (self.board[new_pos as usize])
                                    == Some(Piece::Bishop(checking_for))
                                {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file + i, rank + i)
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                for i in 0..8 {
                    new_pos = (rank + i) * 8 + file - i;
                    /* if 0 <= new_pos && new_pos <= 63{

                    } */
                    // check if something's in the way
                    let current_file = new_pos - (rank + i) * 8;
                    let current_rank = (new_pos - (file - i)) / 8;

                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    /* (0 <= new_pos && new_pos <= 7)
                        || (56 <= new_pos && new_pos <= 63)
                        || (new_pos % 8 == 7 || new_pos % 8 == 0) */
                    /* && ((rank + i) * 8 + (file + i)) / 8 > 8  */
                    {
                        if self.board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if self.board[new_pos as usize] == Some(Piece::Bishop(checking_for))
                                {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file - i, rank + i)
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                for i in 0..8 {
                    new_pos = (rank - i) * 8 + file - i;
                    /* if 0 <= new_pos && new_pos <= 63{

                    } */

                    let current_file = new_pos - (rank - i) * 8;
                    let current_rank = (new_pos - (file - i)) / 8;
                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    /* (0 <= new_pos && new_pos <= 7)
                        || (56 <= new_pos && new_pos <= 63)
                        || (new_pos % 8 == 7 || new_pos % 8 == 0) */
                    /* && ((rank + i) * 8 + (file + i)) / 8 > 8  */
                    {
                        // check if something's in the way
                        if self.board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (self.board[new_pos as usize])
                                    == Some(Piece::Bishop(checking_for))
                                {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file - i, rank - i)
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                eprintln!("backward-right");
                for i in 0..8 {
                    new_pos = (rank - i) * 8 + file + i;
                    /* if 0 <= new_pos && new_pos <= 63{

                    } */
                    let current_file = new_pos - (rank - i) * 8;
                    let current_rank = (new_pos - (file + i)) / 8;
                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    /* (0 <= new_pos && new_pos <= 7)
                        || (56 <= new_pos && new_pos <= 63)
                        || (new_pos % 8 == 7 || new_pos % 8 == 0) */
                    /* && ((rank + i) * 8 + (file + i)) / 8 > 8  */
                    {
                        // check if something's in the way
                        if self.board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (self.board[new_pos as usize])
                                    == Some(Piece::Bishop(checking_for))
                                {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file + i, rank - i)
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
            }

            Some(Piece::Queen(_colour)) => {
                //Diagonal movement
                for i in 0..8 {
                    new_pos = (rank + i) * 8 + file + i;

                    /* if 0 <= new_pos && new_pos <= 63{

                    } */
                    let current_file = new_pos - (rank + i) * 8;
                    let current_rank = (new_pos - (file + i)) / 8;

                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    /* (0 <= new_pos && new_pos <= 7)
                        || (56 <= new_pos && new_pos <= 63)
                        || (new_pos % 8 == 7 || new_pos % 8 == 0) */
                    /* && ((rank + i) * 8 + (file + i)) / 8 > 8  */
                    {
                        // check if something's in the way
                        if self.board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (self.board[new_pos as usize])
                                    == Some(Piece::Queen(checking_for))
                                {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file + i, rank + i)
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                for i in 0..8 {
                    new_pos = (rank + i) * 8 + file - i;
                    /* if 0 <= new_pos && new_pos <= 63{

                    } */
                    // check if something's in the way
                    let current_file = new_pos - (rank + i) * 8;
                    let current_rank = (new_pos - (file - i)) / 8;

                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    /* (0 <= new_pos && new_pos <= 7)
                        || (56 <= new_pos && new_pos <= 63)
                        || (new_pos % 8 == 7 || new_pos % 8 == 0) */
                    /* && ((rank + i) * 8 + (file + i)) / 8 > 8  */
                    {
                        if self.board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if self.board[new_pos as usize] == Some(Piece::Queen(checking_for))
                                {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file - i, rank + i)
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                for i in 0..8 {
                    new_pos = (rank - i) * 8 + file - i;
                    /* if 0 <= new_pos && new_pos <= 63{

                    } */

                    let current_file = new_pos - (rank - i) * 8;
                    let current_rank = (new_pos - (file - i)) / 8;
                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    /* (0 <= new_pos && new_pos <= 7)
                        || (56 <= new_pos && new_pos <= 63)
                        || (new_pos % 8 == 7 || new_pos % 8 == 0) */
                    /* && ((rank + i) * 8 + (file + i)) / 8 > 8  */
                    {
                        // check if something's in the way
                        if self.board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (self.board[new_pos as usize])
                                    == Some(Piece::Queen(checking_for))
                                {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file - i, rank - i)
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }
                //backward-right
                for i in 0..8 {
                    new_pos = (rank - i) * 8 + file + i;
                    /* if 0 <= new_pos && new_pos <= 63{

                    } */
                    let current_file = new_pos - (rank - i) * 8;
                    let current_rank = (new_pos - (file + i)) / 8;
                    if (0 <= current_file && current_file <= 7)
                        && (0 <= current_rank && current_rank <= 7)
                    /* (0 <= new_pos && new_pos <= 7)
                        || (56 <= new_pos && new_pos <= 63)
                        || (new_pos % 8 == 7 || new_pos % 8 == 0) */
                    /* && ((rank + i) * 8 + (file + i)) / 8 > 8  */
                    {
                        // check if something's in the way
                        if self.board[new_pos as usize].is_some() {
                            if self.get_piece_colour(new_pos) == Some(checking_for) {
                                if (self.board[new_pos as usize])
                                    == Some(Piece::Queen(checking_for))
                                {
                                    continue;
                                } // ignore self
                                  // break if friendly piece
                                break;
                            } else {
                                legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                                break;
                            }
                        }

                        // eprintln!("({}, {})", file + i, rank - i)
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)))
                    } else {
                        break;
                    }
                }

                // vertucak movement

                for y in (0..rank).rev() {
                    new_pos = y * 8 + file;

                    // check if something's in the way
                    if self.board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if self.board[new_pos as usize] == Some(Piece::Queen(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }

                    if file <= new_pos
                    /* && new_pos <= 7 * 8 + file  */
                    {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
                // down
                for y in rank..8 {
                    new_pos = y * 8 + file;

                    // check if something's in the way
                    if self.board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if self.board[new_pos as usize] == Some(Piece::Queen(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }

                    if
                    /*file <= new_pos  &&*/
                    new_pos <= 7 * 8 + file {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // horizontal movement

                // left
                for x in (0..file).rev() {
                    new_pos = rank * 8 + x;

                    if self.board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if self.board[new_pos as usize] == Some(Piece::Queen(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }
                    if rank * 8 <= new_pos
                    /* && x_left < left_file_range */
                    {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }

                // right
                for x in file..8 {
                    new_pos = rank * 8 + x;

                    if self.board[new_pos as usize].is_some() {
                        if self.get_piece_colour(new_pos) == Some(checking_for) {
                            // break if friendly piece
                            if self.board[new_pos as usize] == Some(Piece::Queen(checking_for)) {
                                continue;
                            }
                            break;
                        } else {
                            legal_moves.push(format!("{}", Game::convert_to_notation(new_pos))); // add move and break if enemy piece
                            break;
                        }
                    }
                    if
                    /*rank * 8 <= new_pos  &&*/
                    new_pos < (rank + 1) * 8 {
                        legal_moves.push(format!("{}", Game::convert_to_notation(new_pos)));
                    }
                }
            }

            Some(Piece::Knight(_colour)) => {
                // forward
                if (rank + 2) * 8 < 7 * 8 + file {
                    // right
                    if file + 1 < 8 {
                        legal_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank + 2) * 8 + file + 1)
                        ));
                    }
                    // left
                    if file - 1 >= 0 {
                        legal_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank + 2) * 8 + file - 1)
                        ));
                    }
                }
                // backwards
                if (rank - 2) * 8 > 0 {
                    //right
                    if file + 1 < 8 {
                        legal_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank - 2) * 8 + file + 1)
                        ));
                    }
                    // left
                    if file - 1 >= 0 {
                        legal_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank - 2) * 8 + file - 1)
                        ));
                    }
                }
                // right
                if file + 2 < 8 {
                    // forward
                    if (rank + 1) * 8 < 7 * 8 + file + 2 {
                        legal_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank + 1) * 8 + file + 2)
                        ));
                    }
                    // backward
                    if (rank - 1) * 8 < 7 * 8 + file + 2 {
                        legal_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank - 1) * 8 + file + 2)
                        ));
                    }
                }
                // left
                if file - 2 >= 0 {
                    //backward
                    if (rank - 1) * 8 < 7 * 8 + file - 2 {
                        legal_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank - 1) * 8 + file - 2)
                        ));
                    }
                    // forward
                    if (rank + 1) * 8 < 7 * 8 + file - 2 {
                        legal_moves.push(format!(
                            "{}",
                            Game::convert_to_notation((rank + 1) * 8 + file - 2)
                        ));
                    }
                }
                // remove moves that collide with friendly pieces
                for i in 0..legal_moves.len() {
                    let (file, rank) = Game::convert_from_notation(&legal_moves[i]);
                    if self.board[(rank * 8 + file) as usize].is_some() {
                        if self.get_piece_colour(rank * 8 + file) == Some(checking_for) {
                            legal_moves.remove(i);
                        }
                    }
                }
            }

            Some(Piece::King(_colour)) => {
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

                // remove moves that collide with friendly pieces
                for i in 0..legal_moves.len() {
                    let (file, rank) = Game::convert_from_notation(&legal_moves[i]);
                    if self.board[(rank * 8 + file) as usize].is_some() {
                        if self.get_piece_colour(rank * 8 + file) == Some(checking_for) {
                            legal_moves.remove(i);
                        }
                    }
                }
            }
            _ => panic!("{:?}", self.board[position as usize]),
        }

        // remove all moves that would put oneself in check
        /* for (index, _move) in legal_moves.iter().enumerate() {
            if self.make_fake_move(_position, &_move, checking_for, &legal_moves)
                == Some(GameState::Check)
            {
                legal_moves.to_owned().remove(index);
            }
        } */

        return Some(legal_moves);
    }

    /// takes potential moves and removes the ones that would result in putting onself in check
    fn get_legal_moves(
        &self,
        _position: &str,
        checking_for: Colour,
        mut possible_moves: Vec<String>,
    ) -> Option<Vec<String>> {
        /*let mut legal_moves   match self.get_possible_moves(_position, checking_for) {
            Some(vec) => vec,
            None => {
                eprint!("no legal moves");
                return None;
            }
        }; */
        // make a copy of the moves if it gives you out of range errors
        for (index, to) in possible_moves.clone().iter().enumerate() {
            // let mut fake_board = temp.clone();
            match self.make_fake_move(_position, to, checking_for, &possible_moves) {
                Some(fake_board) => {
                    if self.check_checker(checking_for, fake_board) == true {
                        println!("jabadabadoo");
                        possible_moves.remove(index);
                    }
                }
                None => (),
            }

            /*  if self
                .make_fake_move(_position, to, checking_for, &legal_moves)
                .is_some()
            {
                if self.check_checker(
                    checking_for,
                    self.make_fake_move(_position, to, checking_for, &legal_moves),
                ) == true
                {
                    legal_moves.remove(index);
                }
            } */
        }
        return Some(possible_moves);
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
                // if vector.contains(&_to.to_string()) {
                // this shouldn always be true but who knows
                fake_board[to as usize] = fake_board[from as usize];
                fake_board[from as usize] = None;
                return Some(fake_board);
                /*// }  else {
                    // return Some(GameState::InProgress); // illegal move
                    return None;
                } */
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

        if let Some(possible_moves) = game.get_possible_moves("e2", game.active_colour) {
            let moves = game.get_legal_moves("e2", game.active_colour, possible_moves);
            println!("{:?}", moves);
        } else {
            panic!()
        }
    }

    #[test]
    fn try_notation() {
        println!("{:?}", Game::convert_from_notation("a1"));
    }

    #[test]
    fn try_make_move() {
        let mut game = Game::new();

        game.make_move("e2", "e4");

        println!("{:?}", game)
    }
    #[test]
    fn check_test() {
        let game = Game::new();

        let in_check = game.check_checker(Colour::White, game.board);

        println!("{}", in_check);

        assert_eq!(in_check, true)
    }
}
