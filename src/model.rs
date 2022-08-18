use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Card<'a> {
    pub id: Uuid,
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
