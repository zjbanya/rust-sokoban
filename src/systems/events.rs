use crate::{
    components::{AudioStore, Box, BoxSpot, EventQueue, Position},
    events::{BoxPlacedOnSpot, EntityMoved, Event},
};
use std::collections::HashMap;

use ggez::Context;
use hecs::World;

pub fn run_process_events(world: &mut World, context: &mut Context) {
    let events = world
        .query::<&mut EventQueue>()
        .iter()
        .next()
        .unwrap()
        .1
        .events
        .drain(..)
        .collect::<Vec<_>>();

    let mut new_events = Vec::new();

    let mut query = world.query::<(&Position, &BoxSpot)>();
    let box_spots_by_position = query
        .iter()
        .map(|(_, (position, box_spot))| ((position.x, position.y), box_spot))
        .collect::<HashMap<_, _>>();

    let mut query = world.query::<&mut AudioStore>();
    let audio_store = query.iter().next().unwrap().1;

    for event in events {
        println!("New event: {:?}", event);
        match event {
            Event::PlayerHitObstacle => {
                // 在此播放声音
                audio_store.play_sound(context, "wall");
            }
            Event::EntityMoved(EntityMoved { entity }) => {
                // 某个实体刚刚被移动，检查它是否为箱子；
                // 如果它被移到了特定位置，则触发更多事件。
                if let Ok(the_box) = world.get::<&Box>(entity) {
                    if let Ok(box_position) = world.get::<&Position>(entity) {
                        // 检查该位置上是否有对象，如果有，
                        // 则判断其类型是正确的还是错误的
                        if let Some(box_spot) =
                            box_spots_by_position.get(&(box_position.x, box_position.y))
                        {
                            new_events.push(Event::BoxPlacedSpot(BoxPlacedOnSpot {
                                is_correct_spot: (box_spot.color == the_box.color),
                            }));
                        }
                    }
                }
            }
            Event::BoxPlacedSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                let sound = if is_correct_spot {
                    "correct"
                } else {
                    "incorrect"
                };
                audio_store.play_sound(context, sound);
            }
        }
    }

    // 最后将事件重新添加回世界中
    {
        let mut query = world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.append(&mut new_events);
    }
}
