use std::{i32, u64};
use std::io::{self, Write};

const SHIFTS: [u8; 4] = [1, 6, 7, 8];
const FULLBOARD: u64 = 279258638311359;
const SUP: i32 =   1_000_000;
const INF: i32 = - 1_000_000;
const MAXDEPTH: u8 = 7;

#[derive(Copy, Clone, Debug)]
struct Board {
    player1: u64,
    player2: u64,
    heights: [u8 ; 7]
}

impl Board {
    fn new() -> Self {
        Board {
            player1: 0,
            player2: 0,
            heights: [0 ; 7]
        }
    }

    fn is_valid(&self, col: u8) -> bool {
        self.heights[col as usize] < 6
    }

    fn is_defeated(&self, botplayer: bool) -> bool {
        let current: u64 = if botplayer {self.player1} else {self.player2};
        for shift in SHIFTS.iter() {
            let con2: u64 = current & (current >> shift);
            let con4: u64 = con2 & (con2 >> 2*shift);
            if con4 > 0 {
                return true;
            }
        };
        false
    }

    fn is_draw(&self) -> bool {
        (self.player1 | self.player2) == FULLBOARD
    }

    fn play(&mut self, col: u8, botplayer: bool) {
        let row: u8 = self.heights[col as usize];
        if row >= 6 { 
            println!("Tentative de placement de pion illégale");
            return; }
        if botplayer {
            self.player2 = self.player2 | (1 << 7 * col + row);
        } else {
            self.player1 = self.player1 | (1 << 7 * col + row);
        }
        self.heights[col as usize] = row + 1;
    }

    fn unplay(&mut self, col: u8, botplayer: bool) {
        let row: u8 = self.heights[col as usize];
        if row == 0 { 
            println!("Tentative de retrait de pion illégale");
            return; 
        }
        if botplayer {
            self.player2 = self.player2 & (u64::MAX ^ 1u64 << 7 * col + row - 1);
        } else {
            self.player1 = self.player1 & (u64::MAX ^ 1u64 << 7 * col + row - 1);
        }
        self.heights[col as usize] = row-1; 
    }

    fn display(&self) {
        for row in (0..6).rev() { 
            for col in 0..7 {
                let idx = col * 7 + row; 
                let mask = 1u64 << idx;
                let ch = if (self.player1 & mask) != 0 {
                    'X' 
                } else if (self.player2 & mask) != 0 {
                    '0' 
                } else {
                    '.' 
                };
                print!("{} ", ch);
            }
            println!();
        }
        println!("0 1 2 3 4 5 6"); 
    }

}

fn get_score(board: Board) -> i32 {
    // La grille du player 1 (humain) bloque le bot
    let ownboard: u64 = board.player1 ^ FULLBOARD; 
    // La grille du player 2 (bot) bloque l'humain
    let advboard: u64 = board.player2 ^ FULLBOARD;
    let mut score: i32 = 0;
    for shift in SHIFTS.iter() {
        let owncon2: u64 = ownboard & (ownboard >> shift);
        let owncon4: u64 = owncon2 & (owncon2 >> 2*shift);
        score += owncon4.count_ones() as i32;
        let advcon2: u64 = advboard & (advboard >> shift);
        let advcon4: u64 = advcon2 & (advcon2 >> 2*shift);
        score -= advcon4.count_ones() as i32;
    }
    score
}

fn human(board: &Board) -> u8 {
    board.display();
    loop {
        print!("Choisissez une colonne (0-6) : ");
        io::stdout().flush().unwrap(); // Force l'affichage immédiat
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<u8>() {
                if num <= 6 {
                    return num;
                }
            }
        }
        println!("Entrée invalide. Veuillez entrer un nombre entre 0 et 6.");
    }
}

fn bot(board: &Board) -> u8 {
    let mut copyboard = *board;
    let (col, _score) = minmax(&mut copyboard, true, MAXDEPTH);
    col
}

fn minmax(board: &mut Board, botplayer: bool, depth: u8) -> (u8, i32) {

    if board.is_defeated(botplayer) {
        let score: i32 =  if botplayer {INF} else {SUP};
        return (0, score);
    } else if board.is_draw() || depth == 0 {
        let score: i32 = get_score(*board);
        return (0, score);
    }

    if botplayer {
        // Cas où l'on maximise
        let mut column: u8 = 0;
        let mut score: i32 = INF;
    
        for col in 0..7 {
            if board.is_valid(col) {
                board.play(col, true);
                let (_c, s) = minmax(board, false, depth-1);
                board.unplay(col, true);
                                
                if s > score {
                    column = col;
                    score = s;
                }
            }
        }
        return (column, score);
        
    } else {
        // Cas où l'on minimise
        let mut column: u8 = 0;
        let mut score: i32 = SUP;

        for col in 0..7 {
            if board.is_valid(col) {
                board.play(col, false);
                let (_c, s) = minmax(board, true, depth-1);
                board.unplay(col, false);

                if s < score {
                    column = col;
                    score = s;
                }
            }
        }
        return (column, score);
    }
}

fn game() {
    let mut board: Board = Board::new();
    let mut opponent: bool = false;
    loop {
        let col: u8;
        if opponent {
            col = bot(&board);
            println!("Bot puts a coin in the column {}.\n", col)
        } else {
            col = human(&board);
            println!()
        }
        board.play(col, opponent);
        if board.is_draw() || board.is_defeated(!opponent) {
            board.display();
            break;
        }
        opponent = !opponent;
    }
}

fn main() {
    game()
}