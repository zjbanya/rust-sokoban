use std::collections::HashMap;

use hecs::World;

use crate::components::{Box, BoxSpot, Gameplay, GameplayState, Position};

pub fn run_gameplay_state(world: &World) {
    // 获取所有按位置索引的方框
    let mut query = world.query::<(&Position, &Box)>();
    let boxes_by_position: HashMap<(u8, u8), &Box> = query
        .iter()
        .map(|(_, t)| ((t.0.x, t.0.y), t.1))
        .collect::<HashMap<_, _>>();

    // 遍历所有箱子放置点，检查该位置是否有相应的箱子
    let boxes_out_of_position: usize = world
        .query::<(&Position, &BoxSpot)>()
        .iter()
        .map(|(_, (position, box_spot))| {
            if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                if box_spot.color == the_box.color {
                    0
                } else {
                    1
                }
            } else {
                1
            }
        })
        .collect::<Vec<usize>>()
        .into_iter()
        .sum();

    // 如果程序运行到了这一步，说明所有箱子位置上都有箱子，
    // 游戏已获胜
    if boxes_out_of_position == 0 {
        let mut query = world.query::<&mut Gameplay>();
        let gameplay = query.iter().next().unwrap().1;
        gameplay.state = GameplayState::Won;
    }
}
