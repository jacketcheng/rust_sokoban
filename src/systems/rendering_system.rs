use crate::constants::*;
use crate::components::*;
use crate::Gameplay;
use ggez::graphics::Color;
use ggez::{Context, graphics::{self, DrawParam, Image}};
use glam::Vec2;
use specs::{Join, System, ReadStorage, Read};

// 渲染系统结构体
pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}
/// 为renderingSystem实现System特征,实现渲染系统
/// 1. 清空屏幕
/// 2. 获取所有可渲染组件的实体，按z轴排列好后渲染。
/// 3. 按排列好的顺序一个一个把实体渲染为图片展示。
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        ReadStorage<'a, Position>, 
        ReadStorage<'a, Renderable>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, positions, renderables) = data;
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);
        
        for (position, renderable) in rendering_data.iter() {   
            let image = Image::new(self.context, renderable.path.clone()).expect("image not found");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("render failed");
        }

        self.draw_txt(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_txt(&gameplay.moves_count.to_string(), 525.0, 100.0);

        graphics::present(self.context).expect("fail to present");

    }
}

impl RenderingSystem<'_> {
    pub fn draw_txt(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = Vec2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = Vec2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context, 
            graphics::DrawParam::new().dest(destination), 
            None, 
            graphics::FilterMode::Linear)
        .expect("fail to draw queued text");

    }
}