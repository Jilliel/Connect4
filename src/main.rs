//! Ce fichier a pour but d'implémenter le jeu Connect4
//! ainsi qu'un algorithme AlphaBeta comme adversaire.
use std::io;

/// Définit ici des variables utiles au Connect4.
const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const NULL: i32 = 0;
// Droite | Haut | Diagonale pi/4 | Diagonale -pi/4
const SHIFT: [(i32, i32) ; 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

/// Fonction renvoyant le joueur suivant.
/// Renvoie un int32.
fn nextplayer(player: i32) -> i32 {
    3 - player
}

/// Fonction permettant d'obtenir la ligne jouable dans la colonne donnée.
/// Renvoie HEIGHT si aucune case n'est disponible dans cette colonne.
/// Renvoie un int32.
fn possible(grid: &[[i32; HEIGHT]; WIDTH], col: usize) -> usize{
    for index in 0..HEIGHT {
        if grid[col][index] == NULL {
            return index;
        }
    };
    HEIGHT
}

/// Fonction permettant de jouer un coup sur la colonne donnée en argument.
/// Renvoie l'indice sur lequel le coup a pû être possible.
fn play(grid: &mut [[i32; HEIGHT]; WIDTH], col: usize, row: usize, player: i32) {
    grid[col][row] = player;
}

/// Enlève le jeton à la colonne et à la ligne passées en argument. 
/// Ne renvoie rien.
fn unplay(grid: &mut [[i32; HEIGHT]; WIDTH], col: usize, row: usize) {
    grid[col][row] = NULL;
}

/// Indique si la case représentée par (colonne, ligne) est dans la grille.
/// Renvoie un booléen.
fn valid(col: i32, row: i32) -> bool {
    0 <= col && col < WIDTH as i32 && 0 <= row && row < HEIGHT as i32
}

/// Indique si la grille est une position terminale dans le jeu.
/// Renvoie un booléen.
fn terminal(grid: &[[i32; HEIGHT]; WIDTH], col: usize, row: usize) -> bool {
    let player: i32 = grid[col][row];
    for (dc, dr) in SHIFT {
        let mut streak: i32 = 0;
        for k in -3..4 {
            let newcol: i32 = col as i32 + k * dc;
            let newrow: i32 = row as i32 + k * dr;
            if !valid(newcol, newrow) {
                streak = 0;
                continue;
            } else if grid[newcol as usize][newrow as usize] != player {
                streak = 0;
                continue;
            } else {
                streak += 1;
            }
            // On vérifie si 4 jetons sont alignés.
            if streak == 4 {
                return true;
            }
        }
    }    
    false
}

/// Affiche la grille dans la console.
/// Ne renvoie rien.
fn show(grid: &[[i32; HEIGHT]; WIDTH]) {
    // Affiche la grille
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let player: i32 = grid[col][HEIGHT-1-row];
            print!("{player} ")
        }
        println!()
    }
    // Affiche les colonnes
    for col in 0..WIDTH {
        print!("{col} ")
    } 
    println!("\n")
}

/// Demande un coup à l'utilisateur.
/// Renvoie l'indice
fn human() -> usize {
    loop {
        let mut buffer: String = String::new();
        io::stdin()
        .read_line(&mut buffer)
        .expect("System Failure");
        
        let choice: i32 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Column id must be a number.");
                continue
            },
        };

        if 0 <= choice && choice < WIDTH as i32 {
            return choice as usize;
        } else {
            println!("Column id must be in [0; WIDTH-1] range with WIDTH = {WIDTH}.");
        }
    }
}

/// Joue une partie.
/// Renvoie le vainqueur.
fn game() {
    let mut grid: [[i32 ; HEIGHT] ; WIDTH] = [[0 ; HEIGHT] ; WIDTH];
    let mut player: i32 = 1;
    loop {
        // Affiche la grille.
        show(&grid);
        let col: usize = human();
        let row: usize = possible(&grid, col);
        // On regarde si la colonne est jouable.
        if row == HEIGHT {
            println!("Column id is not permitted.");
            println!("Player {player} has lost the game.");
            break;
        }
        play(&mut grid, col, row, player);
        // On regarde si la position est terminale.
        if terminal(&grid, col, row) {
            show(&grid);
            println!("Player {player} has won the game.");
            break;
        } 
        // On change de joueur.
        player = nextplayer(player);
    }
}

/// Fonction principale, lance une partie. Ne renvoie rien.
fn main() {
    game();
}
