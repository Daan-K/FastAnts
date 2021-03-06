use std::io::{Cursor, BufReader};

use instruction::Instruction;
use world::World;

pub fn sample0() -> World {
    let world_str: &[u8] = b"100
100
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5 5 5 5 . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5 5 5 5 . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5 5 5 5 . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . + + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # . + + + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . - - - - - - . . . . . . . . . . . . . + + + + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . - - - - - - - . . . . . . . . . . . . + + + + + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . - - - - - - - - . . . . . . . . . . . + + + + + + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . - - - - - - - - - . . . . . . . . . . . + + + + + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . - - - - - - - - - - . . . . . . . . . . . + + + + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . - - - - - - - - - - - . . . . . . . . . . . + + + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . - - - - - - - - - - . . . . . . . . . . . . + + + + + + + . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . - - - - - - - - - . . . . . . . . . . . . . + + + + + + . . . . . # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . - - - - - - - - . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . - - - - - - - . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . - - - - - - . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . #
# . . . . . . . . . . . . . 5 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . # . . . . . . . . . . . . . . . # # . . . . . . . . . . . . #
 # . . . . . . . . . . . . 5 5 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . # # . . . . . . . . . . . . . . # # . . . . . . . . . . . . . #
# . . . . . . . . . . . . 5 5 5 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . # # . . . . . . . . . . . . . . # # . . . . . . . . . . . . . #
 # . . . . . . . . . . . 5 5 5 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . # # # . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . #
# . . . . . . . . . . . . 5 5 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . # # # . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . 5 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . # # . . . . . . . . . . . . . . # # # # # # # # # . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . # # # . . . . . . . . . . . . . # # # # # # # # # # . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . # # # # . . . . . . . . . . . . # # . . . . . . # # . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . # # . # # . . . # . . . . . . . # # . . . . . . # # . . . . . 5 5 5 #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # . . . . . . # # . . # # . . # # . . . . . . # # # # # # # # # # . . . . . 5 5 5 . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . # # . # # . . . . . . # # # # # # # # # . . . . . . 5 5 5 . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . # # # # . . . . . . . . . . . . . . . . . . . . . 5 5 5 . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # # # # # # # # # # # . . . . . . . . . . . . . . . . . . . # . . . . . # # # . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # # # # # # # # # # # . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . # . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . # # # # # # # # . . . . . . . . . . # # . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . # # # # # # # # # # . . . . . . . . . # # . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . # # . . . . . . # # . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . # # . . . . . . # # . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . # # . . . . # . # # . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . # # . . . # . . # # . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . # # . . # . . . # # . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . # # . # . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . # # . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . # # . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . # # # # # # # # # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . # # # # # # # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # # # # # # # # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # # # # # # # # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . # # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . # # # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . # . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5 5 5 5 . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5 5 5 5 . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . 5 5 5 5 . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . # . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . # # . . . . . . # . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # # # # # # # # # . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . # # . . . . . . # # . . . . . . . . . . . . . . . . . . # . . . . . . . . . # # # # # # # # # # . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . # . . . . . . # # . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . # # . . . . . # # . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . 5 . . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . # # . . . . # # . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . 5 5 . . . . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . # # . . . # # . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . 5 5 5 . 5 . . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . # # . . # # . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . 5 5 5 . 5 5 . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . # # . # # . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . 5 5 . 5 5 5 . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . # # # # . . . . . . . . . . . . . . . . . . . . . . . . # # . . . . . . 5 . . 5 5 5 . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . # # # . . . . . . . 5 5 5 5 . . . . . . . . . . . . . . # # . . . . . . . . . 5 5 . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . # # . . . . . . . . 5 5 5 5 . . . . . . . . . . . . . . # # . . . . . . . . . 5 . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5 5 5 5 . . . . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5 5 5 5 . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5 5 5 5 . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5 5 5 5 . . . . . . . . . . . # # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
# . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . #
 # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
";

    let reader = BufReader::new(Cursor::new(world_str));
    World::parse(reader)
}

pub fn default_program() -> Vec<Instruction> {
    let instrs_str: &[u8] = b"Sense Ahead 1 3 Food
Move 2 0
PickUp 8 0
Flip 3 4 5
Turn Left 0
Flip 2 6 7
Turn Right 0
Move 0 3
Sense Ahead 9 11 Home
Move 10 8
Drop 0
Flip 3 12 13
Turn Left 8
Flip 2 14 15
Turn Right 8
Move 8 11
";

    let buffer = BufReader::new(Cursor::new(instrs_str));
    Instruction::parse(buffer)
}

