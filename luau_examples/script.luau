local module = require("./module")

local function main()
    print(rust_api.add(1, 1))
    print(module.something)
end

main()
