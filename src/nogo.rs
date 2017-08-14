use std::fmt;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::num::ParseIntError;

use computer::Computer;

use game_board::GameBoard;

#[derive(Debug)]
pub enum NogoError {
    NumArg,
    IncorrectType,
    InvalidDimension,
    FailedToOpen,
    CorruptFile,
    Parse(ParseIntError),
    Io(io::Error),
}

impl From<ParseIntError> for NogoError {
    fn from(e: ParseIntError) -> NogoError {
        NogoError::Parse(e)
    }
}

impl From<io::Error> for NogoError {
    fn from(e: io::Error) -> NogoError {
        NogoError::Io(e)
    }
}

impl fmt::Display for NogoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NogoError::NumArg           => write!(f, "Usage: nogors p1type p2type \
                                                      [height width | filename]"),
            NogoError::IncorrectType    => write!(f, "Invalid type"),
            NogoError::InvalidDimension => write!(f, "Invalid board dimension"),
            NogoError::FailedToOpen     => write!(f, "Unable to open file"),
            NogoError::CorruptFile      => write!(f, "Incorrect file contents"),
            NogoError::Parse(ref e)     => write!(f, "Problem parsing: {}", e),
            NogoError::Io(ref e)        => write!(f, "Io failed: {}", e),
        }
    }
}

impl Error for NogoError {
    fn description(&self) -> &str {
        match *self {
            NogoError::NumArg           => "program started with incorrect number of arguments",
            NogoError::IncorrectType    => "incorrect player type",
            NogoError::InvalidDimension => "board dimension invalid",
            NogoError::FailedToOpen     => "can't open file for reading",
            NogoError::CorruptFile      => "bad input in file",
            NogoError::Parse(ref e)     => e.description(),
            NogoError::Io(ref e)        => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            NogoError::NumArg | 
            NogoError::IncorrectType | 
            NogoError::InvalidDimension | 
            NogoError::FailedToOpen |
            NogoError::CorruptFile      => None,
            NogoError::Parse(ref e)     => Some(e),
            NogoError::Io(ref e)        => Some(e),
        }
    }
}

