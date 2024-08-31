use crate::{components::*, map::*, resources::*, systems::*};
use ggez::{conf, event::{self, KeyMods, KeyCode}, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;

mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;


const MAP: &str = "
    N N W W W W W W W
    W W W . . . . . W
    W . . . B . . . W
    W . . . . . . . W 
    W . P . . . . . W
    W . . . . . . . W
    W . . S . . . . W
    W . . . . . . . W
    W W W W W W W W W
"; 

struct Game{world: World}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        {
            let mut is = InputSystem{};
            is.run_now(&self.world);
        }
        {
            let mut gss = GameplayStateSystem{};
            gss.run_now(&self.world);
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
        println!("key pressed: {:?}", keycode);
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
