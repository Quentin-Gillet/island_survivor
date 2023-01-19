mod cell;
mod selection_tool;

use bevy::prelude::*;
use bevy::render::color::Color;
use bevy_mod_picking::{PickableBundle};
use crate::assets::GameAssets;
use crate::GameState;
use crate::terrain::cell::{Cell, CELL_SIZE, GRID_SIZE};
use crate::terrain::selection_tool::{SelectionTool, SelectionToolPlugin};


pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(SelectionToolPlugin)
            .register_type::<Cell>()
            .register_type::<Grid>()
            .add_system_set(SystemSet::on_enter(GameState::Next).with_system(create_terrain));
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Grid {
    grid_size: f32
}

fn create_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    commands
        .spawn(SpatialBundle {
            ..default()
        })
        .with_children(|grid| {
            let mut first = false;
            for x in 0..GRID_SIZE as usize {
                for z in 0..GRID_SIZE as usize {
                    let color = if first{ Color::rgb(10.2, 10.2, 10.2)} else {first = true; Color::rgb(0., 1., 0.)};
                    grid.spawn(PbrBundle {
                        transform:
                        Transform::from_xyz((x as f32 * CELL_SIZE) - ((GRID_SIZE * CELL_SIZE) / 2.), 0.,
                                            (z as f32 * CELL_SIZE) - ((GRID_SIZE * CELL_SIZE)  / 2.)),
                        mesh: meshes.add(Mesh::from(shape::Plane {size: CELL_SIZE as f32})),
                        material: materials.add(color.into()),
                        ..default()
                    })
                        .insert(Name::new("Cell"))
                        .insert(Cell {cell_pos: (x as f32, z as f32), cell_size: CELL_SIZE})
                        .insert(PickableBundle::default());
                }
            }
        })
        .insert(Name::new("Grid"))
        .insert(Grid {grid_size: GRID_SIZE});

    commands
        .spawn(SceneBundle {
            scene: assets.selection_tool.clone(),
            ..default()
        })
        .insert(Name::new("Selection tool"))
        .insert(SelectionTool::default());

}