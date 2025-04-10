use macroquad::color::{BLUE, DARKGRAY, GOLD, WHITE};
use shared::Card;

use crate::{
    globals::{DECK_HEIGHT, DECK_Y, PREVIEW_HEIGHT, PREVIEW_WIDTH, PREVIEW_X},
    render::{Renderer, VIRTUAL_HEIGHT},
};

pub struct CardPreview {
    pub next_card: Option<Card>,
}

impl CardPreview {
    pub fn render(&self, renderer: &Renderer) {
        let preview_y = DECK_Y + 0.02 + (DECK_HEIGHT - PREVIEW_HEIGHT) / 2.0;
        renderer.draw_rectangle(
            PREVIEW_X,
            preview_y,
            PREVIEW_WIDTH,
            PREVIEW_HEIGHT,
            DARKGRAY,
        );
        renderer.draw_rectangle_lines(
            PREVIEW_X,
            preview_y,
            PREVIEW_WIDTH,
            PREVIEW_HEIGHT,
            4.0,
            GOLD,
        );

        if self.next_card.is_some() {
            renderer.draw_rectangle(
                PREVIEW_X + (PREVIEW_WIDTH - 0.10) / 2.0,
                preview_y + (PREVIEW_HEIGHT - 0.06) / 2.0,
                0.10,
                0.06,
                BLUE,
            );
            renderer.draw_text(
                "Next",
                PREVIEW_X + 0.01,
                preview_y + 0.03,
                0.025 * VIRTUAL_HEIGHT,
                WHITE,
            );
        }
    }
}
