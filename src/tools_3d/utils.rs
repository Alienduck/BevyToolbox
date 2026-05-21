#[derive(Default, Clone, Copy)]
pub enum EasingStyle {
    #[default]
    Linear,
    Sine,
    Quad,
    Cubic,
    Quart,
    Quint,
    Exponential,
    Circular,
    Back,
    Bounce,
    Elastic,
}

#[derive(Default, Clone, Copy)]
pub enum EasingDirection {
    #[default]
    In,
    Out,
    InOut,
}
