use rusty_engine::prelude::*;

struct GameState {
    health_amount: u8,
    lost: bool,

}

fn main() {
    let mut game = Game::new();

    // create a player sprite
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation.x = -500.00;
    player1.layer = 10.0;
    player1.collision = true;

    // start bg music
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // create road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.00;
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    //keyboard input
    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }

    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // move the player sprite
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.2;
    if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
        game_state.health_amount = 0;
    }

    // move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }
    }
}
