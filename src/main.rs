use ggez::{conf, event, graphics::{self, DrawParam, Image}, Context, GameResult};
use glam::Vec2;
use specs::{storage, Builder, Component, Join, ReadStorage, RunNow, System, VecStorage, World, WorldExt};
use std::{path};

// 定义组件
#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position{
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {}

//注册组件
pub fn register_component(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Box>();
    world.register::<BoxSpot>();
}

// 创建墙实体
pub fn create_wall(world: &mut World, position: Position){
    world
        .create_entity()
        .with(Position{z : 10, ..position})
        .with(Renderable{path: "/images/wall.png".to_string()})
        .with(Box{})
        .build();
}

// 创建地板实体
pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position{z: 5, ..position})
        .with(Renderable{
            path: "/images/floor.png".to_string()
        })
        .build();
}

// 创建箱子实体
pub fn create_box(world: &mut World, position: Position){
    world
        .create_entity()
        .with(Position{z: 10, ..position})
        .with(Renderable{
            path: "/images/box.png".to_string()
        })
        .with(Box{})
        .build();
}
// 创建箱子框体斑点实体
pub fn create_box_spot(world: &mut World, position: Position){
    world
        .create_entity()
        .with(Position{z: 9, ..position})
        .with(Renderable{
            path: "/images/box_spot.png".to_string()
        })
        .with(BoxSpot{})
        .build();
}
// 创建玩家实体
pub fn create_play(world: &mut World, position: Position){
    world
        .create_entity()
        .with(Position{z: 10, ..position})
        .with(Renderable{
            path: "/images/player.png".to_string()
        })
        .with(Player{})
        .build();
}

const TILE_WIDTH : f32 = 32.0;

// 渲染系统结构体
pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}
/// 为renderingSystem实现System特征,实现渲染系统
/// 1. 清空屏幕
/// 2. 获取所有可渲染组件的实体，按z轴排列好后渲染。
/// 3. 按排列好的顺序一个一个把实体渲染为图片展示。
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;
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
        graphics::present(self.context).expect("fail to present");

    }
}


struct Game{world: World}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem {context};
            rs.run_now(&self.world);
        }
        Ok(())

    }
}

pub fn init_level(world: &mut World) {
    create_play(world, Position{x: 0, y: 0, z: 0});
    create_wall(world, Position{x: 1, y: 0, z: 0});
    create_box(world, Position{x: 2, y: 0, z: 0});
}

fn main() -> GameResult{
    let mut world = World::new();
    register_component(&mut world);
    init_level(&mut world);

    let context_build = ggez::ContextBuilder::new("rust_bokoban", "sokoban")
    .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
    .window_mode(conf::WindowMode::default().dimensions(800_f32, 600_f32))
    .add_resource_path(path::PathBuf::from("./resources"));

    let (contex, event_loop) = context_build.build()?;

    let game = Game{world};
    event::run(contex, event_loop, game)

}
