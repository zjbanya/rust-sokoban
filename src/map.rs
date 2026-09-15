use crate::components::{BoxColor, Position};
use crate::entities::*;
use ggez::Context;
use ggez::audio::Source;
use hecs::World;

// ANCHOR: initialize_level
pub fn initialize_level(world: &mut World, context: &mut Context) {
    // . 是空白位置
    // W 是墙
    // P 是玩家
    // BB and RB 是箱子
    // RS and BS 是箱子放置点
    // N 是空：用于地图的外边缘
    const MAP: &str = "
    N N W W W W W W 
    W W W . . . . W 
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W 
    W . . . . RS . W 
    W . . BS . . . W 
    W . . . . . . W 
    W W W W W W W W 
    ";

    load_sounds(world, context);
    load_map(world, MAP.to_string());
}
// ANCHOR_END: initialize_level

pub fn load_map(world: &mut World, map_string: String) {
    // 读取所有行
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();
        // 创建在地图上生成物体的位置
        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // 我们将从 factory 函数中获取 z 值
            };

            // 确定我们应该创建什么对象
            // ANCHOR: map_match
            match *column {
                "." => {
                    create_floor(world, position);
                }
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "BB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColor::Blue);
                }
                "RB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColor::Red);
                }
                "BS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColor::Blue);
                }
                "RS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColor::Red);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
            // ANCHOR_END: map_match
        }
    }
}

pub fn load_sounds(world: &mut World, context: &mut Context) {
    let mut query = world.query::<&mut crate::components::AudioStore>();
    let audio_store = query.iter().next().unwrap().1;

    let sounds = ["correct", "incorrect", "wall"];
    for sound in sounds.iter() {
        let sound_name = sound.to_string();
        let sound_path = format!("/sounds/{}.wav", sound_name);
        let sound_source = Source::new(context, sound_path).expect("expect sound loaded");
        audio_store
            .sounds
            .insert(sound_name, Box::new(sound_source));
    }
}
