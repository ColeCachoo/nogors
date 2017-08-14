use std::error;

use nogo::NogoError;
use nogo::Player;

/// Holds game board.
#[derive(Debug)]
pub struct GameBoard {
    height: usize,
    width:  usize,
    board:  Vec<Vec<char>>,
}

impl GameBoard {
    /// Create new game board with given dimensions.
    pub fn new(height: usize, width: usize) -> Result<GameBoard, NogoError> {
        if height < 4 || height > 1000 || width < 4 || width > 1000 {
            return Err(NogoError::InvalidDimension);
        }

        let mut board = Vec::new();

        for iii in 0..height {
            board.push(Vec::new());

            let s      = ".".repeat(width);
            board[iii] = s.chars().collect();
        }

        Ok(GameBoard {
            height: height,
            width:  width,
            board:  board,
        })
    }

    /// Create board from string version of board. (From a file).
    pub fn from(contents: &str) -> Result<GameBoard, NogoError> {
        let mut board = Vec::new();

        for line in contents.split_whitespace() {
            let tmp_vec: Vec<char> = line.chars().collect();
            board.push(tmp_vec);
        }

        let height = board.len();
        let width  = board[0].len();
        if height < 4 || height > 1000 || width < 4 || width > 1000 {
            return Err(NogoError::CorruptFile);
        }

        Ok(GameBoard {
            height: height,
            width:  width,
            board:  board,
        })
    }

