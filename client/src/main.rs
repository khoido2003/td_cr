use client::{
    globals::{CARD_HEIGHT, CARD_WIDTH},
    render::Renderer,
    ui::{card_preview::CardPreview, deck::Deck, elixir_bar},
};
use macroquad::prelude::*;
use shared::{Card, Vec2D};

fn conf() -> Conf {
    Conf {
        window_title: "Tower Defense".to_owned(),
        window_width: 480,
        window_height: 854,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut game_state = shared::GameState::new();
    let mut elixir_bar = elixir_bar::ElixirBar::new();
    let mut deck = Deck::new(vec![
        Card {
            id: 0,
            name: "Pekka".to_string(),
            cost: 7,
            pos: Vec2D { x: 0.0, y: 0.0 },
            width: CARD_WIDTH,
            height: CARD_HEIGHT,
            health: 1000,
        },
        Card {
            id: 1,
            name: "Goblin".to_string(),
            cost: 2,
            pos: Vec2D { x: 0.0, y: 0.0 },
            width: CARD_WIDTH,
            height: CARD_HEIGHT,
            health: 200,
        },
        Card {
            id: 2,
            name: "Archer".to_string(),
            cost: 3,
            pos: Vec2D { x: 0.0, y: 0.0 },
            width: CARD_WIDTH,
            height: CARD_HEIGHT,
            health: 300,
        },
        Card {
            id: 3,
            name: "Knight".to_string(),
            cost: 4,
            pos: Vec2D { x: 0.0, y: 0.0 },
            width: CARD_WIDTH,
            height: CARD_HEIGHT,
            health: 800,
        },
    ]);

    let card_preview = CardPreview {
        next_card: Some(Card {
            id: 4,
            name: "Knight".to_string(),
            cost: 4,
            pos: Vec2D { x: 0.0, y: 0.0 },
            width: CARD_WIDTH,
            height: CARD_HEIGHT,
            health: 800,
        }),
    };

    loop {
        let renderer = Renderer::new();
        clear_background(BLACK);

        let mouse_pos = mouse_position();
        let mouse_vec = Vec2::from(mouse_pos);

        if let Some(unit) = deck.update(mouse_vec, &mut game_state.players[0].elixir) {
            game_state.units.push(unit);
        }

        // Render
        deck.render(&renderer);
        elixir_bar.render(&renderer);
        card_preview.render(&renderer);

        next_frame().await;
    }
}
