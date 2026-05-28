export type RustApi = {
    add: (a: number, b: number) -> number,
}

local rust_api: RustApi = {} :: RustApi
return rust_api
