# nogors

A game of atari-go written in rust. Idea taken from https://github.com/joelfenwick/teaching/blob/master/csse2310/2016/ass1_spec.pdf

# Installation
1. Download rust and follow instructions via https://www.rust-lang.org/en-US/install.html

2. Clone source with git: 
$ git clone https://github.com/ColeCachoo/nogors.git
$ cd nogors

3. Build:
$ cargo build 

# Usage
Run nogors with 2 computers on a 7x7 board:
$ cargo run c c 7 7

Run nogors with 2 human players on a 15x10 board:
$ cargo run h h 15 10