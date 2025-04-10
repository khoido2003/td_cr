use crate::globals::{
    CARD_HEIGHT, CARD_SLOT_PADDING_X, CARD_SLOT_PADDING_Y, CARD_WIDTH, DECK_HEIGHT, DECK_WIDTH,
    DECK_X, DECK_Y,
};
use crate::render::{Renderer, VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use macroquad::color::{BLUE, BROWN, GOLD, LIGHTGRAY, WHITE};
use macroquad::prelude::*;
use shared::{Card, Unit, Vec2D};

pub struct Deck {
    pub cards: Vec<Card>,
    pub slot_positions: Vec<Vec2D>,
    pub dragging_card: Option<(u32, Vec2, Vec2D)>,
    pub dragging_unit: Option<(u32, Vec2, Vec2D)>,
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        let num_slots = 4;

        let edge_padding_x = CARD_SLOT_PADDING_X;
        let edge_padding_y = CARD_SLOT_PADDING_Y;
        let total_padding_x = 2.0 * edge_padding_x;
        let total_padding_y = 2.0 * edge_padding_y;

        // X-axis layout (same as before)
        let slot_width = CARD_WIDTH + 2.0 * CARD_SLOT_PADDING_X;
        let total_slot_width = num_slots as f32 * slot_width;
        let remaining_space_x = DECK_WIDTH - total_slot_width - total_padding_x;
        let card_spacing = if num_slots > 1 && remaining_space_x > 0.0 {
            remaining_space_x / (num_slots - 1) as f32
        } else {
            0.0
        };
        // Calculate total width including slots and spacing
        let total_width = total_slot_width + card_spacing * (num_slots - 1) as f32;
        if total_width > DECK_WIDTH {
            panic!(
                "Card layout exceeds DECK_WIDTH: {} > {}",
                total_width, DECK_WIDTH
            );
        }

        // Y-axis layout
        let slot_height = CARD_HEIGHT + 2.0 * CARD_SLOT_PADDING_Y;
        let available_height = DECK_HEIGHT - total_padding_y;
        if slot_height > available_height {
            panic!(
                "Card height with padding exceeds available deck height: {} > {}",
                slot_height, available_height
            );
        }

        // Center offset
        let offset_x =
            (DECK_WIDTH - (total_slot_width + card_spacing * (num_slots - 1) as f32)) / 2.0;
        let offset_y = (available_height - slot_height) / 2.0 + edge_padding_y;

        // Center the cards within the deck
        let mut slot_positions = Vec::new();

        for i in 0..num_slots {
            slot_positions.push(Vec2D {
                x: DECK_X + offset_x + (slot_width + card_spacing) * i as f32 + CARD_SLOT_PADDING_X,
                y: DECK_Y + offset_y + CARD_SLOT_PADDING_Y,
            });
        }

        let mut cards = cards;
        for (i, card) in cards.iter_mut().enumerate().take(num_slots) {
            card.pos = slot_positions[i];
        }

        Deck {
            cards,
            slot_positions,
            dragging_card: None,
            dragging_unit: None,
        }
    }

    pub fn render(&self, renderer: &Renderer) {
        // Drawing card deck
        renderer.draw_rectangle(DECK_X, DECK_Y, DECK_WIDTH, DECK_HEIGHT, BROWN);
        renderer.draw_rectangle_lines(DECK_X, DECK_Y, DECK_WIDTH, DECK_HEIGHT, 4.0, GOLD);

        for pos in self.slot_positions.iter() {
            renderer.draw_rectangle_lines(
                pos.x - CARD_SLOT_PADDING_X,
                pos.y - CARD_SLOT_PADDING_Y,
                CARD_WIDTH + 2.0 * CARD_SLOT_PADDING_X,
                CARD_HEIGHT + 2.0 * CARD_SLOT_PADDING_Y,
                2.0,
                LIGHTGRAY,
            );
        }

        // Drawing each card in the deck
        for card in &self.cards {
            renderer.draw_rectangle(card.pos.x, card.pos.y, CARD_WIDTH, CARD_HEIGHT, BLUE);

            renderer.draw_text(
                &card.name,
                card.pos.x,
                card.pos.y + CARD_HEIGHT / 2.0,
                20.0,
                WHITE,
            );
        }

        // Draw dragging unit (if any)
        if let Some((_, _, _)) = self.dragging_unit {
            let mouse_pos = mouse_position();
            let (mouse_x, mouse_y) = renderer.to_virtual(mouse_pos.0, mouse_pos.1);
            let unit_x = mouse_x / VIRTUAL_WIDTH;
            let unit_y = mouse_y / VIRTUAL_HEIGHT;
            renderer.draw_circle(unit_x, unit_y, 0.03 * VIRTUAL_WIDTH, GREEN);
        }
    }

    pub fn update(&mut self, mouse_vec: Vec2, elixir: &mut u32) -> Option<Unit> {
        let mut new_unit: Option<Unit> = None;
        let unit_x = mouse_vec.x / VIRTUAL_WIDTH;
        let unit_y = mouse_vec.y / VIRTUAL_HEIGHT;

        // Check if card is dragging out of the deck or not
        let in_deck = unit_y >= DECK_Y && unit_x >= DECK_X && unit_x <= DECK_X + DECK_WIDTH;

        // Start dragging a card
        if is_mouse_button_pressed(MouseButton::Left)
            && self.dragging_card.is_none()
            && self.dragging_unit.is_none()
        {
            for card in &mut self.cards {
                if card.contains(mouse_vec.x, mouse_vec.y, VIRTUAL_WIDTH, VIRTUAL_HEIGHT) {
                    let offset = Vec2::new(
                        mouse_vec.x - card.pos.x * VIRTUAL_WIDTH,
                        mouse_vec.y - card.pos.y * VIRTUAL_HEIGHT,
                    );
                    self.dragging_card = Some((card.id, offset, card.pos));
                    break;
                }
            }
        }

        // Update dragging card
        if let Some((card_id, offset, original_pos)) = self.dragging_card {
            if let Some(card) = self.cards.iter_mut().find(|c| c.id == card_id) {
                card.pos.x = (mouse_vec.x - offset.x) / VIRTUAL_WIDTH;
                card.pos.y = (mouse_vec.y - offset.y) / VIRTUAL_HEIGHT;
                card.pos.x = card.pos.x.clamp(0.0, 1.0 - CARD_WIDTH);
                card.pos.y = card.pos.y.clamp(0.0, 1.0 - CARD_HEIGHT);

                // Transition to unit if dragged above the deck
                if card.pos.y < DECK_Y - 0.05 {
                    self.dragging_card = None;
                    self.dragging_unit = Some((card_id, Vec2::new(0.0, 0.0), original_pos));
                }
            }
        }

        // Update dragging unit
        if let Some((unit_id, offset, original_pos)) = self.dragging_unit {
            if let Some(card) = self.cards.iter_mut().find(|c| c.id == unit_id) {
                // Transition back to card if dragged into the deck
                if is_mouse_button_down(MouseButton::Left) && in_deck {
                    self.dragging_unit = None;
                    // Calculate the offset to keep the mouse centered on the card
                    let offset = Vec2::new(
                        CARD_WIDTH * VIRTUAL_WIDTH / 2.0,
                        CARD_HEIGHT * VIRTUAL_HEIGHT / 2.0,
                    );
                    self.dragging_card = Some((unit_id, offset, original_pos));
                }
            }

            if is_mouse_button_released(MouseButton::Left) {
                // Safe since unit_id comes from a card
                let card_index = self.cards.iter().position(|c| c.id == unit_id).unwrap();
                let card = &self.cards[card_index];

                // Check if the unit can be deploy to the battlefield or not
                let in_battlefield = unit_y < DECK_Y && *elixir >= card.cost;

                if in_battlefield && !in_deck {
                    let new_unit = Some(Unit {
                        id: unit_id,
                        x: unit_x,
                        y: unit_y,
                        health: card.health,
                        velocity: 10.0,
                    });
                    *elixir -= card.cost;
                    self.cards.remove(card_index);

                // TODO: ADD new card to replace the used one
                //
                //
                } else {
                    // If drag to invalid position => Snap back to the original
                    // postion on the deck
                    if let Some(card) = self.cards.iter_mut().find(|c| c.id == unit_id) {
                        card.pos = original_pos;
                    }
                }
                self.dragging_unit = None;
            }
        }

        if is_mouse_button_released(MouseButton::Left) && self.dragging_card.is_some() {
            if let Some((card_id, _, original_pos)) = self.dragging_card {
                if let Some(card) = self.cards.iter_mut().find(|c| c.id == card_id) {
                    card.pos = original_pos;
                }
            }
            self.dragging_card = None;
        }

        new_unit
    }
}
