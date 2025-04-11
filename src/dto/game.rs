use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChessVariation {
    DoubleChess,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Rating {
    pub name: ChessVariation,
    pub bullet_rating: f32,
    pub blitz_rating: f32,
    pub rapid_rating: f32,
    pub classical_rating: f32,
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Game {
    pub id: Uuid,
    pub link: String,
    pub date: u64,
    pub teams: Vec<Team>,
    pub result: i8,
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Team {
    pub name: String,
    pub players: Vec<Uuid>,
}
