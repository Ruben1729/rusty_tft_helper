use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::fs;
use crate::champion::{ChampionCost, CostAndQty};

pub mod champion;
pub mod rolling_chance;
use self::champion::{Champion};
use self::rolling_chance::ROLLING_CHANCE;

pub struct TftHelperEngine {
    player_level: usize,
    champions: Vec<Champion>
}

impl TftHelperEngine {
    pub fn new() -> Self {
        let mut engine = TftHelperEngine {
            player_level: 1,
            champions: Vec::new()
        };

        match engine.load_data() {
            Ok(_) => {
                for champ in &engine.champions {
                    println!("{}", champ.name)
                }
                engine
            }
            Err(_) => {
                panic!("Unable to load champions.")
            }
        }
    }

    fn parse_champion_names_and_cost(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::read_to_string(filename)?;
        let champions: Vec<ChampionCost> = serde_json::from_str(&file)?;

        for champ in champions {
            self.champions.push(
                Champion {
                    name: champ.name,
                    cost: champ.cost,
                    qty_used: 0,
                    qty_max: 0,
                }
            );
        }

        Ok(())
    }

    fn parse_champion_cost_and_qty(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::read_to_string(filename)?;
        let cost_qty: Vec<CostAndQty> = serde_json::from_str(&file)?;

        for champ in &mut self.champions {
            champ.qty_max = cost_qty.iter().find(|&&i| i.cost == champ.cost).unwrap_or(
                &CostAndQty {
                    cost: 0,
                    qty: 0
                }
            ).qty;
        }

        Ok(())
    }

    fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.parse_champion_names_and_cost("src/assets/champions.json")?;
        self.parse_champion_cost_and_qty("src/assets/champions_qty.json")?;

        Ok(())
    }
}
