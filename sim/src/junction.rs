use std::collections::VecDeque;

use crate::{team::Team, pos::Pos};

#[derive(Clone, Copy, Debug)]
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
    items: VecDeque<JunctionItem>,
    pos: Pos,
    level: Level,
    capped: bool,
}

impl Junction {
    pub fn new(p: Pos, l: Level) -> Junction {
        Junction { items: VecDeque::new(), pos: p, level: l, capped: false }
    }
    pub fn get_top(&mut self) -> Option<&mut JunctionItem> {
        self.items.back_mut()
    }
    pub fn get_top_unmut(&self) -> Option<&JunctionItem> {
        self.items.back()
    }
    pub fn add_item(&mut self, j: JunctionItem) -> bool {
        if self.capped {
            return false;
        }
        match j {
            JunctionItem::Cone(ref t) => {
                if let Some(JunctionItem::Cone(nt)) = self.items.back() && (nt == t) {
                    self.items.push_back(j);
                }
                else {
                    self.items.clear();
                    self.items.push_back(j)
                }
            },
            JunctionItem::Beacon(_) => {
                self.items.clear();
                self.items.push_back(j);
                self.capped = true;
            } 
        }
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
        write!(f, "Level: {:#?}, Top item: {:#?}, Capped: {}", self.level, self.items.back(), self.capped)
    }
}

impl std::fmt::Debug for Junction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

