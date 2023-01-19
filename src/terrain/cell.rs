use bevy::prelude::*;

pub const GRID_SIZE: f32 = 50.;
pub const CELL_SIZE: f32 = 10.;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Cell {
    pub(crate) cell_pos: (f32, f32),
    pub(crate) cell_size: f32,
}
