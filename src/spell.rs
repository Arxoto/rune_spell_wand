#![allow(dead_code)]

use crate::rune;

pub struct SpellRune {
    bullet: rune::Bullet,
    trigger_spell: Option<(rune::Trigger, Spell)>,
}

pub struct Spell {
    attr: rune::Attribute,
    rune_list: Vec<SpellRune>,
}