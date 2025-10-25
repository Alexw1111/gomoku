// Gomoku AI - Tauri Backend
// Simple. Fast. Works.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ai_core;

use ai_core::{AI, Game};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MoveResult {
    row: usize,
    col: usize,
}

#[derive(Serialize, Deserialize)]
struct GameStatus {
    is_win: bool,
    winner: Option<i8>,
}

// Get AI move
#[tauri::command]
fn get_ai_move(
    board: Vec<Vec<i8>>,
    current_player: i8,
    depth: usize,
) -> Result<MoveResult, String> {
    let size = board.len();
    let game = Game::from_board(board, current_player);
    let mut ai = AI::new(size);

    match ai.find_move(&game, depth) {
        Some((row, col)) => Ok(MoveResult { row, col }),
        None => Err("No valid move found".to_string()),
    }
}

// Check win condition
#[tauri::command]
fn check_win(board: Vec<Vec<i8>>, row: usize, col: usize) -> Result<GameStatus, String> {
    let size = board.len();

    if row >= size || col >= size {
        return Err("Invalid position".to_string());
    }

    let player = board[row][col];
    if player == 0 {
        return Ok(GameStatus {
            is_win: false,
            winner: None,
        });
    }

    let dirs = [(0, 1), (1, 0), (1, 1), (1, -1)];

    for &(dr, dc) in &dirs {
        let count = 1
            + count_dir(&board, size, row, col, dr, dc, player)
            + count_dir(&board, size, row, col, -dr, -dc, player);

        if count >= 5 {
            return Ok(GameStatus {
                is_win: true,
                winner: Some(player),
            });
        }
    }

    Ok(GameStatus {
        is_win: false,
        winner: None,
    })
}

fn count_dir(board: &Vec<Vec<i8>>, size: usize, row: usize, col: usize, dr: i32, dc: i32, player: i8) -> usize {
    let mut count = 0;
    let (mut r, mut c) = (row as i32 + dr, col as i32 + dc);

    while r >= 0 && r < size as i32 && c >= 0 && c < size as i32 && board[r as usize][c as usize] == player {
        count += 1;
        r += dr;
        c += dc;
    }
    count
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ai_move, check_win])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
