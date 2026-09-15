use crate::components::*;
use hecs::{Entity, World};

pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable::new_static("/images/wall.png"),
        Wall {},
        Immovable {},
    ))
}

pub fn create_floor(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 5, ..position },
        Renderable::new_static("/images/floor.png"),
    ))
}

pub fn create_box(world: &mut World, position: Position, color: BoxColor) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable::new_animated(vec![
            &format!("/images/box_{}_1.png", color),
            &format!("/images/box_{}_2.png", color),
        ]),
        Box { color },
        Movable {},
    ))
}

pub fn create_box_spot(world: &mut World, position: Position, color: BoxColor) -> Entity {
    world.spawn((
        Position { z: 9, ..position },
        Renderable::new_static(&format!("/images/box_spot_{}.png", color)),
        BoxSpot { color },
    ))
}

pub fn create_player(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable::new_animated(vec![
            "/images/player_1.png",
            "/images/player_2.png",
            "/images/player_3.png",
        ]),
        Player {},
        Movable {},
    ))
}

pub fn create_gameplay(world: &mut World) -> Entity {
    world.spawn(((Gameplay::default()),))
}

// ANCHOR: create_time
pub fn create_time(world: &mut World) -> Entity {
    world.spawn((Time::default(),))
}
// ANCHOR_END: create_time

pub fn create_event_queue(world: &mut World) -> Entity {
    world.spawn((EventQueue::default(),))
}

pub fn create_audio_store(world: &mut World) -> Entity {
    world.spawn((AudioStore::default(),))
}
