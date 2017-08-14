use nogo::Player;
use nogo::Nogo;
use nogo::NogoError;

#[derive(Debug, Copy, Clone)]
pub struct Computer {
    row:            usize,
    column:         usize,
    mult_factor:    usize,
    height:         usize,
    width:          usize,
    counter:        usize,
    b:              usize,
}

impl Computer {
    /// Creates a new computer from arguments. If there should be a computer
    /// player function returns Some(Computer) otherwise returns None. There
    /// should be a computer player when player1_type or player2_type is
    /// equal to 'c'.
    pub fn new(nogo: &Nogo, player: Player) -> Option<Computer> {
        let initial_row;
        let initial_column;
        let mult_factor;

        match player {
            Player::O => {
                if nogo.get_p1type() == 'h' {
                    return None;
                }
                initial_row     = 1;
                initial_column  = 4;
                mult_factor     = 29;
            },
            Player::X => {
                if nogo.get_p2type() == 'h' {
                    return None;
                }
                initial_row     = 2;
                initial_column  = 10;
                mult_factor     = 17;
            },
        };

        Some(Computer {
            row:            initial_row,
            column:         initial_column,
            mult_factor:    mult_factor,
            height:         nogo.get_height(),
            width:          nogo.get_width(),
            counter:        0,
            b:              initial_row * nogo.get_width() + initial_column,
        })
    }

    /// Creates a Computer from a formatted line in a save file.
    pub fn load(nogo: &Nogo, file_line: &str, player: Player) 
        -> Result<Option<Computer>, NogoError> 
    {
        let parsed: Vec<usize> = file_line.split_whitespace()
                                          .map(|u| u.parse().unwrap())
                                          .collect();

        let mut iter = parsed.iter().take(2);
        let height  = *iter.next().ok_or(NogoError::CorruptFile)?;
        let width   = *iter.next().ok_or(NogoError::CorruptFile)?;

        let mut iter = match player {
            Player::O => {
                parsed.iter().skip(3).take(3)
            },
            Player::X => {
                parsed.iter().skip(6).take(3)
            },
        };

        let row     = *iter.next().ok_or(NogoError::CorruptFile)?;
        let column  = *iter.next().ok_or(NogoError::CorruptFile)?;
        let counter = *iter.next().ok_or(NogoError::CorruptFile)?;

        let computer = Computer::new(nogo, player);

        let mut c;
        if computer.is_some() {
            c = computer.unwrap();
            c.row       = row;
            c.column    = column;
            c.counter   = counter;
            c.height    = height;
            c.width     = width;
        } else {
            return Ok(None);
        }

        Ok(Some(c))
    }

    /// Gets computer's move. Automatically generates next move.
    pub fn get_and_generate_move(&mut self) -> (usize, usize) {
        let r = self.row % self.height;
        let c = self.column % self.width;

        self.generate_next_move();

        (r, c)
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn get_counter(&self) -> usize {
        self.counter
    }

    /// Generates next move based off counter. Stores move in Computer.
    fn generate_next_move(&mut self) {
        self.counter += 1;
        match self.counter % 5 {
            1 => {
                self.row    += 1;
                self.column += 1;
            },

            2 => {
                self.row    += 2;
                self.column += 1;
            },

            3 => {
                self.row    += 1;
                self.column += 0;
            },

            4 => {
                self.row    += 0;
                self.column += 1;
            },

            _ => {  // self.counter % 5 == 0
                let n       = (self.b + self.counter / 5 * self.mult_factor) % 1_000_003;
                self.row    = n / self.width;
                self.column = n % self.width;
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // First 12 moves a computer O should try on a 7x7 board.
    #[test]
    fn test_computer() {
        let mut computer = Computer {
            row:            1,
            column:         4,
            mult_factor:    29,
            height:         7,
            width:          7,
            counter:        0,
            b:              1 * 7 + 4,
        };

        assert_eq!((1, 4), computer.get_and_generate_move());
        assert_eq!((2, 5), computer.get_and_generate_move());
        assert_eq!((4, 6), computer.get_and_generate_move());
        assert_eq!((5, 6), computer.get_and_generate_move());
        assert_eq!((5, 0), computer.get_and_generate_move());
        assert_eq!((5, 5), computer.get_and_generate_move());
        assert_eq!((6, 6), computer.get_and_generate_move());
        assert_eq!((1, 0), computer.get_and_generate_move());
        assert_eq!((2, 0), computer.get_and_generate_move());
        assert_eq!((2, 1), computer.get_and_generate_move());
        assert_eq!((2, 6), computer.get_and_generate_move());
        assert_eq!((3, 0), computer.get_and_generate_move());
        assert_eq!((5, 1), computer.get_and_generate_move());
    }
}
