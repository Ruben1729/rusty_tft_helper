use serde::{Deserialize, Serialize};
use crate::rolling_chance::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Champion {
    pub name:       String,
    pub cost:       usize,
    pub qty_used:   usize,
    pub qty_max:    usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChampionCost {
    pub name: String,
    pub cost: usize,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct CostAndQty {
    pub cost: usize,
    pub qty: usize
}

impl Champion {
    pub fn qty_remaining(&self) -> usize {
        self.qty_max - self.qty_used
    }

    pub fn get_probability(&self, player_level: usize) {
        (ROLLING_CHANCE[player_level - 1][self.cost] * self.qty_remaining()) / self.qty_max;
    }
}
