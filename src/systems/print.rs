use ggez::{Context, input::keyboard::KeyCode};
use hecs::World;

#[allow(dead_code)]
fn run_input_print(_world: &World, context: &mut Context) {
    const KEYS: [(KeyCode, &str); 4] = [
        (KeyCode::Up, "UP"),
        (KeyCode::Down, "DOWN"),
        (KeyCode::Left, "LEFT"),
        (KeyCode::Right, "RIGHT"),
    ];
    for (key, name) in KEYS {
        if context.keyboard.is_key_pressed(key) {
            println!("{}", name);
        }
    }
}
