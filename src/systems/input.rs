use ggez::{Context, input::keyboard::KeyCode};
use hecs::{Entity, World};

use std::collections::HashMap;

use crate::components::*;
use crate::constants::*;
use crate::events::*;

// ANCHOR: run_input
pub fn run_input(world: &World, context: &mut Context) {
    let mut to_move: Vec<(Entity, KeyCode)> = Vec::new();
    let mut events = Vec::new();
    // ANCHOR_END: run_input

    // 获取所有可移动和不可移动的物体
    let mov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Movable)>()
        .iter()
        .map(|t| ((t.1.0.x, t.1.0.y), t.0))
        .collect::<HashMap<_, _>>();
    let immov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Immovable)>()
        .iter()
        .map(|t| ((t.1.0.x, t.1.0.y), t.0))
        .collect::<HashMap<_, _>>();

    for (_, (position, _player)) in world.query::<(&mut Position, &Player)>().iter() {
        if context.keyboard.is_key_repeated() {
            continue;
        }

        // 沿正确的轴向，从当前位置遍历至地图末尾，
        // 并检查哪些元素需要移动。
        let key = if context.keyboard.is_key_just_pressed(KeyCode::Up) {
            KeyCode::Up
        } else if context.keyboard.is_key_just_pressed(KeyCode::Down) {
            KeyCode::Down
        } else if context.keyboard.is_key_just_pressed(KeyCode::Left) {
            KeyCode::Left
        } else if context.keyboard.is_key_just_pressed(KeyCode::Right) {
            KeyCode::Right
        } else {
            continue;
        };

        let (start, end, is_x) = match key {
            KeyCode::Up => (position.y, 0, false),
            KeyCode::Down => (position.y, MAP_HEIGHT - 1, false),
            KeyCode::Left => (position.x, 0, true),
            KeyCode::Right => (position.x, MAP_WIDTH - 1, true),
            _ => continue,
        };

        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };

        for x_or_y in range {
            let pos = if is_x {
                (x_or_y, position.y)
            } else {
                (position.x, x_or_y)
            };

            // 查找一个可移动元素
            // 如果存在，尝试移动它并继续
            // 如果不存在，则继续尝试查找一个不可移动元素
            match mov.get(&pos) {
                Some(entity) => to_move.push((*entity, key)),
                None => {
                    // 查找不可移动的元素
                    // 如果存在，我们需要停止，不移动任何东西
                    // 如果不存在，我们停止，因为找到了空隙
                    match immov.get(&pos) {
                        Some(_id) => {
                            to_move.clear();
                            events.push(Event::PlayerHitObstacle);
                            break; // 加上 break 后，遇到障碍立刻跳出循环，不再往后扫, 能避免打印两次 Playing sound: wall！锦上添花
                        }
                        None => break,
                    }
                }
            }
        }
    }

    // 更新游戏动作
    if !to_move.is_empty() {
        let mut query = world.query::<&mut Gameplay>();
        let gameplay = query.iter().next().unwrap().1;
        gameplay.moves_count += 1;
    }

    // ANCHOR: event_moved
    // 现在实际移动需要移动的内容
    for (entity, key) in to_move {
        let mut position = world.get::<&mut Position>(entity).unwrap();

        match key {
            KeyCode::Up => position.y -= 1,
            KeyCode::Down => position.y += 1,
            KeyCode::Left => position.x -= 1,
            KeyCode::Right => position.x += 1,
            _ => (),
        }

        // 为刚刚移动的实体触发一个事件
        events.push(Event::EntityMoved(EntityMoved { entity }));
    }
    // ANCHOR_END: event_moved

    // ANCHOR: event_add
    // 最后将事件重新添加回世界中
    {
        let mut query = world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.append(&mut events);
    }
    // ANCHOR_END: event_add
}

#[allow(dead_code)]
fn input_system_duplicate(world: &World, context: &mut Context) {
    for (_, (position, _player)) in world.query::<(&mut Position, &Player)>().iter() {
        if context.keyboard.is_key_pressed(KeyCode::Up) {
            position.y -= 1;
        }
        if context.keyboard.is_key_pressed(KeyCode::Down) {
            position.y += 1;
        }
        if context.keyboard.is_key_pressed(KeyCode::Left) {
            position.x -= 1;
        }
        if context.keyboard.is_key_pressed(KeyCode::Right) {
            position.x += 1;
        }
    }
}
