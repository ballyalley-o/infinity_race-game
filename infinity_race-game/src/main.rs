use rusty_engine::prelude::*;
use rand::prelude::*;

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

    // create obstacles
    let obstacles_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingCarRed,
        SpritePreset::RacingConeStraight,
        SpritePreset::RacingConeStraight,
        SpritePreset::RollingHoleStart,
    ];

    for (i, preset) in obstacles_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // create health bar
  let health_amount = game.add_text("health_amount", "HEALTH: 0");
  health_amount.translation = Vec2::new(550.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.00;
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // check if the player lost
    if game_state.lost {
        return;
    }
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
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    //handle collisions
    let health_amount = engine.texts.get_mut("health_amount").unwrap();
    for event in engine.collision_events.drain(..) {
        // check if the player collided with an obstacle
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_amount.value = format!("HEALTH: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }

    // check if the game is over
    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "GAME OVER");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}


