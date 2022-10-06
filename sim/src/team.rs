#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Team {
    TeamOne,
    TeamTwo,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Team::TeamOne => write!(f, "Team one"),
            &Team::TeamTwo => write!(f, "Team two"),
        }
    }
}