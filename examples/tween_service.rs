use bevy::prelude::*;
use bevy_toolbox::tools_3d::{
    tween_service::{Tween, TweenFloat, TweenInfo, TweenPlugin},
    utils::{EasingDirection, EasingStyle},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TweenPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, sync_transform)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let start = TweenFloat(-300.0);
    let goal = TweenFloat(300.0);
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
        start,
        Tween::new(start, goal, info),
    ));
}

fn sync_transform(mut query: Query<(&TweenFloat, &mut Transform)>) {
    for (val, mut transform) in query.iter_mut() {
        transform.translation.x = val.0;
    }
}
