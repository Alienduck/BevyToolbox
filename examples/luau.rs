use mlua::{Lua, Result};
use std::fs;

fn main() -> Result<()> {
    let lua = Lua::new();

    let add_func = lua.create_function(|_, (a, b): (i32, i32)| Ok(a + b))?;
    let globals = lua.globals();
    globals.set("rust_add", add_func)?;

    let require_fn = lua.create_function(|lua, module_path: String| {
        let normalized = module_path.strip_prefix("./").unwrap_or(&module_path);

        let path = format!("luau_examples/{}.luau", normalized);

        let source = fs::read_to_string(&path).map_err(|e| {
            mlua::Error::RuntimeError(format!("require failed for '{}': {}", path, e))
        })?;

        let chunk = lua.load(&source).set_name(&path);
        chunk.eval::<mlua::Value>()
    })?;

    globals.set("require", require_fn)?;

    let rust_api = lua.create_table()?;
    rust_api.set("add", lua.globals().get::<mlua::Function>("rust_add")?)?;
    globals.set("rust_api", rust_api)?;

    let script_content =
        fs::read_to_string("luau_examples/script.luau").expect("Error reading luau script");

    lua.load(&script_content).exec()?;

    Ok(())
}
