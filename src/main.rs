/* ANCHOR: all */
// Rust sokoban
// main.rs

use ggez::{GameResult, conf, event};
use hecs::World;
use std::path;

mod components;
mod constants;
mod entities;
mod events;
mod map;
mod systems;

// ANCHOR: game
struct Game {
    world: World,
}
// ANCHOR_END: game

// ANCHOR: handler
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut ggez::Context) -> GameResult {
        // 运行输入系统
        {
            systems::input::run_input(&self.world, context);
        }
        // 运行游戏状态
        {
            systems::gameplay::run_gameplay_state(&self.world);
        }
        // 运行事件处理
        {
            systems::events::run_process_events(&mut self.world, context);
        }
        // 获取并更新时间资源
        {
            let mut query = self.world.query::<&mut crate::components::Time>();
            let time = query.iter().next().unwrap().1;
            time.delta += context.time.delta();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> GameResult {
        {
            systems::rendering::run_rendering(&self.world, context);
        }
        Ok(())
    }
}
// ANCHOR_END: handler

// ANCHOR: main
pub fn main() -> GameResult {
    let mut world = World::new();

    // 创建游戏上下文和事件循环
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;

    entities::create_gameplay(&mut world);
    entities::create_time(&mut world);
    entities::create_event_queue(&mut world);
    entities::create_audio_store(&mut world);

    map::initialize_level(&mut world, &mut context);
    // 创建游戏的状态
    let game = Game { world };
    // 运行主事件循环
    event::run(context, event_loop, game)
}
// ANCHOR_END: main

/* ANCHOR_END: all */
