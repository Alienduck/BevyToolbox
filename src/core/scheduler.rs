use bevy::prelude::*;
use mlua::{BorrowedStr, Function, Lua, MultiValue, Thread, ThreadStatus, Value};

struct ScheduledThread {
    thread: Thread,
    resume_at: f64,
}

pub struct LuaScheduler {
    threads: Vec<ScheduledThread>,
}

impl LuaScheduler {
    pub fn new() -> Self {
        Self {
            threads: Vec::new(),
        }
    }

    pub fn spawn(&mut self, thread: Thread) {
        self.threads.push(ScheduledThread {
            thread,
            resume_at: 0.0,
        });
    }
}

pub fn register_task(lua: &Lua, scheduler: &mut LuaScheduler) {
    let task_table = lua.create_table().unwrap();

    task_table
        .set(
            "wait",
            lua.create_function(|_, t: Option<f64>| {
                let yield_fn: Function = mlua::Lua::new()
                    .globals()
                    .get("coroutine")
                    .and_then(|t: mlua::Table| t.get("yield"))
                    .unwrap_or_else(|_| panic!("coroutine.yield not found"));
                Ok(())
            })
            .unwrap(),
        )
        .unwrap();

    let wait_fn = lua
        .create_function(|lua, t: Option<f64>| {
            let coroutine: mlua::Table = lua.globals().get("coroutine")?;
            let yield_fn: Function = coroutine.get("yield")?;
            yield_fn.call::<MultiValue>(("wait", t.unwrap_or(0.0)))?;
            Ok(())
        })
        .unwrap();
    task_table.set("wait", wait_fn).unwrap();

    lua.globals().set("task", task_table).unwrap();
}

pub fn tick_scheduler(mut scheduler: NonSendMut<LuaScheduler>, time: Res<Time>) {
    let elapsed = time.elapsed_secs_f64();
    let mut i = 0;

    while i < scheduler.threads.len() {
        if scheduler.threads[i].resume_at > elapsed {
            i += 1;
            continue;
        }

        match scheduler.threads[i].thread.status() {
            ThreadStatus::Resumable => match scheduler.threads[i].thread.resume::<MultiValue>(()) {
                Ok(values) => {
                    let mut iter = values.iter();
                    let is_wait = matches!(
                        iter.next(),
                        Some(Value::String(s)) if s.to_str().unwrap() == "wait"
                    );

                    if is_wait {
                        let wait_secs = iter.next().and_then(|v| v.as_f64()).unwrap_or(0.0);
                        scheduler.threads[i].resume_at = elapsed + wait_secs;
                        i += 1;
                    } else {
                        i += 1;
                    }
                }
                Err(e) => {
                    eprintln!("[Lua error] {}", e);
                    scheduler.threads.remove(i);
                }
            },
            ThreadStatus::Error => {
                scheduler.threads.remove(i);
            }
            _ => {
                i += 1;
            }
        }
    }
}
