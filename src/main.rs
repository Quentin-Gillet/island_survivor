mod terrain;
mod camera;
mod assets;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{InteractablePickingPlugin, PickingEvent, PickingPlugin};
use crate::assets::GameAssets;
use crate::camera::CameraPlugin;
use crate::terrain::TerrainPlugin;

pub const HEIGHT: i32 = 720;
pub const WIDTH: i32 = 1280;

fn main() {

    App::new()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Next)
                .with_collection::<GameAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Island survivor".to_string(),
                width: WIDTH as f32,
                height: HEIGHT as f32,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Next
}