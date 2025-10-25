// Gomoku AI Core - Simple, Fast, Works.
// Dynamic board size support

use std::collections::HashMap;

const EMPTY: i8 = 0;
const INF: i32 = 1_000_000;
const WIN: i32 = 100_000;

// Zobrist hashing
#[derive(Clone)]
struct ZobristHash {
    table: Vec<Vec<[u64; 2]>>,
    hash: u64,
    size: usize,
}

impl ZobristHash {
    fn new(size: usize) -> Self {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let state = RandomState::new();
        let mut table = vec![vec![[0u64; 2]; size]; size];

        for i in 0..size {
            for j in 0..size {
                for k in 0..2 {
                    let mut hasher = state.build_hasher();
                    (i, j, k).hash(&mut hasher);
                    table[i][j][k] = hasher.finish();
                }
            }
        }

        ZobristHash { table, hash: 0, size }
    }

    fn toggle(&mut self, row: usize, col: usize, player: i8) {
        self.hash ^= self.table[row][col][(player - 1) as usize];
    }

    fn get_hash(&self) -> u64 {
        self.hash
    }
}

#[derive(Clone)]
pub struct Game {
    board: Vec<Vec<i8>>,
    size: usize,
    current: i8,
    zobrist: ZobristHash,
}

impl Game {
    pub fn from_board(board: Vec<Vec<i8>>, current_player: i8) -> Self {
        let size = board.len();
        let mut game = Game {
            board: vec![vec![EMPTY; size]; size],
            size,
            current: current_player,
            zobrist: ZobristHash::new(size),
        };

        for i in 0..size {
            for j in 0..size {
                game.board[i][j] = board[i][j];
                if board[i][j] != EMPTY {
                    game.zobrist.toggle(i, j, board[i][j]);
                }
            }
        }

        game
    }

    fn make_move(&mut self, row: usize, col: usize) -> bool {
        if row >= self.size || col >= self.size || self.board[row][col] != EMPTY {
            return false;
        }
        self.board[row][col] = self.current;
        self.zobrist.toggle(row, col, self.current);
        self.current = 3 - self.current;
        true
    }

    fn undo_move(&mut self, row: usize, col: usize, player: i8) {
        self.board[row][col] = EMPTY;
        self.zobrist.toggle(row, col, player);
        self.current = player;
    }

    fn check_win(&self, row: usize, col: usize) -> bool {
        let player = self.board[row][col];
        if player == EMPTY {
            return false;
        }

        let dirs = [(0, 1), (1, 0), (1, 1), (1, -1)];

        for &(dr, dc) in &dirs {
            let count = 1
                + self.count_dir(row, col, dr, dc, player)
                + self.count_dir(row, col, -dr, -dc, player);

            if count >= 5 {
                return true;
            }
        }
        false
    }

    fn count_dir(&self, row: usize, col: usize, dr: i32, dc: i32, player: i8) -> usize {
        let mut count = 0;
        let (mut r, mut c) = (row as i32 + dr, col as i32 + dc);

        while r >= 0 && r < self.size as i32 && c >= 0 && c < self.size as i32
              && self.board[r as usize][c as usize] == player {
            count += 1;
            r += dr;
            c += dc;
        }
        count
    }

    fn evaluate(&self) -> i32 {
        let current_score = self.evaluate_player(self.current);
        let opponent_score = self.evaluate_player(3 - self.current);
        current_score - (opponent_score as f32 * 1.1) as i32
    }

    fn evaluate_player(&self, player: i8) -> i32 {
        let mut score = 0;
        for row in 0..self.size {
            for col in 0..self.size {
                if self.board[row][col] == player {
                    score += self.eval_position(row, col, player);
                }
            }
        }
        score
    }

    fn eval_position(&self, row: usize, col: usize, player: i8) -> i32 {
        let dirs = [(0, 1), (1, 0), (1, 1), (1, -1)];
        let mut score = 0;

        for &(dr, dc) in &dirs {
            let (count, open) = self.eval_line(row, col, dr, dc, player);
            score += match (count, open) {
                (5.., _) => WIN,
                (4, 2) => 10000,
                (4, 1) => 5000,
                (3, 2) => 5000,
                (3, 1) => 500,
                (2, 2) => 500,
                (2, 1) => 50,
                _ => 10,
            };
        }
        score
    }

