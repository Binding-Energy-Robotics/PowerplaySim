use std::collections::VecDeque;

use crate::{team::Team, pos::Pos};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Level {
    Ground,
    Low,
    Middle,
    High
}

impl Level {
    pub fn score(&self) -> u32 {
        match self {
            Level::Ground => 2,
            Level::Low => 3,
            Level::Middle => 4,
            Level::High => 5,
        }
    }
}

#[derive(Debug)]
pub enum JunctionItem {
    Cone(Team),
    Beacon(Team),
}

impl std::fmt::Display for JunctionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &JunctionItem::Beacon(t) => write!(f, "{}'s beacon", t),
            &JunctionItem::Cone(t) => write!(f, "{}'s cone", t),  
        }
    }
}

impl JunctionItem {
    pub fn team(&self) -> Team {
        match self {
            &JunctionItem::Beacon(t) => t,
            &JunctionItem::Cone(t) => t,
        }
    }  
}

pub struct Junction {
    items: Vec<JunctionItem>,
    pos: Pos,
    level: Level,
    capped: bool,
}

impl Junction {
    pub fn new(p: Pos, l: Level) -> Junction {
        Junction { items: Vec::new(), pos: p, level: l, capped: false }
    }
    pub fn get_top(&mut self) -> Option<&mut JunctionItem> {
        self.items.last_mut()
    }
    pub fn get_top_unmut(&self) -> Option<&JunctionItem> {
        self.items.last()
    }
    pub fn add_item(&mut self, j: JunctionItem) -> bool {
        if self.capped {
            return false;
        }
        if let JunctionItem::Beacon(_) = j {
            self.capped = true;
        }
        self.items.push(j);
        true
    }
    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }
    pub fn get_level(&self) -> &Level {
        &self.level
    }
    pub fn is_capped(&self) -> bool {
        self.capped
    }
    pub fn get_items(&self) -> &Vec<JunctionItem> {
        &self.items
    }
}

impl std::fmt::Display for Junction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Level: {:?}, Top item: {:?}, Capped: {}. Pos: {}", 
        self.level, self.items.last(), self.capped, self.pos)
    }
}

impl std::fmt::Debug for Junction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

