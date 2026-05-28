use mlua::{Lua, Result};
use std::fs;

fn main() -> Result<()> {
    let lua = Lua::new();

    let add_func = lua.create_function(|_, (a, b): (i32, i32)| Ok(a + b))?;

    let globals = lua.globals();
    globals.set("rust_add", add_func)?;

    let script_content =
        fs::read_to_string("luau_examples/script.luau").expect("Error while reading luau file");

    lua.load(&script_content).exec()?;

    Ok(())
}
