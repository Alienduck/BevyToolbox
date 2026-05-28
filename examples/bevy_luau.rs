use bevy::prelude::*;
use bevy_toolbox::core::{
    lua_bridge::{HandleMap, LuaCommand, LuaQueue, next_handle, process_lua_queue},
    part::{LuaColor3, LuaPart, LuaVector3},
    scheduler::{LuaScheduler, tick_scheduler},
};
use mlua::Lua;
use std::{
    fs,
    sync::{Arc, Mutex},
};

struct LuaRuntime(Lua);

fn main() {
    let queue = LuaQueue(Arc::new(Mutex::new(Vec::new())));
    let mut scheduler = LuaScheduler::new();
    let lua = Lua::new();

    register_classes(&lua, &queue);
    register_task_spawn(&lua, &mut scheduler);

    let script = fs::read_to_string("luau_advanced/startup.luau").expect("startup.luau not found");
    let thread = lua
        .create_thread(lua.load(&script).into_function().unwrap())
        .unwrap();
    scheduler.spawn(thread);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(queue)
        .insert_resource(HandleMap::default())
        .insert_non_send_resource(LuaRuntime(lua))
        .insert_non_send_resource(scheduler)
        .add_systems(Startup, startup)
        .add_systems(Update, (tick_scheduler, process_lua_queue))
        .run();
}

fn register_classes(lua: &Lua, queue: &LuaQueue) {
    let globals = lua.globals();

    let v3 = lua.create_table().unwrap();
    v3.set(
        "new",
        lua.create_function(|_, (x, y, z): (f32, f32, f32)| Ok(LuaVector3 { x, y, z }))
            .unwrap(),
    )
    .unwrap();
    globals.set("Vector3", v3).unwrap();

    let c3 = lua.create_table().unwrap();
    c3.set(
        "new",
        lua.create_function(|_, (r, g, b): (f32, f32, f32)| Ok(LuaColor3 { r, g, b }))
            .unwrap(),
    )
    .unwrap();
    globals.set("Color3", c3).unwrap();

    let q = queue.0.clone();
    let part_table = lua.create_table().unwrap();
    part_table
        .set(
            "new",
            lua.create_function(move |_, ()| {
                let handle = next_handle();
                q.lock().unwrap().push(LuaCommand::SpawnPart {
                    handle,
                    position: Vec3::ZERO,
                    size: Vec3::ONE,
                    color: Color::srgb(0.8, 0.8, 0.8),
                });
                Ok(LuaPart {
                    handle,
                    queue: q.clone(),
                })
            })
            .unwrap(),
        )
        .unwrap();
    globals.set("Part", part_table).unwrap();
}

fn register_task_spawn(lua: &Lua, scheduler: &mut LuaScheduler) {
    let task_table: mlua::Table = lua
        .globals()
        .get("task")
        .unwrap_or_else(|_| lua.create_table().unwrap());

    lua.load(
        r#"
        task = task or {}
        function task.wait(t)
            coroutine.yield("wait", t or 0)
        end
    "#,
    )
    .exec()
    .unwrap();

    lua.load(
        r#"
        function task.spawn(f, ...)
            local t = coroutine.create(f)
            coroutine.resume(t, ...)
            return t
        end
    "#,
    )
    .exec()
    .unwrap();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
