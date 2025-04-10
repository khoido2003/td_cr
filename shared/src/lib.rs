use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub cost: u32,
    pub pos: Vec2D,
    pub width: f32,
    pub height: f32,
    pub health: u32,
}

impl Card {
    pub fn left(&mut self) -> f32 {
        self.pos.x
    }

    pub fn right(&mut self) -> f32 {
        self.pos.x + self.width
    }

    pub fn top(&mut self) -> f32 {
        self.pos.y
    }

    pub fn bottom(&mut self) -> f32 {
        self.pos.y + self.height
    }

    pub fn contains(&mut self, x: f32, y: f32, screen_w: f32, screen_h: f32) -> bool {
        x >= self.left() * screen_w
            && x <= self.right() * screen_w
            && y >= self.top() * screen_h
            && y < self.bottom() * screen_h
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub health: u32,
    pub velocity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tower {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub damage: u32,
    pub attack_cooldown: f32,
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
            cards: vec![],
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
                    velocity: 100.0,
                });
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        for unit in &mut self.units {
            unit.y += unit.velocity * dt;
        }

        for tower in &mut self.towers {
            tower.attack_cooldown -= dt;
            if tower.attack_cooldown <= 0.0 {
                if let Some(unit) = self
                    .units
                    .iter_mut()
                    .find(|u| ((u.x - tower.x).powi(2) + (u.y - tower.y).powi(2)).sqrt() < 100.0)
                {
                    unit.health = unit.health.saturating_sub(tower.damage);
                    tower.attack_cooldown = 1.0;
                }
            }
        }

        self.units.retain(|u| u.health > 0);
    }
}
