use std::time::Duration;

use ggez::{
    Context,
    graphics::{self, Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment},
};
use glam::Vec2;
use hecs::{Entity, World};

use crate::components::*;
use crate::constants::*;

pub fn run_rendering(world: &World, context: &mut Context) {
    // 清除屏幕（这会呈现出背景色）
    let mut canvas =
        graphics::Canvas::from_frame(context, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));

    // 渲染 FPS
    let fps = format!("FPS: {:.0}", context.time.fps());
    draw_text(&mut canvas, &fps, 525.0, 120.0);

    // 获取时间
    let mut query = world.query::<&Time>();
let time = query.iter().next().unwrap().1;

    // 遍历所有位置与可渲染对象的组合，加载图像
    // 并将其绘制在指定位置。
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1.0.z);

    // 遍历所有位置与可渲染对象的组合，加载图像
    // 并将其绘制在指定位置。
    for (_, (position, renderable)) in rendering_data.iter() {
        // 加载图像
        let image = get_image(context, renderable, time.delta);
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        // 画
        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        canvas.draw(&image, draw_params);
    }

    // ANCHOR: draw_gameplay_state
    // 渲染任何文字
    let mut query = world.query::<&Gameplay>();
    let gameplay = query.iter().next().unwrap().1;
    draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0);
    draw_text(&mut canvas, &gameplay.moves_count.to_string(), 525.0, 100.0);
    // ANCHOR_END: draw_gameplay_state

    // 最后，呈现画布；这会将所有内容实际显示在屏幕上。
    canvas.finish(context).expect("expected to present");
}

// ANCHOR: draw_text
pub fn draw_text(canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
    let text = Text::new(TextFragment {
        text: text_string.to_string(),
        color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
        scale: Some(PxScale::from(20.0)),
        ..Default::default()
    });
    canvas.draw(&text, Vec2::new(x, y));
}
// ANCHOR_END: draw_text

pub fn get_image(context: &mut Context, renderable: &Renderable, delta: Duration) -> Image {
    let path_index = match renderable.kind() {
        Renderablekind::Static => {
            // 我们只有一张图片，所以直接返回它
            0
        }
        Renderablekind::Animated => {
            // 如果有多个选项，我们需要根据时间差（delta time）来选择合适的一个。
            // 首先获取以毫秒为单位的时间差，通过对 1000 取模得到毫秒部分，
            // 最后除以 250 得到一个 0 到 4 之间的数值。如果结果为 4，
            // 实际上意味着进入了循环的下一轮（即回到了 0），但这部分的
            // 帧循环（wrapping）逻辑将交由可渲染对象（renderable）来处理。
            ((delta.as_micros() / FRAME_DURATION_MS) % 4) as usize
        }
    };
    let image_path = renderable.path(path_index);
    Image::from_path(context, image_path).unwrap()
}
