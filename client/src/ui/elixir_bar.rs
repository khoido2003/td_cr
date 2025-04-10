use macroquad::color::{DARKGRAY, LIGHTGRAY, PURPLE, WHITE};

use crate::{
    globals::{DECK_WIDTH, DECK_X, DECK_Y, ELIXIR_BAR_HEIGHT, ELIXIR_MARGIN},
    render::{Renderer, VIRTUAL_HEIGHT},
};

#[derive(Debug, Default)]
pub struct ElixirBar {
    elixir: u32,
}

impl ElixirBar {
    pub fn new() -> Self {
        ElixirBar { elixir: 7 }
    }

    pub fn render(&self, renderer: &Renderer) {
        for i in 0..10 {
            let filled = i < self.elixir as usize;

            renderer.draw_rectangle(
                DECK_X + DECK_WIDTH * (i as f32 / 10.0),
                1.0 - ELIXIR_BAR_HEIGHT - ELIXIR_MARGIN,
                DECK_WIDTH / 10.0,
                ELIXIR_BAR_HEIGHT,
                if filled { PURPLE } else { DARKGRAY },
            );

            renderer.draw_rectangle_lines(
                DECK_X + DECK_WIDTH * (i as f32 / 10.0),
                1.0 - ELIXIR_BAR_HEIGHT - ELIXIR_MARGIN,
                DECK_WIDTH / 10.0,
                ELIXIR_BAR_HEIGHT,
                2.0,
                LIGHTGRAY,
            );
            renderer.draw_text(
                &format!("{}", self.elixir),
                DECK_X - 0.05,
                1.0 - ELIXIR_MARGIN - ELIXIR_BAR_HEIGHT / 2.0 + 0.01,
                0.04 * VIRTUAL_HEIGHT,
                WHITE,
            );
        }
    }
}
