//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use crate::{
    config::{Meshes, Textures},
    prelude::*,
    ui::Label,
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<Meshes>();
    app.load_resource::<Textures>();
    //app.load_resource_from_path::<Fira>("fonts/FiraCode-Regular.ttf");

    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen)
        .add_systems(
            Update,
            continue_to_menu_screen.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
        );
}

fn spawn_loading_screen(
    mut commands: Commands,
    //camera: Query<Entity, With<SceneCamera>>
) {
    //let camera = camera.single();
    commands
        .ui_root()
        //.insert(TargetCamera(camera))
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label(&"Loading...".into());
        });
}

fn continue_to_menu_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Playing);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
