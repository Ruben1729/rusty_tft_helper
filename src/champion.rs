use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use gtk::{ScrolledWindow, Application, ApplicationWindow, FlowBox, FlowBoxChild, WindowType, Label};
use gtk::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Champion {
    pub name: String,
    cost: usize
}

pub struct ChampionPool {
    pub champions: Vec<Champion>,
    pool: HashMap<String, usize>
}

impl ChampionPool {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let file = File::open("src/assets/champions.json")?;
        let reader = BufReader::new(file);

        let champions: Vec<Champion> = serde_json::from_reader(reader)?;
        let mut pool:HashMap<String, usize> = HashMap::new();

        for champ in &champions {
            pool.insert(champ.name.clone(), 0);
        }

        Ok(ChampionPool {
            champions,
            pool
        })
    }

    pub fn render(&self, window: &ApplicationWindow) {
        let flow_box = FlowBox::new();
        window.add(&flow_box);

        for champ in &self.champions {
            let flow_box_child = FlowBoxChild::new();
            let label = Label::new(Some(&champ.name));
            label.set_size_request(-1, 10);
            flow_box_child.add(&label);
            flow_box.add(&flow_box_child);
        }
    }
}
