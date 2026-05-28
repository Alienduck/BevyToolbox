export type BevyRustApi = {
    spawn_cube: (x: number, y: number, z: number) -> ()
}

local bevy_rust_api: BevyRustApi = {} :: BevyRustApi
return bevy_rust_api
