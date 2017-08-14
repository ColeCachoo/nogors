# nogors

A game of atari-go written in Rust. Idea taken from https://github.com/joelfenwick/teaching/blob/master/csse2310/2016/ass1_spec.pdf

# Installation
1. Download rust and follow instructions via https://www.rust-lang.org/en-US/install.html

2. Clone this repository.

3. Build:

    ```sh
    $ cargo build 
    ```

# Starting program

    $ ./nogors p1_type p2_type [height width | filename]

Run nogors with 2 computer players on a 7x7 board:

    $ cargo run c c 7 7

Run nogors with 2 human players on a 15x10 board:

    $ cargo run h h 15 10

Run nogors with 1 computer and 1 human player from a previously saved file:

    $ cargo run c h saved.txt

# Save to a file

During your turn enter "w [filename]":

    Player X> 3 6
    /-------\
    |XXX....|
    |.XX.O..|
    |.....O.|
    |......X|
    |......O|
    |O....OO|
    |.......|
    \-------/
    Player O> 6 6
    /-------\
    |XXX....|
    |.XX.O..|
    |.....O.|
    |......X|
    |......O|
    |O....OO|
    |......O|
    \-------/
    Player X> w save.txt
    Saving to save.txt
