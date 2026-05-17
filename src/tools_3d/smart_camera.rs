use bevy::prelude::*;

/// Different camera modes
/// TODO: add SecondPerson (follow)
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CameraMode {
    FirstPerson,
    #[default]
    ThirdPerson,
}

/// The smart camera is based on the roblox's camera
#[derive(Component)]
pub struct SmartCamera {
    /// The entity which the camera will follow, else it will be disable
    pub camera_subject: Option<Entity>,
    /// The camera mode, see [`CameraMode`]
    pub mode: CameraMode,
    /// Distance between camera and followed entity (when CameraMode is on ThirdPerson)
    pub distance: f32,
    /// Vertical angle (up/down)
    pub pitch: f32,
    /// Horizontal angle (left/right)
    pub yaw: f32,
}

impl Default for SmartCamera {
    fn default() -> Self {
        Self {
            camera_subject: None,
            mode: CameraMode::ThirdPerson,
            distance: 5.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}
pub struct SmartCameraPlugin;

impl Plugin for SmartCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_smart_camera);
    }
}

fn update_smart_camera(
    mut camera_query: Query<(&SmartCamera, &mut Transform)>,
    transforms_query: Query<&Transform, Without<SmartCamera>>,
) {
    let Ok(camera) = camera_query.single_mut() else {
        return;
    };
    let smart_camera = camera.0;
    let mut camera_transform = camera.1;
    if let Some(target_entity) = smart_camera.camera_subject {
        if let Ok(target_transform) = transforms_query.get(target_entity) {
            let rotation =
                Quat::from_euler(EulerRot::YXZ, smart_camera.yaw, smart_camera.pitch, 0.0);
            match smart_camera.mode {
                CameraMode::ThirdPerson => {
                    let offset = rotation * Vec3::new(0.0, 0.0, smart_camera.distance);
                    camera_transform.translation =
                        target_transform.translation + Vec3::new(0.0, 1.0, 0.0) + offset;
                }
                CameraMode::FirstPerson => {
                    camera_transform.translation =
                        target_transform.translation + Vec3::new(0.0, 1.5, 0.0);
                }
            }
            camera_transform.rotation = rotation;
        }
    }
}
