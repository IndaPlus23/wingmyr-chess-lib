# wingmyr-chess

Functions:
| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new() -> Game` | Initialises a new board with pieces. |
| `pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState>` | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game.|
| `pub fn get_game_state(&self) -> GameState` | Get the current game state. |
| `pub fn get_possible_moves( &self, board: [Option<Piece>; 64], _position: &str, checking_for: Colour, ) -> Option<Vec<String>>` | If a piece is standing on the given tile on a given board, return all possible new positions the piece can reach. |
| `fn get_legal_moves( &self, _position: &str, checking_for: Colour, mut possible_moves: Vec<String>, ) -> Option<Vec<String>>` | Takes a list of possible moves and returns all of those that don't put the king in check |
| `pub fn convert_from_notation(notation: &str) -> (i32, i32)` | Takes chess notation as a string (e.g. `"e1"`) and return a tuple `(file, rank)`. |
| `pub fn convert_to_notation(numeric_position: i32) -> String` | Takes an index on the board and returns it in chess notation |
| `pub fn get_piece_colour(&self, position: i32) -> Option<Colour>` | Takes a position on the board and returns the colour of that piece |
| `pub fn set_promotion(&mut self, _position: &str, _piece: &str)` | Takes a position as chess notation and a string that represents a piece `q-queen, r-rook, n-knight, b-bishop ` . Replaces the piece in the position with the inputed piece type |
| `pub fn checkmate_checker(&self, checking_for: Colour) -> bool` | Checks if there are any legal moves left for a given colour |

The formula for getting the numerical position on the board is `rank * 8 + file`.

Every piece is represented by an enum `Piece` containing each type of chess piece and an associated colour which is represented by the enum `Colour`
