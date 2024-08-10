#![allow(dead_code)]

use crate::rune;

pub struct Wand {
    rune_capacity: i32,
    attr: rune::Attribute,
}

pub struct RuneCollection {
    rune_list: Vec<rune::Rune>,
    rune_index: usize,
}
