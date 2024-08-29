use ggez::{conf, event, winit::event_loop, Context, GameResult};
use std::path;

struct Game{}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        Ok(())
    }
}

fn main() -> GameResult{
    println!("Hello, world!");


    let context_build = ggez::ContextBuilder::new("rust_bokoban", "sokoban")
    .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
    .window_mode(conf::WindowMode::default().dimensions(800_f32, 600_f32))
    .add_resource_path(path::PathBuf::from("/resources"));

    let (contex, event_loop) = context_build.build()?;

    let game = Game{};
    event::run(contex, event_loop, game)

}
