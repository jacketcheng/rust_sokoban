use specs::{storage::{self, GenericWriteStorage}, world::Index, Builder, 
Component, Entities, Join, NullStorage, ReadStorage, RunNow, System, VecStorage, World, WorldExt, Write, WriteStorage};

/// 定义组件
#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position{
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
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