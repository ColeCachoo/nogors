use std::env;
use std::process;

mod nogo;
mod computer;
mod game_board;

use nogo::Nogo;
use nogo::NogoError;

fn main() {
    let mut nogo = match Nogo::new(env::args()) {
        Err(e) => {
            match_error(&e);
            // Process should have already been exited. Keeps compiler from
            // complaining.
            process::exit(7);
        },
        Ok(ng) => ng,
    };

    if let Err(e) = nogo.run() {
        match_error(&e);
    }
}

fn match_error(err: &NogoError) {
    match *err {
        NogoError::NumArg => {
            eprintln!("{}", NogoError::NumArg);
            process::exit(1);
        },

        NogoError::IncorrectType => {
            eprintln!("{}", NogoError::IncorrectType);
            process::exit(2);
        },

        NogoError::InvalidDimension => {
            eprintln!("{}", NogoError::InvalidDimension);
            process::exit(3);
        },

        NogoError::FailedToOpen => {
            eprintln!("{}", NogoError::FailedToOpen);
            process::exit(4);
        },

        NogoError::CorruptFile => {
            eprintln!("{}", NogoError::CorruptFile);
            process::exit(5);
        },

        NogoError::Parse(_) => {
            eprintln!("{}", NogoError::InvalidDimension);
            process::exit(3);
        },

        NogoError::Io(_) => {
            eprintln!("{}", NogoError::FailedToOpen);
            process::exit(4);
        },
    };
}