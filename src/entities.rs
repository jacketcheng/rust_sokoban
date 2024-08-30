use crate::components::*;
use specs::{Builder, World, WorldExt};

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