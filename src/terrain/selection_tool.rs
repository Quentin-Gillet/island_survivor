use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickingEvent};
use crate::assets::GameAssets;
use crate::camera::PanOrbitCameraRes;
use crate::GameState;
use crate::terrain::cell::{Cell, CELL_SIZE, GRID_SIZE};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SelectionTool {
    pub(crate) active: bool,
    pub cell_pos: (f32, f32),
}

impl Default for SelectionTool {
    fn default() -> Self {
        SelectionTool {active: true, cell_pos: (0., 0.)}
    }
}

impl SelectionTool {
    pub fn grid_pos_to_world(cell_pos: (f32, f32)) -> Transform {
        Transform::from_xyz((cell_pos.0 as f32 * CELL_SIZE) - ((GRID_SIZE * CELL_SIZE) / 2.), 0.1,
                            (cell_pos.1 as f32 * CELL_SIZE) - ((GRID_SIZE * CELL_SIZE)  / 2.))
    }
}

pub struct SelectionToolPlugin;

impl Plugin for SelectionToolPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Next).with_system(grid_selection.label("mouse_input")));
    }
}

fn grid_selection(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut selection_tool: Query<(&mut Transform, &mut SelectionTool), With<SelectionTool>>,
    cells: Query<(Entity, &Cell), With<Cell>>,
    assets: Res<GameAssets>,
    camera: Res<PanOrbitCameraRes>,
) {
    if camera.panning {
        return;
    }
    for event in events.iter() {
        match event {
            PickingEvent::Hover(e) => {
                match e {
                    HoverEvent::JustEntered(e) => {
                        let (_, cell) = cells.get(*e).unwrap();
                        let transform = SelectionTool::grid_pos_to_world(cell.cell_pos);
                        for (mut selection_tool_transform, mut selection_tool) in &mut selection_tool {
                            if selection_tool.active {
                                selection_tool_transform.translation = transform.translation;
                                selection_tool.cell_pos = cell.cell_pos;
                            }
                        }
                    }
                    _ => ()
                }
            },
            PickingEvent::Clicked(_) => {
                info!("selection tool clicked");
                for (_, selection_tool) in &mut selection_tool {
                    if selection_tool.active {
                        let mut transform = SelectionTool::grid_pos_to_world(selection_tool.cell_pos);
                        transform.scale = Vec3::new(2., 2., 2.);
                        commands
                            .spawn(SceneBundle {
                                scene: assets.tree.clone(),
                                transform,
                                ..default()
                            })
                            .insert(Name::new("Tree"));
                    }
                }
            }
            _ => ()
        }
    }
}