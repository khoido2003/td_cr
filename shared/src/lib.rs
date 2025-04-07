use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub cost: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub health: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tower {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub damage: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: u32,
    pub elixir: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GameState {
    pub players: Vec<Player>,
    pub units: Vec<Unit>,
    pub towers: Vec<Tower>,
    pub cards: Vec<Card>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: vec![Player { id: 1, elixir: 10 }],
            units: Vec::new(),
            towers: Vec::new(),
            cards: vec![Card {
                id: 1,
                name: "Pekka".to_string(),
                cost: 7,
            }],
        }
    }

    pub fn spawn_unit(&mut self, card_id: u32, x: f32, y: f32) {
        if let Some(card) = self.cards.iter().find(|c| c.id == card_id) {
            if self.players[0].elixir >= card.cost {
                self.players[0].elixir -= card.cost;
                self.units.push(Unit {
                    id: self.units.len() as u32 + 1,
                    x,
                    y,
                    health: 1000,
                });
            }
        }
    }
}
