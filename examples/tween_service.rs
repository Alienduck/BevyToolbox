use bevy::prelude::*;
use bevy_toolbox::tools_3d::{
    tween_service::{TweenAppExt, TweenInfo, TweenPlugin, TweenService},
    utils::{EasingDirection, EasingStyle},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TweenPlugin))
        .register_tween::<Sprite>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let info = TweenInfo {
        duration: 2.0,
        easing_style: EasingStyle::Linear,
        easing_direction: EasingDirection::InOut,
        repeat_count: -1,
        reverses: true,
        delay_time: 0.5,
    };
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.2, 0.2),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(-300.0, 0.0, 0.0),
        TweenService::create(
            info,
            Transform::from_xyz(300.0, 100.0, 0.0).with_scale(Vec3::splat(2.0)),
        ),
        TweenService::create(
            info,
            Sprite {
                color: Color::srgb(0.2, 1.0, 0.2),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
        ),
    ));
}
