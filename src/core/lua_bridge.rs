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

pub fn process_lua_queue(world: &mut World) {
    let queue = world.resource::<LuaQueue>().0.clone();
    let mut q = queue.lock().unwrap();
    let commands: Vec<LuaCommand> = q.drain(..).collect();
    drop(q);

    for cmd in commands {
        match cmd {
            LuaCommand::SpawnPart {
                handle,
                position,
                size,
                color,
            } => {
                let mat = world.resource_mut::<Assets<StandardMaterial>>().add(color);
                let mesh = world
                    .resource_mut::<Assets<Mesh>>()
                    .add(Cuboid::new(size.x, size.y, size.z));

                let entity = world
                    .spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(mat.clone()),
                        Transform::from_translation(position),
                    ))
                    .id();

                world
                    .resource_mut::<HandleMap>()
                    .0
                    .insert(handle, (entity, mat));
            }
            LuaCommand::SetPosition { handle, value } => {
                let entity = world
                    .resource::<HandleMap>()
                    .0
                    .get(&handle)
                    .map(|&(e, _)| e);
                if let Some(entity) = entity {
                    if let Some(mut t) = world.get_mut::<Transform>(entity) {
                        t.translation = value;
                    }
                }
            }
            LuaCommand::SetSize { handle, value } => {
                let entity = world
                    .resource::<HandleMap>()
                    .0
                    .get(&handle)
                    .map(|&(e, _)| e);
                if let Some(entity) = entity {
                    if let Some(mut t) = world.get_mut::<Transform>(entity) {
                        t.scale = value;
                    }
                }
            }
            LuaCommand::SetColor { handle, r, g, b } => {
                let mat_handle = world
                    .resource::<HandleMap>()
                    .0
                    .get(&handle)
                    .map(|(_, m)| m.clone());
                if let Some(mat_handle) = mat_handle {
                    if let Some(mat) = world
                        .resource_mut::<Assets<StandardMaterial>>()
                        .get_mut(&mat_handle)
                    {
                        mat.base_color = Color::srgb(r, g, b);
                    }
                }
            }
            LuaCommand::Despawn { handle } => {
                let entry = world.resource_mut::<HandleMap>().0.remove(&handle);
                if let Some((entity, _)) = entry {
                    world.despawn(entity);
                }
            }
        }
    }
}
