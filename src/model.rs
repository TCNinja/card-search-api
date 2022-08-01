use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Card<'a> {
    pub game: Game,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Game {
    MagicTheGathering,
    Pokemon,
    YuGiOh,
}
