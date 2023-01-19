use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {

    #[asset(path = "selection_tool.glb#Scene0")]
    pub(crate) selection_tool: Handle<Scene>,

    #[asset(path = "tree.glb#Scene0")]
    pub(crate) tree: Handle<Scene>,

}