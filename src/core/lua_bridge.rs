// src/lua_bridge.rs
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};

static NEXT_HANDLE: AtomicU64 = AtomicU64::new(1);
pub fn next_handle() -> u64 {
    NEXT_HANDLE.fetch_add(1, Ordering::Relaxed)
}

pub enum LuaCommand {
    SpawnPart {
        handle: u64,
        position: Vec3,
        size: Vec3,
        color: Color,
    },
    SetPosition {
        handle: u64,
        value: Vec3,
    },
    SetSize {
        handle: u64,
        value: Vec3,
    },
    SetColor {
        handle: u64,
        r: f32,
        g: f32,
        b: f32,
    },
    Despawn {
        handle: u64,
    },
}

#[derive(Resource, Clone)]
pub struct LuaQueue(pub Arc<Mutex<Vec<LuaCommand>>>);

#[derive(Resource, Default)]
pub struct HandleMap(pub HashMap<u64, (Entity, Handle<StandardMaterial>)>);

pub fn process_lua_queue(
    mut commands: Commands,
    queue: Res<LuaQueue>,
    mut handle_map: ResMut<HandleMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transforms: Query<&mut Transform>,
) {
    let mut q = queue.0.lock().unwrap();
    for cmd in q.drain(..) {
        match cmd {
            LuaCommand::SpawnPart {
                handle,
                position,
                size,
                color,
            } => {
                let mat = materials.add(color);
                let entity = commands
                    .spawn((
                        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
                        MeshMaterial3d(mat.clone()),
                        Transform::from_translation(position),
                    ))
                    .id();
                handle_map.0.insert(handle, (entity, mat));
            }
            LuaCommand::SetPosition { handle, value } => {
                if let Some(&(entity, _)) = handle_map.0.get(&handle) {
                    if let Ok(mut t) = transforms.get_mut(entity) {
                        t.translation = value;
                    }
                }
            }
            LuaCommand::SetSize { handle, value } => {
                if let Some(&(entity, _)) = handle_map.0.get(&handle) {
                    if let Ok(mut t) = transforms.get_mut(entity) {
                        t.scale = value;
                    }
                }
            }
            LuaCommand::SetColor { handle, r, g, b } => {
                if let Some((_, mat_handle)) = handle_map.0.get(&handle) {
                    if let Some(mat) = materials.get_mut(mat_handle) {
                        mat.base_color = Color::srgb(r, g, b);
                    }
                }
            }
            LuaCommand::Despawn { handle } => {
                if let Some((entity, _)) = handle_map.0.remove(&handle) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
