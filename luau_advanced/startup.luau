local api = bevy_rust_api

-- print("furst cube")
-- api.spawn_cube(.0, 0, .0)
-- print("second cube")
-- api.spawn_cube(1, 0, 1)
-- print("third cube")
-- api.spawn_cube(2, 0, 2)

-- luau_advanced/startup.luau

local p = Part.new()
p.Position = Vector3.new(0, 2, 0)
p.Color    = Color3.new(1, 0.2, 0.2)

task.wait(2000)

p.Color = Color3.new(0.2, 1, 0.2)

task.wait(1)

-- p:Destroy()
