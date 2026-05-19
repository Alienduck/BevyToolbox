use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_toolbox::tools_3d::smart_camera::{
    CameraInput, CameraMode, SmartCamera, SmartCameraPlugin, world_to_screen_point,
};

fn main() {
    let mut app = App::default();
    app.add_plugins((DefaultPlugins, SmartCameraPlugin))
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                player_move,
                toggle_camera_mode,
                update_point,
                rotating,
                change_camera_distance,
            ),
        )
        .run();
}

#[derive(Component)]
struct ScreenPoint;

#[derive(Component)]
struct Maxwell {
    is_rotating: bool,
}

#[derive(Component)]
struct Player {
    speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player { speed: 2.0 }
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        SceneRoot(asset_server.load("models/DingusTheCat.glb#Scene0")),
        Transform::default(),
        Maxwell { is_rotating: false },
    ));
    let player = commands
        .spawn((
            Transform::default(),
            Player::default(),
            Mesh3d(meshes.add(Capsule3d::new(0.5, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
        ))
        .id();
    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        SmartCamera {
            camera_subject: Some(player),
            distance: 5.0,
            mode: CameraMode::ThirdPerson,
            motion_on_input: false,
            custom_input: Some(CameraInput::Mouse(MouseButton::Back)),
            mouse_lock_enabled: true,
            ..default()
        },
    ));
    commands.spawn((
        Node {
            top: Val::Percent(50.0),
            left: Val::Percent(50.0),
            padding: UiRect::all(Val::Px(15.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Percent(100.0)),
            ..default()
        },
        BackgroundColor(Color::Srgba(Srgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.25,
        })),
        Visibility::Inherited,
        ScreenPoint,
        children![(
            Text("Ce fdp de Willi wonka".into()),
            TextFont {
                font_size: 24.0,
                ..default()
            }
        )],
    ));
}

fn rotating(
    mut maxwell_query: Query<(&mut Transform, &mut Maxwell)>,
    inputs: Res<ButtonInput<KeyCode>>,
    delta: Res<Time>,
) {
    let Ok((mut transform, mut maxwell)) = maxwell_query.single_mut() else {
        return;
    };
    if inputs.just_pressed(KeyCode::KeyQ) {
        maxwell.is_rotating = !maxwell.is_rotating;
    }
    if maxwell.is_rotating {
        transform.rotate_y(561651.0 * delta.delta_secs());
    }
}

fn player_move(
    mut player_query: Query<(&mut Transform, &Player), Without<SmartCamera>>,
    camera_query: Query<&Transform, With<SmartCamera>>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut player_transform, player)) = player_query.single_mut() else {
        return;
    };

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let forward = camera_transform.forward();
    let right = camera_transform.right();

    let flat_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let flat_right = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    let mut movement = Vec3::ZERO;

    if inputs.pressed(KeyCode::KeyW) {
        movement += flat_forward;
    }
    if inputs.pressed(KeyCode::KeyS) {
        movement -= flat_forward;
    }
    if inputs.pressed(KeyCode::KeyA) {
        movement -= flat_right;
    }
    if inputs.pressed(KeyCode::KeyD) {
        movement += flat_right;
    }

    movement = movement.normalize_or_zero();
    player_transform.translation += movement * player.speed * 0.02;
}

fn toggle_camera_mode(
    mut camera_query: Query<&mut SmartCamera>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };
    if inputs.just_pressed(KeyCode::KeyF) {
        camera.mode = match camera.mode {
            CameraMode::FirstPerson => CameraMode::ThirdPerson,
            CameraMode::ThirdPerson => CameraMode::FirstPerson,
        }
    }
}

fn update_point(
    mut point_query: Query<(&mut Node, &mut Visibility), With<ScreenPoint>>,
    camera_query: Query<(&GlobalTransform, &Camera), With<SmartCamera>>,
    maxwell_query: Query<&Transform, With<Maxwell>>,
    window_query: Query<&Window>,
) {
    let Ok((mut point, mut visibility)) = point_query.single_mut() else {
        return;
    };
    let Ok((camera_transform, camera)) = camera_query.single() else {
        return;
    };
    let Ok(maxwell_transform) = maxwell_query.single() else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };
    let (vector, on_screen) =
        world_to_screen_point(camera, camera_transform, maxwell_transform.translation);

    if on_screen {
        *visibility = Visibility::Visible;
        if vector.x < window.width() / 2_f32 {
            point.left = Val::Px(vector.x);
        } else {
            point.right = Val::Px(vector.x);
        }
        point.top = Val::Px(vector.y);
    } else {
        *visibility = Visibility::Hidden;
    }
}

fn change_camera_distance(
    mut camera_query: Query<&mut SmartCamera>,
    mut mouse_motion: MessageReader<MouseWheel>,
) {
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };
    for msg in mouse_motion.read() {
        camera.distance -= msg.y * 0.25;
    }
}
