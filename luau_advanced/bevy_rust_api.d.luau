export type Vector3 = {
    x: number,
    y: number,
    z: number,
}

export type Color3 = {
    r: number,
    g: number,
    b: number,
}

export type Part = {
    Position: Vector3,
    Color: Color3,
    Size: Vector3,
    Destroy: (self: Part) -> (),
}

declare Vector3: {
    new: (x: number, y: number, z: number) -> Vector3,
}

declare Color3: {
    new: (r: number, g: number, b: number) -> Color3,
}

declare Part: {
    new: () -> Part,
}

declare bevy_rust_api: {}
