use std::fmt::{self, Display};

use ggez::event::KeyCode;
use specs::World;

/// 按键记录列表
#[derive(Default)]
pub struct InputQueue{
    pub keys_pressed: Vec<KeyCode>,
}

/// 游戏状态
#[derive(Default)]
pub struct Gameplay{
    pub state: GameplayState,
    pub moves_count: u32,
}

// 注册资源
pub fn register_resource(world: &mut World){
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
}

pub enum GameplayState {
    Playing, // 游戏中
    Won //赢得游戏（游戏结束）
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}