# wingmyr-chess

Functions that currently work:
| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new() -> Game` | Initialises a new board with pieces. |
| `pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState>` | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game.|
| `pub fn get_game_state(&self) -> GameState` | Get the current game state. |
| `pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>>` | (Currently doesn't care if a piece is blocking the path) If a piece is standing on the given tile, return all possible new positions of that piece. |
| `pub fn convert_from_notation(notation: &str) -> (i32, i32)` | Takes chess notation as a string (e.g. `"e1"`) and return a tuple `(file, rank)`. |
| `pub fn convert_to_notation(numeric_position: i32) -> String` | Takes an index on the board and returns it in chess notation

The formula for getting the numerical position on the board is `rank * 8 + file`.
