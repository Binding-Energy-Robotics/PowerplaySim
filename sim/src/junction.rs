use std::collections::VecDeque;

use crate::{team::Team, pos::Pos};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Level {
    Ground,
    Low,
    Middle,
    High
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
    item: Option<JunctionItem>,
    pos: Pos,
    level: Level,
    capped: bool,
}

impl Junction {
    pub fn new(p: Pos, l: Level) -> Junction {
        Junction { item: None, pos: p, level: l, capped: false }
    }
    pub fn get_top(&mut self) -> Option<&mut JunctionItem> {
        self.item.as_mut()
    }
    pub fn get_top_unmut(&self) -> Option<&JunctionItem> {
        self.item.as_ref()
    }
    pub fn add_item(&mut self, j: JunctionItem) -> bool {
        if self.capped {
            return false;
        }
        if let JunctionItem::Beacon(_) = j {
            self.capped = true;
        }
        self.item = Some(j);
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
}

impl std::fmt::Display for Junction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Level: {:?}, Top item: {:?}, Capped: {}. Pos: {}", 
        self.level, self.item, self.capped, self.pos)
    }
}

impl std::fmt::Debug for Junction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

