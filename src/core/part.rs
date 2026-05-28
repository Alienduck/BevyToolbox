use crate::core::lua_bridge::LuaCommand;
use mlua::{FromLua, UserData, UserDataFields, UserDataMethods};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct LuaVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FromLua for LuaVector3 {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => {
                let v = ud.borrow::<Self>()?;
                Ok(v.clone())
            }
            other => Err(mlua::Error::runtime(format!(
                "expected Vector3, got {}",
                other.type_name()
            ))),
        }
    }
}

impl UserData for LuaVector3 {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_, this| Ok(this.x));
        fields.add_field_method_get("Y", |_, this| Ok(this.y));
        fields.add_field_method_get("Z", |_, this| Ok(this.z));
        fields.add_field_method_set("X", |_, this, v: f32| {
            this.x = v;
            Ok(())
        });
        fields.add_field_method_set("Y", |_, this, v: f32| {
            this.y = v;
            Ok(())
        });
        fields.add_field_method_set("Z", |_, this, v: f32| {
            this.z = v;
            Ok(())
        });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("({}, {}, {})", this.x, this.y, this.z))
        });
    }
}

#[derive(Clone)]
pub struct LuaColor3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl FromLua for LuaColor3 {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => {
                let v = ud.borrow::<Self>()?;
                Ok(v.clone())
            }
            other => Err(mlua::Error::runtime(format!(
                "expected Color3, got {}",
                other.type_name()
            ))),
        }
    }
}

impl UserData for LuaColor3 {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("R", |_, this| Ok(this.r));
        fields.add_field_method_get("G", |_, this| Ok(this.g));
        fields.add_field_method_get("B", |_, this| Ok(this.b));
    }
}

pub struct LuaPart {
    pub handle: u64,
    pub queue: Arc<Mutex<Vec<LuaCommand>>>,
}

impl UserData for LuaPart {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_set("Position", |_, this, v: LuaVector3| {
            this.queue.lock().unwrap().push(LuaCommand::SetPosition {
                handle: this.handle,
                value: bevy::math::Vec3::new(v.x, v.y, v.z),
            });
            Ok(())
        });
        fields.add_field_method_set("Size", |_, this, v: LuaVector3| {
            this.queue.lock().unwrap().push(LuaCommand::SetSize {
                handle: this.handle,
                value: bevy::math::Vec3::new(v.x, v.y, v.z),
            });
            Ok(())
        });
        fields.add_field_method_set("Color", |_, this, c: LuaColor3| {
            this.queue.lock().unwrap().push(LuaCommand::SetColor {
                handle: this.handle,
                r: c.r,
                g: c.g,
                b: c.b,
            });
            Ok(())
        });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("Destroy", |_, this, ()| {
            this.queue.lock().unwrap().push(LuaCommand::Despawn {
                handle: this.handle,
            });
            Ok(())
        });
    }
}