    /// Returns character at given coordiante if it exists.
    pub fn get(&self, h: usize, w: usize) -> char {
        self.board[h][w]
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Prints game board with borders around it.
    pub fn print(&self) {
        // Top border.
        print!("/");
        for line in self.board.iter().take(1) {
            for _ in line.iter() {
                print!("-");
            }
        }
        println!("\\");

        // Side borders and board.
        for line in &self.board {
            print!("|");

            for ch in line {
                print!("{}", *ch);
            }

            println!("|");
        }

        // Bottom border.
        print!("\\");
        for line in self.board.iter().take(1) {
            for _ in line.iter() {
                print!("-");
            }
        }
        println!("/");
    }

    /// Inserts the letter of current player on to board, making sure it's
    /// a valid position.
    pub fn insert_move(&mut self, h: usize, w: usize, current_player: &Player) 
        -> Result<(), Box<error::Error>> 
    {
        if h >= self.height {
            return Err(From::from("Invalid row"));
        } else if w >= self.width {
            return Err(From::from("Invalid column"));
        }
        
        let player = match *current_player {
            Player::O => 'O',
            Player::X => 'X',
        };

        if self.board[h][w] == 'O' || self.board[h][w] == 'X' {
            return Err(From::from("Position already taken"));
        }

        self.board[h][w] = player;

        Ok(())
    }

    /// Check if the game has been won or not.
    /// 
    /// Return: 
    ///     Some(usize, usize): If there was a winner function returns a tuple 
    ///         containing the coordinates that a win was determined. These are 
    ///         used to print the correct winning player (a player can place a
    ///         losing piece).
    ///         
    ///     None: No win was found.
    pub fn check_win(&mut self) -> Option<(usize, usize)> {
        for h in 0..self.height {
            for w in 0..self.width {
                if self.board[h][w] == '.' {
                    continue;
                } 
                
                if !self.check_liberty(h, w) {
                    return Some((h, w));
                }
            }
        }

        None
    }

    /// Checks if a piece has any liberties. Liberties are places a piece 
    /// can grow in to ('.'s). Above, below, left, and right of a piece. Same 
    /// pieces touching are linked. If one of them has a liberty they all have a liberty.
    fn check_liberty(&mut self, h: usize, w: usize) -> bool {
        let mut liberty = false;
        // - 1 to set last_height and last_width to real end of vec.
        let last_height = self.height - 1;
        let last_width  = self.width - 1;
        let player      = self.board[h][w];
        let checked     = match player {
            'O' => 'o',
            'X' => 'x',
             _  => '.',       // This should never happen.
        };

        // Just incase it does happen panic since it is undefined behaviour.
        assert_ne!(checked, '.', "'checked' was equal to '.'");

        // Check order is important.
        //
        // Check left.
        if w != 0 {
            let left = self.board[h][w - 1];

            if left == '.' {
                return true;
            } else if left == player {
                // Recursively check all linked pieces. Piece is changed if it 
                // has been checked. Each piece returned to normal as function "unwinds".
                self.board[h][w] = checked;
                liberty = self.check_liberty(h, w - 1);
                self.board[h][w] = player;
            }
        }

        // Check top.
        if h != 0 {
            let top = self.board[h - 1][w];

            if top == '.' && !liberty {
                return true;
            } else if top == player && !liberty {
                self.board[h][w] = checked;
                liberty = self.check_liberty(h - 1, w);
                self.board[h][w] = player;
            }
        }

        // Check right.
        if w != last_width {
            let right = self.board[h][w + 1];

            if right == '.' && !liberty {
                return true;
            } else if right == player && !liberty {
                self.board[h][w] = checked;
                liberty = self.check_liberty(h, w + 1);
                self.board[h][w] = player;
            }
        }

        // Check bottom.
        if h != last_height {
            let bottom = self.board[h + 1][w];

            if bottom == '.' && !liberty {
                return true;
            } else if bottom == player && !liberty {
                self.board[h][w] = checked;
                liberty = self.check_liberty(h + 1, w);
                self.board[h][w] = player;
            }
        }

        liberty
    }

    /// Appends to file the board with no borders.
    pub fn save(&self, filename: &str) -> Result<(), Box<error::Error>> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new().append(true).open(filename)?;

        for line in &self.board {
            for ch in line {
                write!(file, "{}", ch)?;
            }
            writeln!(file)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_win() {
        let mut game = GameBoard::new(6, 5).unwrap();
        let vec = vec![     
                     //   0    1    2    3    4
            /* 0 */ vec!['X', 'X', '.', 'X', 'X'],
            /* 1 */ vec!['X', 'X', 'O', 'X', 'X'],
            /* 2 */ vec!['O', 'O', 'O', 'O', 'O'],
            /* 3 */ vec!['X', 'X', 'O', 'X', 'X'],
            /* 4 */ vec!['X', 'X', 'O', 'X', 'X'],
            /* 5 */ vec!['X', 'X', '.', 'X', 'X']
        ];

        game.board = vec.clone();
        assert_eq!(game.board, vec);

        // Player O
        //assert_eq!(game.check_liberty(0, 2), true);
        assert_eq!(game.check_liberty(1, 2), true);
        assert_eq!(game.check_liberty(2, 0), true);
        assert_eq!(game.check_liberty(2, 1), true);
        assert_eq!(game.check_liberty(2, 2), true);
        assert_eq!(game.check_liberty(2, 3), true);
        assert_eq!(game.check_liberty(2, 4), true);
        assert_eq!(game.check_liberty(3, 2), true);
        assert_eq!(game.check_liberty(4, 2), true);
        //assert_eq!(game.check_liberty(5, 2), true);

        // Player X
        assert_eq!(game.check_liberty(0, 1), true);
        assert_eq!(game.check_liberty(0, 3), true);
        assert_eq!(game.check_liberty(1, 0), true);
        assert_eq!(game.check_liberty(1, 1), true);
        assert_eq!(game.check_liberty(1, 3), true);
        assert_eq!(game.check_liberty(1, 4), true);
        assert_eq!(game.check_liberty(3, 0), true);
        assert_eq!(game.check_liberty(3, 1), true);
        assert_eq!(game.check_liberty(3, 3), true);
        assert_eq!(game.check_liberty(3, 4), true);
        assert_eq!(game.check_liberty(4, 1), true);
        assert_eq!(game.check_liberty(4, 3), true);
        assert_eq!(game.check_liberty(5, 1), true);
        assert_eq!(game.check_liberty(5, 3), true);
    }

    #[test]
    fn test_insert() {
        let mut game = GameBoard::new(6, 5).unwrap();
        let vec = vec![     
                     //   0    1    2    3    4
            /* 0 */ vec!['X', '.', 'O', '.', 'O'],
            /* 1 */ vec!['.', 'O', '.', 'X', '.'],
            /* 2 */ vec!['.', '.', 'X', '.', '.'],
            /* 3 */ vec!['.', 'X', '.', 'O', '.'],
            /* 4 */ vec!['O', '.', 'X', '.', 'X'],
            /* 5 */ vec!['O', '.', 'X', '.', 'X']
        ];

        game.insert_move(0, 0, &Player::X);
        game.insert_move(1, 3, &Player::X);
        game.insert_move(2, 2, &Player::X);
        game.insert_move(3, 1, &Player::X);
        game.insert_move(4, 2, &Player::X);
        game.insert_move(4, 4, &Player::X);
        game.insert_move(5, 2, &Player::X);
        game.insert_move(5, 4, &Player::X);

        game.insert_move(0, 2, &Player::O);
        game.insert_move(0, 4, &Player::O);
        game.insert_move(1, 1, &Player::O);
        game.insert_move(3, 3, &Player::O);
        game.insert_move(4, 0, &Player::O);
        game.insert_move(5, 0, &Player::O);

        assert_eq!(game.board, vec);
    }
}