/// Used to keep track of current player for output and input.
#[derive(Debug)]
pub enum Player {
    O,
    X,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match *self {
            Player::O => 'O',
            Player::X => 'X',
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub struct Nogo {
    // Contains filename from arguments and will contain the name of file user
    // wants to save to.
    filename: String,

    // True if argument is a file name. Program needs to load from
    // a saved file.
    is_file: bool,

    // True if the user wants to save to a file.
    is_save: bool,

    // Player types will either be 'h' or 'c' for human or computer players respectively.
    // Player 1 is O and player 2 is X.
    player1_type: char,
    player2_type: char,

    // Height and width of game board.
    height: usize,
    width:  usize,
}

impl Nogo {
    /// Initializes game from command line arguments.
    pub fn new(mut args: ::std::env::Args) -> Result<Nogo, NogoError> {
        args.next();

        let player1_type = args.next().ok_or(NogoError::NumArg)?;
        let player2_type = args.next().ok_or(NogoError::NumArg)?;
        // Temporarily will contain value for height if there is no filename given.
        let filename     = args.next().ok_or(NogoError::NumArg)?;
        let arg          = args.next();
        let mut is_file  = false;

        let width = if arg.is_none() {
            is_file = true;
            0
        } else {
            arg.unwrap().parse()?
        };

        let height = if is_file {
            0
        } else {
            filename.trim().parse()?
        };

        // Too many arguments.
        if args.next().is_some() {
            return Err(NogoError::NumArg);
        }

        Ok(Nogo {
            filename: filename,
            is_file:  is_file,
            is_save: false,

            player1_type: match &*player1_type {
                "h" => 'h',
                "c" => 'c',
                 _  => { return Err(NogoError::IncorrectType); },
            },

            player2_type: match &*player2_type {
                "h" => 'h',
                "c" => 'c',
                 _  => { return Err(NogoError::IncorrectType); },
            },

            height: height,
            width:  width,
        })
    }

    /// Loads from save file if given then runs game logic.
    pub fn run(&mut self) -> Result<(), NogoError> {
        let mut board;
        let mut current_player;
        let mut computer1;
        let mut computer2;

        if self.is_file {   // Load from file.
            let mut file     = File::open(&self.filename)?;
            let mut contents = String::new();

            file.read_to_string(&mut contents)?;
            let contents = contents.split_at(contents.find('\n').ok_or(NogoError::CorruptFile)?);

            computer1 = Computer::load(self, contents.0, Player::O)?;
            computer2 = Computer::load(self, contents.0, Player::X)?;
            board     = GameBoard::from(contents.1)?;

            let mut first_three = contents.0.split_whitespace().take(3);
            let height = first_three.next().ok_or(NogoError::CorruptFile)?;
            let width = first_three.next().ok_or(NogoError::CorruptFile)?;
            current_player = match first_three.next().ok_or(NogoError::CorruptFile)? {
                "0" => Player::O,
                "1" => Player::X,
                 _  => return Err(NogoError::CorruptFile),
            };

            self.height = height.parse()?;
            self.width  = width.parse()?;

            // Make sure height and width from first 2 numbers in file match the
            // height and width the board got from file.
            if board.get_height() != self.height || board.get_width() != self.width {
                return Err(NogoError::CorruptFile);
            }
        } else {    // Default. Load from args.
            computer1       = Computer::new(self, Player::O);
            computer2       = Computer::new(self, Player::X);
            board           = GameBoard::new(self.height, self.width)?;
            current_player  = Player::O;
        }

        loop {
            board.print();

            let (h, w) = self.get_move(computer1.as_mut(), computer2.as_mut(), &current_player);
            
            if self.is_save {
                match self.save(&board, computer1.as_ref(), computer2.as_ref(), &current_player) {
                    Ok(_)  => {
                        self.is_save = false;
                        continue;     // Don't change player or try to place move.
                    },
                    Err(_) => {
                        eprintln!("Failed to save file");
                        continue;
                    },
                };
            }

            if let Err(e) = board.insert_move(h, w, &current_player) {
                eprintln!("{}", e);
                continue;
            }

            if let Some((h, w)) = board.check_win() {
                board.print();
                let winner = match board.get(h, w) {
                    'O' => 'X',
                    'X' => 'O',
                     _  => '.',   // This should never happen.
                };
                assert_ne!(winner, '.');
                println!("Player {} wins!", winner);
                break;
            }

            Nogo::change_player(&mut current_player);
        }

        Ok(())
    }

    /// Save current game state to file given from user.
    fn save(&self,
            board: &GameBoard, 
            c1: Option<&Computer>, 
            c2: Option<&Computer>, 
            player: &Player) -> Result<(), Box<Error>> {

        let mut file = File::create(&self.filename)?;

        // 0 means O is next to play. 1 means X is next to play.
        let next_to_play = match *player {
            Player::O => 0,
            Player::X => 1,
        };

        let c1_row;
        let c1_column;
        let c1_counter;
        match c1 {
            Some(c) => {
                c1_row      = c.get_row();
                c1_column   = c.get_column();
                c1_counter  = c.get_counter();
            },
            None => {
                c1_row      = 0;
                c1_column   = 0;
                c1_counter  = 0;
            },
        }

        let c2_row;
        let c2_column;
        let c2_counter;
        match c2 {
            Some(c) => {
                c2_row      = c.get_row();
                c2_column   = c.get_column();
                c2_counter  = c.get_counter();
            },
            None => {
                c2_row      = 0;
                c2_column   = 0;
                c2_counter  = 0;
            },
        }

        writeln!(file, "{} {} {} {} {} {} {} {} {}", 
                 self.height,   self.width,     next_to_play,
                 c1_row,        c1_column,      c1_counter,
                 c2_row,        c2_column,      c2_counter)?;

        board.save(&self.filename)?;

        Ok(())
    }

    pub fn get_p1type(&self) -> char {
        self.player1_type
    }

    pub fn get_p2type(&self) -> char {
        self.player2_type
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Gets move from computer or player. Saves current game to specified
    /// file from user.
    fn get_move(&mut self, 
                c1: Option<&mut Computer>, 
                c2: Option<&mut Computer>, 
                player: &Player) 
        -> (usize, usize) 
    {
        print!("Player {}> ", player);
        io::stdout().flush().unwrap();

        let computer = match *player {
            Player::O => c1,
            Player::X => c2,
        };

        if computer.is_some() {
            let (h, w) = computer.unwrap().get_and_generate_move();
            println!("{} {}", h, w);
            return (h, w);
        }

        loop {
            let input = match Nogo::get_player_move() {
                Ok(s)  => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    print!("Player {}> ", player);
                    io::stdout().flush().unwrap();
                    continue;
                },
            };

            if input.0 == "w" {
                println!("Saving to {}", input.1);
                self.filename = input.1;
                self.is_save = true;
                return (0, 0);  // Leave function to go save.
            }

            let h = match input.0.parse() {
                Ok(u)  => u,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    print!("Player {}> ", player);
                    io::stdout().flush().unwrap();
                    continue;
                },
            };

            let w = match input.1.parse() {
                Ok(u)  => u,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    print!("Player {}> ", player);
                    io::stdout().flush().unwrap();
                    continue;
                },
            };

            return (h, w);
        }
    }

    /// Gets player move from standard input. Returns input as tuple.
    fn get_player_move() -> Result<(String, String), Box<Error>> {
        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer)?;

        let input: Vec<&str> = buffer.split_whitespace().collect();

        let h = match input.get(0) {
            Some(n) => String::from(*n),
            None    => return Err(From::from("please enter 2 numbers")),
        };

        let w = match input.get(1) {
            Some(n) => String::from(*n),
            None    => return Err(From::from("please enter 2 numbers")),
        };

        Ok((h, w))
    }

    /// Change current player to next player.
    fn change_player(current_player: &mut Player) {
        *current_player = match *current_player {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_change_player() {
        let mut current_player = Player::O;

        match current_player {
            Player::O => assert!(true),
            Player::X => panic!("not equal"),
        }

        Nogo::change_player(&mut current_player);

        match current_player {
            Player::O => panic!("not equal"),
            Player::X => assert!(true),
        }
    }
}