pub fn ant1() -> Vec<Instruction> {
    let instrs_str: &[u8] = b"Sense Ahead 2233 1 Home
Sense LeftAhead 2233 2 Home
Sense RightAhead 2233 2332 Home
Turn Left 4
Turn Left 5
Turn Left 6
Sense Ahead 2282 7 Home
Sense LeftAhead 2282 8 Home
Sense RightAhead 2282 2331 Home
Sense Ahead 643 10 Marker 4
Sense LeftAhead 633 11 Marker 4
Sense RightAhead 642 27 Marker 4
Sense Ahead 699 13 Marker 4
Sense LeftAhead 635 14 Marker 4
Sense RightAhead 632 32 Marker 4
Sense Ahead 754 16 Marker 4
Sense LeftAhead 637 17 Marker 4
Sense RightAhead 634 37 Marker 4
Sense Ahead 809 19 Marker 4
Sense LeftAhead 639 20 Marker 4
Sense RightAhead 636 42 Marker 4
Sense Ahead 865 22 Marker 4
Sense LeftAhead 641 23 Marker 4
Sense RightAhead 638 47 Marker 4
Sense Ahead 921 25 Marker 4
Sense LeftAhead 631 26 Marker 4
Sense RightAhead 640 52 Marker 4
Sense Ahead 1669 28 FoeHome
Move 57 29
Flip 2 30 31
Turn Left 32
Turn Right 52
Sense Ahead 1681 33 FoeHome
Move 64 34
Flip 2 35 36
Turn Left 37
Turn Right 27
Sense Ahead 1693 38 FoeHome
Move 70 39
Flip 2 40 41
Turn Left 42
Turn Right 32
Sense Ahead 1705 43 FoeHome
Move 76 44
Flip 2 45 46
Turn Left 47
Turn Right 37
Sense Ahead 1717 48 FoeHome
Move 83 49
Flip 2 50 51
Turn Left 52
Turn Right 42
Sense Ahead 1729 53 FoeHome
Move 90 54
Flip 2 55 56
Turn Left 27
Turn Right 47
Sense Here 619 58 Marker 0
Mark 2 59
Mark 3 60
Unmark 1 61
Mark 0 62
Sense Here 9 63 Home
PickUp 97 9
Sense Here 621 65 Marker 0
Mark 1 66
Unmark 2 67
Mark 0 68
Sense Here 12 69 Home
PickUp 113 12
Sense Here 623 71 Marker 0
Mark 1 72
Mark 2 73
Mark 0 74
Sense Here 15 75 Home
PickUp 129 15
Sense Here 625 77 Marker 0
Unmark 1 78
Unmark 2 79
Unmark 3 80
Mark 0 81
Sense Here 18 82 Home
PickUp 145 18
Sense Here 627 84 Marker 0
Unmark 1 85
Unmark 2 86
Mark 3 87
Mark 0 88
Sense Here 21 89 Home
PickUp 161 21
Sense Here 629 91 Marker 0
Unmark 1 92
Unmark 3 93
Mark 2 94
Mark 0 95
Sense Here 24 96 Home
PickUp 177 24
Mark 4 98
Sense Here 102 99 Marker 1
Sense Here 101 100 Marker 2
Sense Here 103 193 Marker 3
Sense Here 106 104 Marker 3
Sense Here 110 111 Marker 2
Turn Left 264
Turn Left 105
Turn Left 335
Mark 5 107
Turn Left 108
Turn Left 109
Turn Left 406
Turn Right 548
Turn Right 112
Turn Right 477
Mark 4 114
Sense Here 118 115 Marker 1
Sense Here 117 116 Marker 2
Sense Here 264 126 Marker 3
Sense Here 120 119 Marker 3
Sense Here 127 122 Marker 2
Turn Left 335
Turn Left 121
Turn Left 406
Mark 5 123
Turn Left 124
Turn Left 125
Turn Left 477
Turn Right 193
Turn Right 128
Turn Right 548
Mark 4 130
Sense Here 134 131 Marker 1
Sense Here 133 132 Marker 2
Sense Here 142 143 Marker 3
Sense Here 135 335 Marker 3
Sense Here 138 136 Marker 2
Turn Left 406
Turn Left 137
Turn Left 477
Mark 5 139
Turn Left 140
Turn Left 141
Turn Left 548
Turn Right 264
Turn Right 144
Turn Right 193
Mark 4 146
Sense Here 150 147 Marker 1
Sense Here 149 148 Marker 2
Sense Here 159 154 Marker 3
Sense Here 406 158 Marker 3
Sense Here 152 151 Marker 2
Turn Left 477
Turn Left 153
Turn Left 548
Mark 5 155
Turn Left 156
Turn Left 157
Turn Left 193
Turn Right 335
Turn Right 160
Turn Right 264
Mark 4 162
Sense Here 166 163 Marker 1
Sense Here 165 164 Marker 2
Sense Here 170 168 Marker 3
Sense Here 174 175 Marker 3
Sense Here 167 477 Marker 2
Turn Left 548
Turn Left 169
Turn Left 193
Mark 5 171
Turn Left 172
Turn Left 173
Turn Left 264
Turn Right 406
Turn Right 176
Turn Right 335
Mark 4 178
Sense Here 182 179 Marker 1
Sense Here 181 180 Marker 2
Sense Here 184 183 Marker 3
Sense Here 191 186 Marker 3
Sense Here 548 190 Marker 2
Turn Left 193
Turn Left 185
Turn Left 264
Mark 5 187
Turn Left 188
Turn Left 189
Turn Left 335
Turn Right 477
Turn Right 192
Turn Right 406
Move 256 194
Move 256 195
Move 256 196
Move 256 197
Move 256 198
Move 256 199
Move 256 200
Move 256 201
Move 256 202
Move 256 203
Move 256 204
Move 256 205
Move 256 206
Move 256 207
Move 256 208
Move 256 209
Move 256 210
Move 256 211
Move 256 212
Move 256 213
Move 256 214
Move 256 215
Move 256 216
Move 256 217
Move 256 218
Move 256 219
Move 256 220
Move 256 221
Move 256 222
Move 256 223
Move 256 224
Move 256 225
Move 256 226
Move 256 227
Move 256 228
Move 256 229
Move 256 230
Move 256 231
Move 256 232
Move 256 233
Move 256 234
Move 256 235
Move 256 236
Move 256 237
Move 256 238
Move 256 239
Move 256 240
Move 256 241
Move 256 242
Move 256 243
Move 256 244
Move 256 245
Move 256 246
Move 256 247
Move 256 248
Move 256 249
Move 256 250
Move 256 251
Move 256 252
Move 256 253
Flip 2 254 255
Turn Left 264
Turn Right 548
Sense Here 257 97 Home
Drop 258
PickUp 259 259
Drop 260
Move 261 258
Turn Left 262
Turn Left 263
Turn Left 1789
Move 327 265
Move 327 266
Move 327 267
Move 327 268
Move 327 269
Move 327 270
Move 327 271
Move 327 272
Move 327 273
Move 327 274
Move 327 275
Move 327 276
Move 327 277
Move 327 278
Move 327 279
Move 327 280
Move 327 281
Move 327 282
Move 327 283
Move 327 284
Move 327 285
Move 327 286
Move 327 287
Move 327 288
Move 327 289
Move 327 290
Move 327 291
Move 327 292
Move 327 293
Move 327 294
Move 327 295
Move 327 296
Move 327 297
Move 327 298
Move 327 299
Move 327 300
Move 327 301
Move 327 302
Move 327 303
Move 327 304
Move 327 305
Move 327 306
Move 327 307
Move 327 308
Move 327 309
Move 327 310
Move 327 311
Move 327 312
Move 327 313
Move 327 314
Move 327 315
Move 327 316
Move 327 317
Move 327 318
Move 327 319
Move 327 320
Move 327 321
Move 327 322
Move 327 323
Move 327 324
Flip 2 325 326
Turn Left 335
Turn Right 193
Sense Here 328 113 Home
Drop 329
PickUp 330 330
Drop 331
Move 332 329
Turn Left 333
Turn Left 334
Turn Left 1789
Move 398 336
Move 398 337
Move 398 338
Move 398 339
Move 398 340
Move 398 341
Move 398 342
Move 398 343
Move 398 344
Move 398 345
Move 398 346
Move 398 347
Move 398 348
Move 398 349
Move 398 350
Move 398 351
Move 398 352
Move 398 353
Move 398 354
Move 398 355
Move 398 356
Move 398 357
Move 398 358
Move 398 359
Move 398 360
Move 398 361
Move 398 362
Move 398 363
Move 398 364
Move 398 365
Move 398 366
Move 398 367
Move 398 368
Move 398 369
Move 398 370
Move 398 371
Move 398 372
Move 398 373
Move 398 374
Move 398 375
Move 398 376
Move 398 377
Move 398 378
Move 398 379
Move 398 380
Move 398 381
Move 398 382
Move 398 383
Move 398 384
Move 398 385
Move 398 386
Move 398 387
Move 398 388
Move 398 389
Move 398 390
Move 398 391
Move 398 392
Move 398 393
Move 398 394
Move 398 395
Flip 2 396 397
Turn Left 406
Turn Right 264
Sense Here 399 129 Home
Drop 400
PickUp 401 401
Drop 402
Move 403 400
Turn Left 404
Turn Left 405
Turn Left 1789
Move 469 407
Move 469 408
Move 469 409
Move 469 410
Move 469 411
Move 469 412
Move 469 413
Move 469 414
Move 469 415
Move 469 416
Move 469 417
Move 469 418
Move 469 419
Move 469 420
Move 469 421
Move 469 422
Move 469 423
Move 469 424
Move 469 425
Move 469 426
Move 469 427
Move 469 428
Move 469 429
Move 469 430
Move 469 431
Move 469 432
Move 469 433
Move 469 434
Move 469 435
Move 469 436
Move 469 437
Move 469 438
Move 469 439
Move 469 440
Move 469 441
Move 469 442
Move 469 443
Move 469 444
Move 469 445
Move 469 446
Move 469 447
Move 469 448
Move 469 449
Move 469 450
Move 469 451
Move 469 452
Move 469 453
Move 469 454
Move 469 455
Move 469 456
Move 469 457
Move 469 458
Move 469 459
Move 469 460
Move 469 461
Move 469 462
Move 469 463
Move 469 464
Move 469 465
Move 469 466
Flip 2 467 468
Turn Left 477
Turn Right 335
Sense Here 470 145 Home
Drop 471
PickUp 472 472
Drop 473
Move 474 471
Turn Left 475
Turn Left 476
Turn Left 1789
Move 540 478
Move 540 479
Move 540 480
Move 540 481
Move 540 482
Move 540 483
Move 540 484
Move 540 485
Move 540 486
Move 540 487
Move 540 488
Move 540 489
Move 540 490
Move 540 491
Move 540 492
Move 540 493
Move 540 494
Move 540 495
Move 540 496
Move 540 497
Move 540 498
Move 540 499
Move 540 500
Move 540 501
Move 540 502
Move 540 503
Move 540 504
Move 540 505
Move 540 506
Move 540 507
Move 540 508
Move 540 509
Move 540 510
Move 540 511
Move 540 512
Move 540 513
Move 540 514
Move 540 515
Move 540 516
Move 540 517
Move 540 518
Move 540 519
Move 540 520
Move 540 521
Move 540 522
Move 540 523
Move 540 524
Move 540 525
Move 540 526
Move 540 527
Move 540 528
Move 540 529
Move 540 530
Move 540 531
Move 540 532
Move 540 533
Move 540 534
Move 540 535
Move 540 536
Move 540 537
Flip 2 538 539
Turn Left 548
Turn Right 406
Sense Here 541 161 Home
Drop 542
PickUp 543 543
Drop 544
Move 545 542
Turn Left 546
Turn Left 547
Turn Left 1789
Move 611 549
Move 611 550
Move 611 551
Move 611 552
Move 611 553
Move 611 554
Move 611 555
Move 611 556
Move 611 557
Move 611 558
Move 611 559
Move 611 560
Move 611 561
Move 611 562
Move 611 563
Move 611 564
Move 611 565
Move 611 566
Move 611 567
Move 611 568
Move 611 569
Move 611 570
Move 611 571
Move 611 572
Move 611 573
Move 611 574
Move 611 575
Move 611 576
Move 611 577
Move 611 578
Move 611 579
Move 611 580
Move 611 581
Move 611 582
Move 611 583
Move 611 584
Move 611 585
Move 611 586
Move 611 587
Move 611 588
Move 611 589
Move 611 590
Move 611 591
Move 611 592
Move 611 593
Move 611 594
Move 611 595
Move 611 596
Move 611 597
Move 611 598
Move 611 599
Move 611 600
Move 611 601
Move 611 602
Move 611 603
Move 611 604
Move 611 605
Move 611 606
Move 611 607
Move 611 608
Flip 2 609 610
Turn Left 193
Turn Right 477
Sense Here 612 177 Home
Drop 613
PickUp 614 614
Drop 615
Move 616 613
Turn Left 617
Turn Left 618
Turn Left 1789
Sense Here 9 620 Home
PickUp 97 9
Sense Here 12 622 Home
PickUp 113 12
Sense Here 15 624 Home
PickUp 129 15
Sense Here 18 626 Home
PickUp 145 18
Sense Here 21 628 Home
PickUp 161 21
Sense Here 24 630 Home
PickUp 177 24
Turn Left 643
Turn Right 643
Turn Left 699
Turn Right 699
Turn Left 754
Turn Right 754
Turn Left 809
Turn Right 809
Turn Left 865
Turn Right 865
Turn Left 921
Turn Right 921
Move 977 644
Sense Ahead 645 643 Friend
Flip 2 646 648
Turn Left 647
Move 731 650
Turn Right 649
Move 953 670
Flip 2 651 653
Turn Left 652
Move 786 655
Turn Right 654
Move 675 645
Flip 2 656 658
Turn Left 657
Move 841 660
Turn Right 659
Move 731 650
Flip 2 661 663
Turn Left 662
Move 897 665
Turn Right 664
Move 786 655
Flip 2 666 668
Turn Left 667
Move 953 670
Turn Right 669
Move 841 660
Flip 2 671 673
Turn Left 672
Move 675 645
Turn Right 674
Move 897 665
Sense Here 680 676 Marker 0
Mark 0 677
Mark 2 678
Mark 3 679
Unmark 1 680
Sense Here 9 681 Home
PickUp 97 682
Turn Left 683
Turn Left 684
Turn Left 685
Move 698 686
Move 698 687
Move 698 688
Move 698 689
Move 698 690
Move 698 691
Move 698 692
Move 698 693
Move 698 694
Move 698 695
Move 698 696
Move 698 697
Move 698 18
Sense Here 1019 18 Marker 4
Move 991 700
Sense Ahead 701 699 Friend
Flip 2 702 704
Turn Left 703
Move 786 706
Turn Right 705
Move 675 726
Flip 2 707 709
Turn Left 708
Move 841 711
Turn Right 710
Move 731 701
Flip 2 712 714
Turn Left 713
Move 897 716
Turn Right 715
Move 786 706
Flip 2 717 719
Turn Left 718
Move 953 721
Turn Right 720
Move 841 711
Flip 2 722 724
Turn Left 723
Move 675 726
Turn Right 725
Move 897 716
Flip 2 727 729
Turn Left 728
Move 731 701
Turn Right 730
Move 953 721
Sense Here 735 732 Marker 0
Mark 0 733
Mark 1 734
Unmark 2 735
Sense Here 12 736 Home
PickUp 113 737
Turn Left 738
Turn Left 739
Turn Left 740
Move 753 741
Move 753 742
Move 753 743
Move 753 744
Move 753 745
Move 753 746
Move 753 747
Move 753 748
Move 753 749
Move 753 750
Move 753 751
Move 753 752
Move 753 21
Sense Here 1033 21 Marker 4
Move 1005 755
Sense Ahead 756 754 Friend
Flip 2 757 759
Turn Left 758
Move 841 761
Turn Right 760
Move 731 781
Flip 2 762 764
Turn Left 763
Move 897 766
Turn Right 765
Move 786 756
Flip 2 767 769
Turn Left 768
Move 953 771
Turn Right 770
Move 841 761
Flip 2 772 774
Turn Left 773
Move 675 776
Turn Right 775
Move 897 766
Flip 2 777 779
Turn Left 778
Move 731 781
Turn Right 780
Move 953 771
Flip 2 782 784
Turn Left 783
Move 786 756
Turn Right 785
Move 675 776
Sense Here 790 787 Marker 0
Mark 0 788
Mark 1 789
Mark 2 790
Sense Here 15 791 Home
PickUp 129 792
Turn Left 793
Turn Left 794
Turn Left 795
Move 808 796
Move 808 797
Move 808 798
Move 808 799
Move 808 800
Move 808 801
Move 808 802
Move 808 803
Move 808 804
Move 808 805
Move 808 806
Move 808 807
Move 808 24
Sense Here 1047 24 Marker 4
Move 1019 810
Sense Ahead 811 809 Friend
Flip 2 812 814
Turn Left 813
Move 897 816
Turn Right 815
Move 786 836
Flip 2 817 819
Turn Left 818
Move 953 821
Turn Right 820
Move 841 811
Flip 2 822 824
Turn Left 823
Move 675 826
Turn Right 825
Move 897 816
Flip 2 827 829
Turn Left 828
Move 731 831
Turn Right 830
Move 953 821
Flip 2 832 834
Turn Left 833
Move 786 836
Turn Right 835
Move 675 826
Flip 2 837 839
Turn Left 838
Move 841 811
Turn Right 840
Move 731 831
Sense Here 846 842 Marker 0
Mark 0 843
Unmark 1 844
Unmark 2 845
Unmark 3 846
Sense Here 18 847 Home
PickUp 145 848
Turn Left 849
Turn Left 850
Turn Left 851
Move 864 852
Move 864 853
Move 864 854
Move 864 855
Move 864 856
Move 864 857
Move 864 858
Move 864 859
Move 864 860
Move 864 861
Move 864 862
Move 864 863
Move 864 9
Sense Here 977 9 Marker 4
Move 1033 866
Sense Ahead 867 865 Friend
Flip 2 868 870
Turn Left 869
Move 953 872
Turn Right 871
Move 841 892
Flip 2 873 875
Turn Left 874
Move 675 877
Turn Right 876
Move 897 867
Flip 2 878 880
Turn Left 879
Move 731 882
Turn Right 881
Move 953 872
Flip 2 883 885
Turn Left 884
Move 786 887
Turn Right 886
Move 675 877
Flip 2 888 890
Turn Left 889
Move 841 892
Turn Right 891
Move 731 882
Flip 2 893 895
Turn Left 894
Move 897 867
Turn Right 896
Move 786 887
Sense Here 902 898 Marker 0
Mark 0 899
Unmark 1 900
Unmark 2 901
Mark 3 902
Sense Here 21 903 Home
PickUp 161 904
Turn Left 905
Turn Left 906
Turn Left 907
Move 920 908
Move 920 909
Move 920 910
Move 920 911
Move 920 912
Move 920 913
Move 920 914
Move 920 915
Move 920 916
Move 920 917
Move 920 918
Move 920 919
Move 920 12
Sense Here 991 12 Marker 4
Move 1047 922
Sense Ahead 923 921 Friend
Flip 2 924 926
Turn Left 925
Move 675 928
Turn Right 927
Move 897 948
Flip 2 929 931
Turn Left 930
Move 731 933
Turn Right 932
Move 953 923
Flip 2 934 936
Turn Left 935
Move 786 938
Turn Right 937
Move 675 928
Flip 2 939 941
Turn Left 940
Move 841 943
Turn Right 942
Move 731 933
Flip 2 944 946
Turn Left 945
Move 897 948
Turn Right 947
Move 786 938
Flip 2 949 951
Turn Left 950
Move 953 923
Turn Right 952
Move 841 943
Sense Here 958 954 Marker 0
Mark 0 955
Unmark 1 956
Unmark 3 957
Mark 2 958
Sense Here 24 959 Home
PickUp 177 960
Turn Left 961
Turn Left 962
Turn Left 963
Move 976 964
Move 976 965
Move 976 966
Move 976 967
Move 976 968
Move 976 969
Move 976 970
Move 976 971
Move 976 972
Move 976 973
Move 976 974
Move 976 975
Move 976 15
Sense Here 1005 15 Marker 4
Sense Here 981 978 Marker 1
Sense Here 980 979 Marker 2
Sense Here 989 985 Marker 3
Sense Here 1061 988 Marker 3
Sense Here 983 982 Marker 2
Turn Left 1080
Turn Left 984
Turn Left 1099
Turn Left 986
Turn Left 987
Turn Left 1118
Turn Right 1156
Turn Right 990
Turn Right 1137
Sense Here 995 992 Marker 1
Sense Here 994 993 Marker 2
Sense Here 999 997 Marker 3
Sense Here 1002 1003 Marker 3
Sense Here 996 1080 Marker 2
Turn Left 1099
Turn Left 998
Turn Left 1118
Turn Left 1000
Turn Left 1001
Turn Left 1137
Turn Right 1061
Turn Right 1004
Turn Right 1156
Sense Here 1009 1006 Marker 1
Sense Here 1008 1007 Marker 2
Sense Here 1011 1010 Marker 3
Sense Here 1017 1013 Marker 3
Sense Here 1099 1016 Marker 2
Turn Left 1118
Turn Left 1012
Turn Left 1137
Turn Left 1014
Turn Left 1015
Turn Left 1156
Turn Right 1080
Turn Right 1018
Turn Right 1061
Sense Here 1023 1020 Marker 1
Sense Here 1022 1021 Marker 2
Sense Here 1024 1118 Marker 3
Sense Here 1027 1025 Marker 3
Sense Here 1030 1031 Marker 2
Turn Left 1137
Turn Left 1026
Turn Left 1156
Turn Left 1028
Turn Left 1029
Turn Left 1061
Turn Right 1099
Turn Right 1032
Turn Right 1080
Sense Here 1037 1034 Marker 1
Sense Here 1036 1035 Marker 2
Sense Here 1137 1044 Marker 3
Sense Here 1039 1038 Marker 3
Sense Here 1045 1041 Marker 2
Turn Left 1156
Turn Left 1040
Turn Left 1061
Turn Left 1042
Turn Left 1043
Turn Left 1080
Turn Right 1118
Turn Right 1046
Turn Right 1099
Sense Here 1051 1048 Marker 1
Sense Here 1050 1049 Marker 2
Sense Here 1058 1059 Marker 3
Sense Here 1052 1156 Marker 3
Sense Here 1055 1053 Marker 2
Turn Left 1061
Turn Left 1054
Turn Left 1080
Turn Left 1056
Turn Left 1057
Turn Left 1099
Turn Right 1137
Turn Right 1060
Turn Right 1118
Sense Here 1533 1062 Marker 5
Sense Ahead 1187 1063 Marker 4
Sense LeftAhead 1177 1064 Marker 4
Sense RightAhead 1186 1065 Marker 4
Turn Left 1066
Turn Left 1067
Turn Left 1068
Sense Here 1070 1069 Home
Unmark 4 18
Sense Here 9 1071 Home
PickUp 1072 1072
Drop 1073
Move 18 1070
Unmark 4 18
Turn Left 1076
Turn Left 1077
Turn Left 1078
Sense LeftAhead 1183 1079 Marker 4
Sense RightAhead 1180 1563 Marker 4
Sense Here 1543 1081 Marker 5
Sense Ahead 1243 1082 Marker 4
Sense LeftAhead 1179 1083 Marker 4
Sense RightAhead 1176 1084 Marker 4
Turn Left 1085
Turn Left 1086
Turn Left 1087
Sense Here 1089 1088 Home
Unmark 4 21
Sense Here 12 1090 Home
PickUp 1091 1091
Drop 1092
Move 21 1089
Unmark 4 21
Turn Left 1095
Turn Left 1096
Turn Left 1097
Sense LeftAhead 1185 1098 Marker 4
Sense RightAhead 1182 1573 Marker 4
Sense Here 1553 1100 Marker 5
Sense Ahead 1298 1101 Marker 4
Sense LeftAhead 1181 1102 Marker 4
Sense RightAhead 1178 1103 Marker 4
Turn Left 1104
Turn Left 1105
Turn Left 1106
Sense Here 1108 1107 Home
Unmark 4 24
Sense Here 15 1109 Home
PickUp 1110 1110
Drop 1111
Move 24 1108
Unmark 4 24
Turn Left 1114
Turn Left 1115
Turn Left 1116
Sense LeftAhead 1175 1117 Marker 4
Sense RightAhead 1184 1583 Marker 4
Sense Here 1563 1119 Marker 5
Sense Ahead 1353 1120 Marker 4
Sense LeftAhead 1183 1121 Marker 4
Sense RightAhead 1180 1122 Marker 4
Turn Left 1123
Turn Left 1124
Turn Left 1125
Sense Here 1127 1126 Home
Unmark 4 9
Sense Here 18 1128 Home
PickUp 1129 1129
Drop 1130
Move 9 1127
Unmark 4 9
Turn Left 1133
Turn Left 1134
Turn Left 1135
Sense LeftAhead 1177 1136 Marker 4
Sense RightAhead 1186 1533 Marker 4
Sense Here 1573 1138 Marker 5
Sense Ahead 1409 1139 Marker 4
Sense LeftAhead 1185 1140 Marker 4
Sense RightAhead 1182 1141 Marker 4
Turn Left 1142
Turn Left 1143
Turn Left 1144
Sense Here 1146 1145 Home
Unmark 4 12
Sense Here 21 1147 Home
PickUp 1148 1148
Drop 1149
Move 12 1146
Unmark 4 12
Turn Left 1152
Turn Left 1153
Turn Left 1154
Sense LeftAhead 1179 1155 Marker 4
Sense RightAhead 1176 1543 Marker 4
Sense Here 1583 1157 Marker 5
Sense Ahead 1465 1158 Marker 4
Sense LeftAhead 1175 1159 Marker 4
Sense RightAhead 1184 1160 Marker 4
Turn Left 1161
Turn Left 1162
Turn Left 1163
Sense Here 1165 1164 Home
Unmark 4 15
Sense Here 24 1166 Home
PickUp 1167 1167
Drop 1168
Move 15 1165
Unmark 4 15
Turn Left 1171
Turn Left 1172
Turn Left 1173
Sense LeftAhead 1181 1174 Marker 4
Sense RightAhead 1178 1553 Marker 4
Turn Left 1187
Turn Right 1187
Turn Left 1243
Turn Right 1243
Turn Left 1298
Turn Right 1298
Turn Left 1353
Turn Right 1353
Turn Left 1409
Turn Right 1409
Turn Left 1465
Turn Right 1465
Move 1521 1188
Sense Ahead 1189 1187 Friend
Flip 2 1190 1192
Turn Left 1191
Move 1275 12
Turn Right 1193
Move 1497 1214
Flip 2 1195 1197
Turn Left 1196
Move 1330 15
Turn Right 1198
Move 1219 1189
Flip 2 1200 1202
Turn Left 1201
Move 1385 18
Turn Right 1203
Move 1275 1194
Flip 2 1205 1207
Turn Left 1206
Move 1441 21
Turn Right 1208
Move 1330 1199
Flip 2 1210 1212
Turn Left 1211
Move 1497 24
Turn Right 1213
Move 1385 1204
Flip 2 1215 1217
Turn Left 1216
Move 1219 9
Turn Right 1218
Move 1441 1209
Sense Here 1224 1220 Marker 0
Mark 0 1221
Mark 2 1222
Mark 3 1223
Unmark 1 1224
Sense Here 9 1225 Home
PickUp 97 1226
Turn Left 1227
Turn Left 1228
Turn Left 1229
Move 1242 1230
Move 1242 1231
Move 1242 1232
Move 1242 1233
Move 1242 1234
Move 1242 1235
Move 1242 1236
Move 1242 1237
Move 1242 1238
Move 1242 1239
Move 1242 1240
Move 1242 1241
Move 1242 18
Sense Here 1019 18 Marker 4
Move 1523 1244
Sense Ahead 1245 1243 Friend
Flip 2 1246 1248
Turn Left 1247
Move 1330 15
Turn Right 1249
Move 1219 1270
Flip 2 1251 1253
Turn Left 1252
Move 1385 18
Turn Right 1254
Move 1275 1245
Flip 2 1256 1258
Turn Left 1257
Move 1441 21
Turn Right 1259
Move 1330 1250
Flip 2 1261 1263
Turn Left 1262
Move 1497 24
Turn Right 1264
Move 1385 1255
Flip 2 1266 1268
Turn Left 1267
Move 1219 9
Turn Right 1269
Move 1441 1260
Flip 2 1271 1273
Turn Left 1272
Move 1275 12
Turn Right 1274
Move 1497 1265
Sense Here 1279 1276 Marker 0
Mark 0 1277
Mark 1 1278
Unmark 2 1279
Sense Here 12 1280 Home
PickUp 113 1281
Turn Left 1282
Turn Left 1283
Turn Left 1284
Move 1297 1285
Move 1297 1286
Move 1297 1287
Move 1297 1288
Move 1297 1289
Move 1297 1290
Move 1297 1291
Move 1297 1292
Move 1297 1293
Move 1297 1294
Move 1297 1295
Move 1297 1296
Move 1297 21
Sense Here 1033 21 Marker 4
Move 1525 1299
Sense Ahead 1300 1298 Friend
Flip 2 1301 1303
Turn Left 1302
Move 1385 18
Turn Right 1304
Move 1275 1325
Flip 2 1306 1308
Turn Left 1307
Move 1441 21
Turn Right 1309
Move 1330 1300
Flip 2 1311 1313
Turn Left 1312
Move 1497 24
Turn Right 1314
Move 1385 1305
Flip 2 1316 1318
Turn Left 1317
Move 1219 9
Turn Right 1319
Move 1441 1310
Flip 2 1321 1323
Turn Left 1322
Move 1275 12
Turn Right 1324
Move 1497 1315
Flip 2 1326 1328
Turn Left 1327
Move 1330 15
Turn Right 1329
Move 1219 1320
Sense Here 1334 1331 Marker 0
Mark 0 1332
Mark 1 1333
Mark 2 1334
Sense Here 15 1335 Home
PickUp 129 1336
Turn Left 1337
Turn Left 1338
Turn Left 1339
Move 1352 1340
Move 1352 1341
Move 1352 1342
Move 1352 1343
Move 1352 1344
Move 1352 1345
Move 1352 1346
Move 1352 1347
Move 1352 1348
Move 1352 1349
Move 1352 1350
Move 1352 1351
Move 1352 24
Sense Here 1047 24 Marker 4
Move 1527 1354
Sense Ahead 1355 1353 Friend
Flip 2 1356 1358
Turn Left 1357
Move 1441 21
Turn Right 1359
Move 1330 1380
Flip 2 1361 1363
Turn Left 1362
Move 1497 24
Turn Right 1364
Move 1385 1355
Flip 2 1366 1368
Turn Left 1367
Move 1219 9
Turn Right 1369
Move 1441 1360
Flip 2 1371 1373
Turn Left 1372
Move 1275 12
Turn Right 1374
Move 1497 1365
Flip 2 1376 1378
Turn Left 1377
Move 1330 15
Turn Right 1379
Move 1219 1370
Flip 2 1381 1383
Turn Left 1382
Move 1385 18
Turn Right 1384
Move 1275 1375
Sense Here 1390 1386 Marker 0
Mark 0 1387
Unmark 1 1388
Unmark 2 1389
Unmark 3 1390
Sense Here 18 1391 Home
PickUp 145 1392
Turn Left 1393
Turn Left 1394
Turn Left 1395
Move 1408 1396
Move 1408 1397
Move 1408 1398
Move 1408 1399
Move 1408 1400
Move 1408 1401
Move 1408 1402
Move 1408 1403
Move 1408 1404
Move 1408 1405
Move 1408 1406
Move 1408 1407
Move 1408 9
Sense Here 977 9 Marker 4
Move 1529 1410
Sense Ahead 1411 1409 Friend
Flip 2 1412 1414
Turn Left 1413
Move 1497 24
Turn Right 1415
Move 1385 1436
Flip 2 1417 1419
Turn Left 1418
Move 1219 9
Turn Right 1420
Move 1441 1411
Flip 2 1422 1424
Turn Left 1423
Move 1275 12
Turn Right 1425
Move 1497 1416
Flip 2 1427 1429
Turn Left 1428
Move 1330 15
Turn Right 1430
Move 1219 1421
Flip 2 1432 1434
Turn Left 1433
Move 1385 18
Turn Right 1435
Move 1275 1426
Flip 2 1437 1439
Turn Left 1438
Move 1441 21
Turn Right 1440
Move 1330 1431
Sense Here 1446 1442 Marker 0
Mark 0 1443
Unmark 1 1444
Unmark 2 1445
Mark 3 1446
Sense Here 21 1447 Home
PickUp 161 1448
Turn Left 1449
Turn Left 1450
Turn Left 1451
Move 1464 1452
Move 1464 1453
Move 1464 1454
Move 1464 1455
Move 1464 1456
Move 1464 1457
Move 1464 1458
Move 1464 1459
Move 1464 1460
Move 1464 1461
Move 1464 1462
Move 1464 1463
Move 1464 12
Sense Here 991 12 Marker 4
Move 1531 1466
Sense Ahead 1467 1465 Friend
Flip 2 1468 1470
Turn Left 1469
Move 1219 9
Turn Right 1471
Move 1441 1492
Flip 2 1473 1475
Turn Left 1474
Move 1275 12
Turn Right 1476
Move 1497 1467
Flip 2 1478 1480
Turn Left 1479
Move 1330 15
Turn Right 1481
Move 1219 1472
Flip 2 1483 1485
Turn Left 1484
Move 1385 18
Turn Right 1486
Move 1275 1477
Flip 2 1488 1490
Turn Left 1489
Move 1441 21
Turn Right 1491
Move 1330 1482
Flip 2 1493 1495
Turn Left 1494
Move 1497 24
Turn Right 1496
Move 1385 1487
Sense Here 1502 1498 Marker 0
Mark 0 1499
Unmark 1 1500
Unmark 3 1501
Mark 2 1502
Sense Here 24 1503 Home
PickUp 177 1504
Turn Left 1505
Turn Left 1506
Turn Left 1507
Move 1520 1508
Move 1520 1509
Move 1520 1510
Move 1520 1511
Move 1520 1512
Move 1520 1513
Move 1520 1514
Move 1520 1515
Move 1520 1516
Move 1520 1517
Move 1520 1518
Move 1520 1519
Move 1520 15
Sense Here 1005 15 Marker 4
Sense Here 977 1522 Home
PickUp 97 977
Sense Here 991 1524 Home
PickUp 113 991
Sense Here 1005 1526 Home
PickUp 129 1005
Sense Here 1019 1528 Home
PickUp 145 1019
Sense Here 1033 1530 Home
PickUp 161 1033
Sense Here 1047 1532 Home
PickUp 177 1047
Sense Here 9 1534 Home
PickUp 97 1535
Sense Ahead 1617 1536 Food
Sense LeftAhead 1607 1537 Food
Sense RightAhead 1616 1538 Food
Turn Left 1539
Turn Left 1540
Turn Left 1541
Unmark 4 1542
Unmark 5 18
Sense Here 12 1544 Home
PickUp 113 1545
Sense Ahead 1626 1546 Food
Sense LeftAhead 1609 1547 Food
Sense RightAhead 1606 1548 Food
Turn Left 1549
Turn Left 1550
Turn Left 1551
Unmark 4 1552
Unmark 5 21
Sense Here 15 1554 Home
PickUp 129 1555
Sense Ahead 1634 1556 Food
Sense LeftAhead 1611 1557 Food
Sense RightAhead 1608 1558 Food
Turn Left 1559
Turn Left 1560
Turn Left 1561
Unmark 4 1562
Unmark 5 24
Sense Here 18 1564 Home
PickUp 145 1565
Sense Ahead 1642 1566 Food
Sense LeftAhead 1613 1567 Food
Sense RightAhead 1610 1568 Food
Turn Left 1569
Turn Left 1570
Turn Left 1571
Unmark 4 1572
Unmark 5 9
Sense Here 21 1574 Home
PickUp 161 1575
Sense Ahead 1651 1576 Food
Sense LeftAhead 1615 1577 Food
Sense RightAhead 1612 1578 Food
Turn Left 1579
Turn Left 1580
Turn Left 1581
Unmark 4 1582
Unmark 5 12
Sense Here 24 1584 Home
PickUp 177 1585
Sense Ahead 1660 1586 Food
Sense LeftAhead 1605 1587 Food
Sense RightAhead 1614 1588 Food
Turn Left 1589
Turn Left 1590
Turn Left 1591
Unmark 4 1592
Unmark 5 15
Turn Left 27
Turn Right 27
Turn Left 32
Turn Right 32
Turn Left 37
Turn Right 37
Turn Left 42
Turn Right 42
Turn Left 47
Turn Right 47
Turn Left 52
Turn Right 52
Turn Left 1617
Turn Right 1617
Turn Left 1626
Turn Right 1626
Turn Left 1634
Turn Right 1634
Turn Left 1642
Turn Right 1642
Turn Left 1651
Turn Right 1651
Turn Left 1660
Turn Right 1660
Unmark 5 1618
Move 1619 1618
Mark 5 1620
PickUp 1621 9
Sense Here 97 1622 Marker 0
Mark 2 1623
Mark 3 1624
Unmark 1 1625
Mark 0 97
Unmark 5 1627
Move 1628 1627
Mark 5 1629
PickUp 1630 12
Sense Here 113 1631 Marker 0
Mark 1 1632
Unmark 2 1633
Mark 0 113
Unmark 5 1635
Move 1636 1635
Mark 5 1637
PickUp 1638 15
Sense Here 129 1639 Marker 0
Mark 1 1640
Mark 2 1641
Mark 0 129
Unmark 5 1643
Move 1644 1643
Mark 5 1645
PickUp 1646 18
Sense Here 145 1647 Marker 0
Unmark 1 1648
Unmark 2 1649
Unmark 3 1650
Mark 0 145
Unmark 5 1652
Move 1653 1652
Mark 5 1654
PickUp 1655 21
Sense Here 161 1656 Marker 0
Unmark 1 1657
Unmark 2 1658
Mark 3 1659
Mark 0 161
Unmark 5 1661
Move 1662 1661
Mark 5 1663
PickUp 1664 24
Sense Here 177 1665 Marker 0
Unmark 1 1666
Unmark 3 1667
Mark 2 1668
Mark 0 177
Sense Ahead 1677 1670 Friend
Sense RightAhead 1671 1672 Foe
Sense LeftAhead 1677 1672 Foe
Move 1673 1672
Turn Right 1674
Sense Ahead 1675 1676 FoeHome
Turn Right 1781
Turn Left 1765
Turn Left 1678
Sense Ahead 1679 1745 FoeHome
Sense Ahead 1680 1681 Friend
Turn Left 1749
Sense Ahead 1689 1682 Friend
Sense RightAhead 1683 1684 Foe
Sense LeftAhead 1689 1684 Foe
Move 1685 1684
Turn Right 1686
Sense Ahead 1687 1688 FoeHome
Turn Right 1785
Turn Left 1769
Turn Left 1690
Sense Ahead 1691 1749 FoeHome
Sense Ahead 1692 1693 Friend
Turn Left 1753
Sense Ahead 1701 1694 Friend
Sense RightAhead 1695 1696 Foe
Sense LeftAhead 1701 1696 Foe
Move 1697 1696
Turn Right 1698
Sense Ahead 1699 1700 FoeHome
Turn Right 1765
Turn Left 1773
Turn Left 1702
Sense Ahead 1703 1753 FoeHome
Sense Ahead 1704 1705 Friend
Turn Left 1757
Sense Ahead 1713 1706 Friend
Sense RightAhead 1707 1708 Foe
Sense LeftAhead 1713 1708 Foe
Move 1709 1708
Turn Right 1710
Sense Ahead 1711 1712 FoeHome
Turn Right 1769
Turn Left 1777
Turn Left 1714
Sense Ahead 1715 1757 FoeHome
Sense Ahead 1716 1717 Friend
Turn Left 1761
Sense Ahead 1725 1718 Friend
Sense RightAhead 1719 1720 Foe
Sense LeftAhead 1725 1720 Foe
Move 1721 1720
Turn Right 1722
Sense Ahead 1723 1724 FoeHome
Turn Right 1773
Turn Left 1781
Turn Left 1726
Sense Ahead 1727 1761 FoeHome
Sense Ahead 1728 1729 Friend
Turn Left 1741
Sense Ahead 1737 1730 Friend
Sense RightAhead 1731 1732 Foe
Sense LeftAhead 1737 1732 Foe
Move 1733 1732
Turn Right 1734
Sense Ahead 1735 1736 FoeHome
Turn Right 1777
Turn Left 1785
Turn Left 1738
Sense Ahead 1739 1741 FoeHome
Sense Ahead 1740 1669 Friend
Turn Left 1745
Move 1742 1741
Sense RightAhead 1743 1744 FoeHome
Turn Right 1729
Turn Right 1761
Move 1746 1745
Sense RightAhead 1747 1748 FoeHome
Turn Right 1669
Turn Right 1741
Move 1750 1749
Sense RightAhead 1751 1752 FoeHome
Turn Right 1681
Turn Right 1745
Move 1754 1753
Sense RightAhead 1755 1756 FoeHome
Turn Right 1693
Turn Right 1749
Move 1758 1757
Sense RightAhead 1759 1760 FoeHome
Turn Right 1705
Turn Right 1753
Move 1762 1761
Sense RightAhead 1763 1764 FoeHome
Turn Right 1717
Turn Right 1757
PickUp 1766 1766
Move 1767 1765
Sense Ahead 1765 1768 FoeHome
Turn Left 1785
PickUp 1770 1770
Move 1771 1769
Sense Ahead 1769 1772 FoeHome
Turn Left 1765
PickUp 1774 1774
Move 1775 1773
Sense Ahead 1773 1776 FoeHome
Turn Left 1769
PickUp 1778 1778
Move 1779 1777
Sense Ahead 1777 1780 FoeHome
Turn Left 1773
PickUp 1782 1782
Move 1783 1781
Sense Ahead 1781 1784 FoeHome
Turn Left 1777
PickUp 1786 1786
Move 1787 1785
Sense Ahead 1785 1788 FoeHome
Turn Left 1781
Sense Ahead 1790 1789 Friend
Drop 1791
Drop 1792
Drop 1793
Drop 1794
Drop 1795
Drop 1796
Drop 1797
Drop 1798
Drop 1799
Drop 1800
Drop 1801
Drop 1802
Drop 1803
Drop 1804
Turn Left 1805
Turn Left 1806
Turn Left 1807
Move 1808 1807
Flip 2 1809 1811
Turn Left 1810
Turn Left 24
Turn Right 1812
Turn Right 12
Mark 0 1814
Mark 2 1815
Mark 3 1816
Unmark 1 1817
Move 1818 1818
Mark 0 1819
Mark 2 1820
Mark 3 1821
Unmark 1 1822
Turn Left 1823
Move 1824 1823
Mark 0 1825
Mark 1 1826
Unmark 2 1827
Turn Left 1828
Move 1829 1828
Mark 0 1830
Mark 1 1831
Mark 2 1832
Turn Left 1833
Move 1834 1833
Mark 0 1835
Unmark 1 1836
Unmark 2 1837
Unmark 3 1838
Turn Right 1839
Move 1840 1839
Mark 0 1841
Mark 1 1842
Mark 2 1843
Move 1844 1843
Mark 0 1845
Mark 1 1846
Mark 2 1847
Move 1848 1847
Mark 0 1849
Mark 1 1850
Mark 2 1851
Move 1852 1851
Mark 0 1853
Mark 1 1854
Mark 2 1855
Turn Left 1856
Move 1857 1856
Mark 0 1858
Unmark 1 1859
Unmark 2 1860
Unmark 3 1861
Move 1862 1861
Mark 0 1863
Unmark 1 1864
Unmark 2 1865
Unmark 3 1866
Move 1867 1866
Mark 0 1868
Unmark 1 1869
Unmark 2 1870
Unmark 3 1871
Turn Right 1872
Turn Right 1873
Move 1874 1873
Mark 0 1875
Unmark 1 1876
Unmark 2 1877
Unmark 3 1878
Turn Right 1879
Move 1880 1879
Mark 0 1881
Unmark 1 1882
Unmark 2 1883
Unmark 3 1884
Move 1885 1884
Mark 0 1886
Unmark 1 1887
Unmark 2 1888
Unmark 3 1889
Move 1890 1889
Mark 0 1891
Mark 1 1892
Mark 2 1893
Turn Right 1894
Move 1895 1894
Mark 0 1896
Mark 1 1897
Mark 2 1898
Move 1899 1898
Mark 0 1900
Mark 1 1901
Mark 2 1902
Move 1903 1902
Mark 0 1904
Mark 1 1905
Mark 2 1906
Move 1907 1906
Mark 0 1908
Mark 1 1909
Mark 2 24
Mark 0 1911
Mark 2 1912
Mark 3 1913
Unmark 1 1914
Move 1915 1915
Mark 0 1916
Mark 2 1917
Mark 3 1918
Unmark 1 1919
Turn Left 1920
Move 1921 1920
Mark 0 1922
Mark 1 1923
Unmark 2 1924
Turn Left 1925
Move 1926 1925
Mark 0 1927
Mark 1 1928
Mark 2 1929
Turn Left 1930
Move 1931 1930
Mark 0 1932
Unmark 1 1933
Unmark 2 1934
Unmark 3 1935
Turn Right 1936
Move 1937 1936
Mark 0 1938
Mark 1 1939
Mark 2 1940
Move 1941 1940
Mark 0 1942
Mark 1 1943
Mark 2 1944
Move 1945 1944
Mark 0 1946
Mark 1 1947
Mark 2 1948
Move 1949 1948
Mark 0 1950
Mark 1 1951
Mark 2 1952
Turn Left 1953
Move 1954 1953
Mark 0 1955
Unmark 1 1956
Unmark 2 1957
Unmark 3 1958
Move 1959 1958
Mark 0 1960
Unmark 1 1961
Unmark 2 1962
Unmark 3 1963
Turn Right 1964
Move 1965 1964
Mark 0 1966
Unmark 1 1967
Unmark 2 1968
Unmark 3 1969
Turn Right 1970
Turn Right 1971
Move 1972 1971
Mark 0 1973
Unmark 1 1974
Unmark 2 1975
Unmark 3 1976
Move 1977 1976
Mark 0 1978
Unmark 1 1979
Unmark 2 1980
Unmark 3 1981
Move 1982 1981
Mark 0 1983
Mark 1 1984
Mark 2 1985
Turn Right 1986
Move 1987 1986
Mark 0 1988
Mark 1 1989
Mark 2 1990
Move 1991 1990
Mark 0 1992
Mark 1 1993
Mark 2 1994
Move 1995 1994
Mark 0 1996
Mark 1 1997
Mark 2 1998
Move 1999 1998
Mark 0 2000
Mark 1 2001
Mark 2 15
Mark 0 2003
Mark 2 2004
Mark 3 2005
Unmark 1 2006
Move 2007 2007
Mark 0 2008
Mark 2 2009
Mark 3 2010
Unmark 1 2011
Turn Right 2012
Move 2013 2012
Mark 0 2014
Unmark 1 2015
Unmark 3 2016
Mark 2 2017
Turn Right 2018
Move 2019 2018
Mark 0 2020
Unmark 1 2021
Unmark 2 2022
Mark 3 2023
Turn Right 2024
Move 2025 2024
Mark 0 2026
Unmark 1 2027
Unmark 2 2028
Unmark 3 2029
Turn Left 2030
Move 2031 2030
Mark 0 2032
Unmark 1 2033
Unmark 2 2034
Mark 3 2035
Move 2036 2035
Mark 0 2037
Unmark 1 2038
Unmark 2 2039
Mark 3 2040
Move 2041 2040
Mark 0 2042
Unmark 1 2043
Unmark 2 2044
Mark 3 2045
Move 2046 2045
Mark 0 2047
Unmark 1 2048
Unmark 2 2049
Mark 3 2050
Turn Right 2051
Move 2052 2051
Mark 0 2053
Unmark 1 2054
Unmark 2 2055
Unmark 3 2056
Move 2057 2056
Mark 0 2058
Unmark 1 2059
Unmark 2 2060
Unmark 3 2061
Move 2062 2061
Mark 0 2063
Unmark 1 2064
Unmark 2 2065
Unmark 3 2066
Turn Left 2067
Turn Left 2068
Move 2069 2068
Mark 0 2070
Unmark 1 2071
Unmark 2 2072
Unmark 3 2073
Turn Left 2074
Move 2075 2074
Mark 0 2076
Unmark 1 2077
Unmark 2 2078
Unmark 3 2079
Move 2080 2079
Mark 0 2081
Unmark 1 2082
Unmark 2 2083
Unmark 3 2084
Move 2085 2084
Mark 0 2086
Unmark 1 2087
Unmark 2 2088
Mark 3 2089
Turn Left 2090
Move 2091 2090
Mark 0 2092
Unmark 1 2093
Unmark 2 2094
Mark 3 2095
Move 2096 2095
Mark 0 2097
Unmark 1 2098
Unmark 2 2099
Mark 3 2100
Move 2101 2100
Mark 0 2102
Unmark 1 2103
Unmark 2 2104
Mark 3 2105
Move 2106 2105
Mark 0 2107
Unmark 1 2108
Unmark 2 2109
Mark 3 12
Mark 0 2111
Mark 2 2112
Mark 3 2113
Unmark 1 2114
Move 2115 2115
Mark 0 2116
Mark 2 2117
Mark 3 2118
Unmark 1 2119
Turn Right 2120
Move 2121 2120
Mark 0 2122
Unmark 1 2123
Unmark 3 2124
Mark 2 2125
Turn Right 2126
Move 2127 2126
Mark 0 2128
Unmark 1 2129
Unmark 2 2130
Mark 3 2131
Turn Right 2132
Move 2133 2132
Mark 0 2134
Unmark 1 2135
Unmark 2 2136
Unmark 3 2137
Turn Left 2138
Move 2139 2138
Mark 0 2140
Unmark 1 2141
Unmark 2 2142
Mark 3 2143
Move 2144 2143
Mark 0 2145
Unmark 1 2146
Unmark 2 2147
Mark 3 2148
Move 2149 2148
Mark 0 2150
Unmark 1 2151
Unmark 2 2152
Mark 3 2153
Move 2154 2153
Mark 0 2155
Unmark 1 2156
Unmark 2 2157
Mark 3 2158
Turn Right 2159
Move 2160 2159
Mark 0 2161
Unmark 1 2162
Unmark 2 2163
Unmark 3 2164
Move 2165 2164
Mark 0 2166
Unmark 1 2167
Unmark 2 2168
Unmark 3 2169
Turn Left 2170
Move 2171 2170
Mark 0 2172
Unmark 1 2173
Unmark 2 2174
Unmark 3 2175
Turn Left 2176
Turn Left 2177
Move 2178 2177
Mark 0 2179
Unmark 1 2180
Unmark 2 2181
Unmark 3 2182
Move 2183 2182
Mark 0 2184
Unmark 1 2185
Unmark 2 2186
Unmark 3 2187
Move 2188 2187
Mark 0 2189
Unmark 1 2190
Unmark 2 2191
Mark 3 2192
Turn Left 2193
Move 2194 2193
Mark 0 2195
Unmark 1 2196
Unmark 2 2197
Mark 3 2198
Move 2199 2198
Mark 0 2200
Unmark 1 2201
Unmark 2 2202
Mark 3 2203
Move 2204 2203
Mark 0 2205
Unmark 1 2206
Unmark 2 2207
Mark 3 2208
Move 2209 2208
Mark 0 2210
Unmark 1 2211
Unmark 2 2212
Mark 3 21
Move 2214 2213
Move 2215 2214
Move 2216 2215
Mark 0 2217
Mark 1 2218
Mark 2 2219
Turn Left 2220
Turn Left 2221
Turn Left 2222
Move 2223 2222
Move 2224 2223
Move 2225 2224
Move 2226 2225
Turn Left 2227
Turn Left 2228
Turn Left 2229
Move 2230 2229
Turn Left 2231
Turn Left 2232
Turn Left 2225
Drop 2234
Drop 2235
Drop 2236
Sense RightAhead 2273 2237 Marker 0
Drop 2238
Sense LeftAhead 2273 2239 Marker 0
Drop 2240
Sense Ahead 2278 2241 Marker 0
Drop 2242
Sense Ahead 2243 3 Marker 1
Drop 2244
Drop 2245
Drop 2246
Drop 2247
Drop 2248
Sense Ahead 2250 2249 Friend
Move 1789 2250
Unmark 0 2251
Unmark 1 2252
Drop 2253
Drop 2254
Drop 2255
Drop 2256
Drop 2257
Drop 2258
Drop 2259
Drop 2260
Drop 2261
Drop 2262
Move 2263 2263
Turn Right 2312
Drop 2265
Drop 2266
Drop 2267
Drop 2268
Drop 2269
Unmark 0 2270
Unmark 1 2271
Move 2272 2272
Turn Left 2272
Mark 1 2274
Drop 2275
Drop 2276
Drop 2277
Unmark 1 2264
Mark 1 2279
Drop 2280
Drop 2281
Move 2002 2282
Drop 2283
Drop 2284
Drop 2285
Sense RightAhead 2322 2286 Marker 0
Drop 2287
Sense LeftAhead 2322 2288 Marker 0
Drop 2289
Sense Ahead 2327 2290 Marker 0
Drop 2291
Sense Ahead 2292 18 Marker 1
Drop 2293
Drop 2294
Drop 2295
Drop 2296
Drop 2297
Sense Ahead 2299 2298 Friend
Move 2213 2299
Unmark 0 2300
Unmark 1 2301
Drop 2302
Drop 2303
Drop 2304
Drop 2305
Drop 2306
Drop 2307
Drop 2308
Drop 2309
Drop 2310
Drop 2311
Move 2312 2312
Turn Right 2312
Drop 2314
Drop 2315
Drop 2316
Drop 2317
Drop 2318
Unmark 0 2319
Unmark 1 2320
Move 2321 2321
Turn Left 2321
Mark 1 2323
Drop 2324
Drop 2325
Drop 2326
Unmark 1 2313
Mark 1 2328
Drop 2329
Drop 2330
Move 2110 2331
Mark 0 1910
Mark 0 1813
";

    let reader = BufReader::new(Cursor::new(instrs_str));
    Instruction::parse(reader)
}