    fn eval_line(&self, row: usize, col: usize, dr: i32, dc: i32, player: i8) -> (usize, usize) {
        let mut count = 1;
        let mut open = 0;

        let (mut r, mut c) = (row as i32 + dr, col as i32 + dc);
        while r >= 0 && r < self.size as i32 && c >= 0 && c < self.size as i32 {
            if self.board[r as usize][c as usize] == player {
                count += 1;
            } else if self.board[r as usize][c as usize] == EMPTY {
                open += 1;
                break;
            } else {
                break;
            }
            r += dr;
            c += dc;
        }

        let (mut r, mut c) = (row as i32 - dr, col as i32 - dc);
        while r >= 0 && r < self.size as i32 && c >= 0 && c < self.size as i32 {
            if self.board[r as usize][c as usize] == player {
                count += 1;
            } else if self.board[r as usize][c as usize] == EMPTY {
                open += 1;
                break;
            } else {
                break;
            }
            r -= dr;
            c -= dc;
        }

        (count, open)
    }

    fn has_neighbor(&self, row: usize, col: usize) -> bool {
        for dr in -2..=2 {
            for dc in -2..=2 {
                if dr == 0 && dc == 0 { continue; }
                let r = row as i32 + dr;
                let c = col as i32 + dc;
                if r >= 0 && r < self.size as i32 && c >= 0 && c < self.size as i32 {
                    if self.board[r as usize][c as usize] != EMPTY {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn score_move(&self, row: usize, col: usize) -> i32 {
        let mut score = 0;
        let dirs = [(0, 1), (1, 0), (1, 1), (1, -1)];

        let mut temp_board = self.board.clone();
        temp_board[row][col] = self.current;

        for &(dr, dc) in &dirs {
            let (count, open) = Self::eval_line_static(&temp_board, self.size, row, col, dr, dc, self.current);
            score += match (count, open) {
                (5.., _) => 50000,
                (4, _) => 10000,
                (3, 2) => 5000,
                (3, 1) => 1000,
                (2, 2) => 500,
                _ => 10,
            };
        }

        temp_board[row][col] = 3 - self.current;
        for &(dr, dc) in &dirs {
            let (count, open) = Self::eval_line_static(&temp_board, self.size, row, col, dr, dc, 3 - self.current);
            score += match (count, open) {
                (5.., _) => 50000,
                (4, _) => 12000,
                (3, 2) => 6000,
                (3, 1) => 1100,
                (2, 2) => 550,
                _ => 10,
            };
        }

        score
    }

    fn eval_line_static(board: &Vec<Vec<i8>>, size: usize, row: usize, col: usize, dr: i32, dc: i32, player: i8) -> (usize, usize) {
        let mut count = 1;
        let mut open = 0;

        let (mut r, mut c) = (row as i32 + dr, col as i32 + dc);
        while r >= 0 && r < size as i32 && c >= 0 && c < size as i32 {
            if board[r as usize][c as usize] == player {
                count += 1;
            } else if board[r as usize][c as usize] == EMPTY {
                open += 1;
                break;
            } else {
                break;
            }
            r += dr;
            c += dc;
        }

        let (mut r, mut c) = (row as i32 - dr, col as i32 - dc);
        while r >= 0 && r < size as i32 && c >= 0 && c < size as i32 {
            if board[r as usize][c as usize] == player {
                count += 1;
            } else if board[r as usize][c as usize] == EMPTY {
                open += 1;
                break;
            } else {
                break;
            }
            r -= dr;
            c -= dc;
        }

        (count, open)
    }
}

#[derive(Clone, Copy)]
struct TTEntry {
    depth: i32,
    score: i32,
    flag: TTFlag,
    best_move: Option<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq)]
enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}

pub struct AI {
    tt: HashMap<u64, TTEntry>,
    killer_moves: Vec<[(usize, usize); 2]>,
    history: Vec<Vec<i32>>,
    size: usize,
}

impl AI {
    pub fn new(size: usize) -> Self {
        AI {
            tt: HashMap::new(),
            killer_moves: vec![[(size/2, size/2); 2]; 32],
            history: vec![vec![0; size]; size],
            size,
        }
    }

    pub fn find_move(&mut self, game: &Game, depth: usize) -> Option<(usize, usize)> {
        let moves = self.get_ordered_moves_phase1(game, 0, None);

        for &(row, col) in &moves {
            let mut g = game.clone();
            g.make_move(row, col);
            if g.check_win(row, col) {
                return Some((row, col));
            }
        }

        for &(row, col) in &moves {
            let mut g = game.clone();
            g.board[row][col] = 3 - g.current;
            if g.check_win(row, col) {
                return Some((row, col));
            }
            g.board[row][col] = EMPTY;
        }

        let mut best_move = None;
        for d in 1..=depth {
            if let Some(mv) = self.search_depth(game, d) {
                best_move = Some(mv);
            }
        }

        best_move
    }

    fn search_depth(&mut self, game: &Game, depth: usize) -> Option<(usize, usize)> {
        let mut best_move = None;
        let mut alpha = -INF;
        let beta = INF;

        let moves = self.get_ordered_moves_phase1(game, 0, None);

        for &(row, col) in &moves {
            let mut g = game.clone();
            let player = g.current;
            g.make_move(row, col);

            let score = -self.negamax(&mut g, depth as i32 - 1, -beta, -alpha, 1);

            g.undo_move(row, col, player);

            if score > alpha {
                alpha = score;
                best_move = Some((row, col));
            }
        }

        best_move
    }

    fn negamax(&mut self, game: &mut Game, depth: i32, mut alpha: i32, beta: i32, ply: usize) -> i32 {
        let hash = game.zobrist.get_hash();
        let mut tt_move: Option<(usize, usize)> = None;

        if let Some(entry) = self.tt.get(&hash) {
            tt_move = entry.best_move;

            if entry.depth >= depth {
                match entry.flag {
                    TTFlag::Exact => return entry.score,
                    TTFlag::LowerBound => alpha = alpha.max(entry.score),
                    TTFlag::UpperBound => {}
                }
                if alpha >= beta {
                    return entry.score;
                }
            }
        }

        if depth <= 0 {
            return game.evaluate();
        }

        let moves = self.get_ordered_moves_phase1(game, ply, tt_move);
        if moves.is_empty() {
            return 0;
        }

        let mut best_score = -INF;
        let mut best_move: Option<(usize, usize)> = None;
        let alpha_orig = alpha;

        for &(row, col) in &moves {
            let player = game.current;
            game.make_move(row, col);

            if game.check_win(row, col) {
                game.undo_move(row, col, player);
                return WIN - (depth as i32);
            }

            let score = -self.negamax(game, depth - 1, -beta, -alpha, ply + 1);
            game.undo_move(row, col, player);

            if score > best_score {
                best_score = score;
                best_move = Some((row, col));
            }

            alpha = alpha.max(score);

            if alpha >= beta {
                if let Some(mv) = best_move {
                    self.update_killers(ply, mv);
                    self.history[mv.0][mv.1] += (depth * depth) as i32;
                }
                break;
            }
        }

        let flag = if best_score <= alpha_orig {
            TTFlag::UpperBound
        } else if best_score >= beta {
            TTFlag::LowerBound
        } else {
            TTFlag::Exact
        };

        self.tt.insert(hash, TTEntry {
            depth,
            score: best_score,
            flag,
            best_move,
        });

        best_score
    }

    fn update_killers(&mut self, ply: usize, mv: (usize, usize)) {
        let ply = ply.min(31);
        if self.killer_moves[ply][0] != mv {
            self.killer_moves[ply][1] = self.killer_moves[ply][0];
            self.killer_moves[ply][0] = mv;
        }
    }

    fn get_ordered_moves_phase1(&self, game: &Game, ply: usize, tt_move: Option<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut moves_with_scores = Vec::new();

        let mut has_piece = false;
        for row in 0..game.size {
            for col in 0..game.size {
                if game.board[row][col] != EMPTY {
                    has_piece = true;
                    break;
                }
            }
            if has_piece { break; }
        }

        if !has_piece {
            return vec![(game.size / 2, game.size / 2)];
        }

        let ply = ply.min(31);

        for row in 0..game.size {
            for col in 0..game.size {
                if game.board[row][col] == EMPTY && game.has_neighbor(row, col) {
                    let mut score = game.score_move(row, col);

                    if Some((row, col)) == tt_move {
                        score += 10_000_000;
                    }

                    if self.killer_moves[ply][0] == (row, col) {
                        score += 1_000_000;
                    } else if self.killer_moves[ply][1] == (row, col) {
                        score += 500_000;
                    }

                    score += self.history[row][col];

                    moves_with_scores.push(((row, col), score));
                }
            }
        }

        moves_with_scores.sort_by(|a, b| b.1.cmp(&a.1));
        moves_with_scores.truncate(15);

        moves_with_scores.into_iter().map(|(m, _)| m).collect()
    }
}
