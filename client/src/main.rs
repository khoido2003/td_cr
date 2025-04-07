use macroquad::prelude::*;

#[macroquad::main("Tower Defense")]
async fn main() {
    let mut game_state = shared::GameState::new();

    game_state.towers.push(shared::Tower {
        id: 1,
        x: 200.0,
        y: 300.0,
        damage: 10,
    });

    game_state.spawn_unit(1, 400.0, 300.0);

    loop {
        clear_background(BLACK);
        draw_rectangle(50.0, 560.0, 100.0, 140.0, BLUE);
        draw_text("Pekka", 60.0, 580.0, 20.0, WHITE);

        for tower in &game_state.towers {
            draw_circle(tower.x, tower.y, 20.0, RED);
        }

        for unit in &game_state.units {
            draw_circle(unit.x, unit.y, 15.0, GREEN);
        }

        // Draw player mana
        draw_text(
            &format!("Elixir: {}", game_state.players[0].elixir),
            10.0,
            20.0,
            30.0,
            YELLOW,
        );

next_frame().await;
    }
}
