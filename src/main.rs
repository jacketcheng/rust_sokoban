use ggez::{conf, event::{self, KeyCode, KeyMods}, graphics::{self, DrawParam, Image}, Context, GameResult};
use glam::Vec2;
use specs::{storage::{self, GenericWriteStorage}, world::Index, Builder, Component, Entities, Join, NullStorage, ReadStorage, RunNow, System, VecStorage, World, WorldExt, Write, WriteStorage};
use std::{collections::HashMap, path};

const TILE_WIDTH: f32 = 32.0;
const MAP_WIDTH: u8 = 8;
const MAP_HEIGHT:u8 = 9;
const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W 
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
"; 

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

#[derive(Default)]
pub struct InputQueue{
    pub keys_pressed: Vec<KeyCode>,
}

// 不可移动组件
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

// 可移动组件
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;


//注册组件
pub fn register_component(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
}

// 注册资源
pub fn register_resource(world: &mut World){
    world.insert(InputQueue::default());
}

// 创建墙实体
pub fn create_wall(world: &mut World, position: Position){
    world
        .create_entity()
        .with(Position{z : 10, ..position})
        .with(Renderable{path: "/images/wall.png".to_string()})
        .with(Box{})
        .with(Immovable)
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
        .with(Movable)
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
        .with(Movable)
        .build();
}
// 创建玩家实体
pub fn create_player(world: &mut World, position: Position){
    world
        .create_entity()
        .with(Position{z: 10, ..position})
        .with(Renderable{
            path: "/images/player.png".to_string()
        })
        .with(Player{})
        .with(Movable)
        .build();
}

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

pub struct InputSystem{}

impl<'a> System<'a> for InputSystem{
    type SystemData = (
        Write<'a, InputQueue>, 
        Entities<'a>,
        WriteStorage<'a, Position>, 
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, 
            mut positions, players, movables, immovables) = data;
        
        let mut to_move = Vec::new();

        for (position, player) in (&positions, &players).join() {   
            if let Some(key) = input_queue.keys_pressed.pop() {
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let immov : HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect();

                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else{
                        (position.x, x_or_y)
                    };

                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => {
                            match immov.get(&pos) { 
                                Some(_id) => to_move.clear(),
                                None => break,
                            }
                        }
                    }
                }
            } 
        }

        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => ()
                }
            }
        }
    }
}

struct Game{world: World}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        {
            let mut is = InputSystem{};
            is.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem {context};
            rs.run_now(&self.world);
        }
        Ok(())
    }

    fn key_down_event(
            &mut self,
            ctx: &mut Context,
            keycode: KeyCode,
            _keymods: KeyMods,
            _repeat: bool,
        ) {
        // println!("key pressed: {:?}", keycode);
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }

}

pub fn init_level(world: &mut World) {
    // create_play(world, Position{x: 0, y: 0, z: 0});
    // create_wall(world, Position{x: 1, y: 0, z: 0});
    // create_box(world, Position{x: 2, y: 0, z: 0});
    load_map(world, MAP.to_string())
}

/// 加载地图
pub fn load_map(world: &mut World, map_string: String) {
    let rows: Vec<&str> = map_string.trim().split("\n").map(|x| x.trim()).collect();
    for (y, row) in rows.iter().enumerate() {
        let columns : Vec<&str> = row.split(" ").collect();

        for (x, column) in columns.iter().enumerate() {
            let position = Position{x: x as u8, y: y as u8, z: 0};

            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_wall(world, position);
                    create_floor(world, position);
                },
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                },
                "B" => {
                    create_floor(world, position);
                    create_box(world, position);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                },
                "N" => (),
                c => panic!("unrecognized map item {}",c),

            }
        }

    }
}

fn main() -> GameResult{
    let mut world = World::new();
    register_component(&mut world);
    register_resource(&mut world);
    init_level(&mut world);

    let context_build = ggez::ContextBuilder::new("rust_bokoban", "sokoban")
    .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
    .window_mode(conf::WindowMode::default().dimensions(800_f32, 600_f32))
    .add_resource_path(path::PathBuf::from("./resources"));

    let (contex, event_loop) = context_build.build()?;

    let game = Game{world};
    event::run(contex, event_loop, game)

}
