use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use crate::Piece::Empty;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[derive(Default)]
struct GameDataInternal {
    game: Game,
}
type GameData = Mutex<GameDataInternal>;
#[tauri::command]
fn reset_game(app_handle: AppHandle) {
    let state = &mut app_handle.state::<GameData>();
    state.lock().unwrap().game = Game::default();
}

#[tauri::command]
fn get_game_board(app_handle: AppHandle) -> Game{
    app_handle.state::<GameData>().lock().unwrap().game.clone()
}

#[tauri::command]
fn play_move(app_handle: AppHandle,x:usize,y:usize,piece: Piece)->Game{
    let binding = app_handle.state::<GameData>();
    let game = &mut binding.lock().unwrap().game;
    game.play(x, y, &piece);
    game.clone()

}

#[tauri::command]
fn check_winner(app_handle: AppHandle)->Piece{
    app_handle.state::<GameData>().lock().unwrap().game.winner()
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(GameDataInternal {
                game: Game::default()
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![reset_game,get_game_board,play_move,check_winner])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Copy, Clone, Debug, PartialEq,serde::Serialize,serde::Deserialize)]
pub enum Piece {
    Empty,
    Cross,
    Nought,
}

const WINNING_NUM: i32 = 3;
#[derive(Clone,serde::Serialize)]
pub struct Game {
    board: Vec<Vec<Piece>>,
}
impl Game {
    fn play(&mut self, x: usize, y: usize, piece: &Piece) {
        if self.board[y][x] == Empty {
            self.board[y][x] = piece.clone()
        }
    }
    fn winner(&self) -> Piece {
        let board = &self.board;
        for row in 0..board.len() {
            let start_piece = board[row][0];
            if start_piece == Empty {
                continue;
            }
            let mut count = 1;
            for col in 1..board[row].len() {
                if start_piece == board[row][col] {
                    count += 1;
                }
            }
            if count == WINNING_NUM {
                return start_piece;
            }
        }
        for col in 0..board.len() {
            let start_piece = board[0][col];
            if start_piece == Empty {
                continue;
            }
            let mut count = 1;
            for row in 1..board.len() {
                if start_piece == board[row][col] {
                    count += 1;
                }
            }
            if count == WINNING_NUM {
                return start_piece;
            }
        }
        let start_piece = board[0][0];
        if start_piece != Empty {
            let mut count = 1;
            for diag in 1..board.len() {
                if start_piece == board[diag][diag] {
                    count += 1;
                }
            }
            if count == WINNING_NUM {
                return start_piece;
            }
        }
        let start_piece = board[0][board.len() - 1];
        if start_piece != Empty {
            let mut count = 1;
            for diag in 1..board.len() {
                if start_piece == board[diag][board.len() - diag - 1] {
                    count += 1
                }
            }
            if count == WINNING_NUM {
                return start_piece;
            }
        }
        Empty
    }
}
impl Default for Game {
    fn default() -> Self {
        Self {
            board: vec![vec![Empty; 3]; 3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Piece::{Cross, Nought};
    #[test]
    fn can_initialize_board() {
        let expected_board = vec![vec![Empty; 3]; 3];
        let game = Game::default();
        assert_eq!(game.board, expected_board)
    }
    #[test]
    fn can_make_move_when_space_is_free() {
        // We should be able to add a cross at Row 3, Column 0 when the space is free
        let mut expected_board = Game::default();
        expected_board.board[2][0] = Cross;

        let mut game_board = Game::default();
        game_board.play(0, 2, &Cross);

        assert_eq!(expected_board.board, game_board.board)
    }

    #[test]
    // The piece should not change when already occupied
    fn cannot_make_move_when_space_occupied() {
        let mut expected_board = Game::default();
        expected_board.board[2][0] = Nought;
        let mut game_board = Game::default();
        game_board.play(0, 2, &Nought);
        game_board.play(0, 2, &Cross);
        assert_eq!(expected_board.board, game_board.board)
    }

    #[test]
    fn calculates_winner_correctly_row() {
        let mut game_board = Game::default();
        game_board.board = Vec::from([vec![Empty; 3], vec![Nought; 3], vec![Empty; 3]]);
        assert_eq!(game_board.winner(), Nought)
    }

    #[test]
    fn calculates_winner_correctly_col() {
        let mut game_board = Game::default();
        game_board.board = Vec::from([Vec::from([Empty, Nought, Cross]), Vec::from([Empty, Cross, Cross]), Vec::from([Empty, Nought, Cross])]);
        assert_eq!(game_board.winner(), Cross)
    }

    #[test]
    fn calculates_winner_correctly_diagonal() {
        let mut game_board = Game::default();
        game_board.board = Vec::from([Vec::from([Empty, Nought, Cross]), Vec::from([Empty, Cross, Empty]), Vec::from([Cross, Nought, Cross])]);
        assert_eq!(game_board.winner(), Cross)
    }
}