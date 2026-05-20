#[derive(Default)]
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

#[derive(Default)]
pub enum EasingDirection {
    #[default]
    In,
    Out,
    InOut,
}
