use crate::prelude::*;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_third_person_camera::*;

const ZOOM: (f32, f32) = (1.5, 30.);

/// Camera logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins(ThirdPersonCameraPlugin)
        // TODO: figure out how not to block 3d scene (bevy splitscreen example)
        //.add_systems(Startup, spawn_ui_camera)
        .add_systems(Startup, spawn_scene_camera);
}

//#[derive(Component)]
//pub struct Ui;

#[derive(Component)]
pub struct SceneCamera;

// fn spawn_ui_camera(mut commands: Commands) {
//     commands.spawn((
//         Name::new("ui-camera"),
//         Camera2d,
//         Camera {
//             order: 1,
//             clear_color: ClearColorConfig::None,
//             ..default()
//         },
//         Ui,
//         Msaa::Off,
//     ));
// }

fn spawn_scene_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        ThirdPersonCamera {
            aim_speed: 3.0,                  // default
            aim_zoom: 0.7,                   // default
            zoom_enabled: true,              // default
            zoom: Zoom::new(ZOOM.0, ZOOM.1), // default
            aim_enabled: true,
            offset_enabled: true,
            offset_toggle_enabled: true,
            gamepad_settings: CustomGamepadSettings::default(),
            ..default()
        },
        Camera {
            order: 0,
            ..default()
        },
        Msaa::Sample4,
        // Marks camera as having a skybox, by default it doesn't specify the render layers the skybox can be seen on
        AtmosphereCamera::default(),
        SceneCamera,
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    );
    commands.spawn(camera);
}
