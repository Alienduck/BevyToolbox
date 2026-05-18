use bevy::prelude::*;
use bevy_toolbox::tools_3d::smart_camera::{CameraMode, SmartCamera, SmartCameraPlugin};

fn main() {
    let mut app = App::default();
    app.add_plugins((DefaultPlugins, SmartCameraPlugin))
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        SceneRoot(asset_server.load("models/DingusTheCat.glb#Scene0")),
        Transform::default(),
    ));
    let player = commands.spawn(Transform::default()).id();
    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        SmartCamera {
            camera_subject: Some(player),
            distance: 5.0,
            mode: CameraMode::ThirdPerson,
            ..default()
        },
    ));
}
