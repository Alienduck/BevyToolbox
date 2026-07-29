use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, inputs)
        .run();
}

#[derive(Component)]
struct MyCube;

fn startup(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(mesh.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
        Transform::default(),
        MyCube,
    ));

    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Transform::from_xyz(5.0, 2.0, 5.0).looking_at(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3::Y,
        ),
    ));
}

fn inputs(
    inputs: Res<ButtonInput<KeyCode>>,
    query: Query<&mut MeshMaterial3d<StandardMaterial>, With<MyCube>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    if inputs.just_pressed(KeyCode::KeyF) {
        change_color(query, materials, Color::srgb(0.2, 1.0, 0.2));
    }
}

fn change_color(
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>, With<MyCube>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    new_color: Color,
) {
    let Ok(mut cube_mat) = query.single_mut() else {
        return;
    };
    *cube_mat = MeshMaterial3d(materials.add(new_color));
}
