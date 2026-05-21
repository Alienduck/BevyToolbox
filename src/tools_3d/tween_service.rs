// tween_service.rs
use crate::tools_3d::utils::{EasingDirection, EasingStyle};
use bevy::{ecs::component::Mutable, math::VectorSpace, prelude::*};

pub struct TweenPlugin;

impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TweenService>();
        // Transform is always registered — it's the most common case
        app.register_tween::<Transform>();
    }
}

// Extension trait: lets users call app.register_tween::<T>() for any Tweenable
pub trait TweenAppExt {
    fn register_tween<T>(&mut self) -> &mut Self
    where
        T: Tweenable + Component + Clone + Component<Mutability = Mutable>;
}

impl TweenAppExt for App {
    fn register_tween<T>(&mut self) -> &mut Self
    where
        T: Tweenable + Component + Clone + Component<Mutability = Mutable>,
    {
        self.add_systems(Update, tween_update_system::<T>);
        self
    }
}

#[derive(Resource, Default)]
pub struct TweenService;

impl TweenService {
    pub fn create<T: Tweenable + Clone>(tween_info: TweenInfo, goal: T) -> Tween<T> {
        Tween::new(goal, tween_info)
    }
}

#[derive(Clone, Copy)]
pub struct TweenInfo {
    pub duration: f32,
    pub easing_style: EasingStyle,
    pub easing_direction: EasingDirection,
    /// -1 = infinite, 0 = play once, N = repeat N times
    pub repeat_count: i32,
    pub reverses: bool,
    pub delay_time: f32,
}

impl Default for TweenInfo {
    fn default() -> Self {
        TweenInfo {
            duration: 1.0,
            easing_style: EasingStyle::default(),
            easing_direction: EasingDirection::default(),
            repeat_count: 0,
            reverses: false,
            delay_time: 0.0,
        }
    }
}

pub trait Tweenable: Sized + Send + Sync + 'static {
    fn tween(start: Self, end: Self, a: f32) -> Self;
}

impl Tweenable for Transform {
    fn tween(start: Self, end: Self, a: f32) -> Self {
        Transform {
            translation: start.translation.lerp(end.translation, a),
            rotation: start.rotation.slerp(end.rotation, a),
            scale: start.scale.lerp(end.scale, a),
        }
    }
}

impl Tweenable for Color {
    fn tween(start: Self, end: Self, a: f32) -> Self {
        start.to_srgba().lerp(end.to_srgba(), a).into()
    }
}

impl Tweenable for Sprite {
    fn tween(start: Self, end: Self, a: f32) -> Self {
        Sprite {
            color: Color::tween(start.color, end.color, a),
            custom_size: match (start.custom_size, end.custom_size) {
                (Some(s), Some(e)) => Some(s.lerp(e, a)),
                _ => start.custom_size,
            },
            ..start
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    #[default]
    Delayed,
    Playing,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Component)]
pub struct Tween<T: Tweenable + Clone> {
    pub start: Option<T>,
    pub goal: T,
    pub info: TweenInfo,
    pub state: PlaybackState,
    pub elapsed: f32,
    pub delay_elapsed: f32,
    pub repetitions: i32,
    pub is_reversing: bool,
}

impl<T: Tweenable + Clone> Tween<T> {
    pub fn new(goal: T, info: TweenInfo) -> Self {
        Self {
            state: if info.delay_time > 0.0 {
                PlaybackState::Delayed
            } else {
                PlaybackState::Playing
            },
            start: None,
            goal,
            info,
            elapsed: 0.0,
            delay_elapsed: 0.0,
            repetitions: 0,
            is_reversing: false,
        }
    }

    pub fn play(&mut self) {
        if matches!(
            self.state,
            PlaybackState::Completed | PlaybackState::Cancelled
        ) {
            self.elapsed = 0.0;
            self.delay_elapsed = 0.0;
            self.repetitions = 0;
            self.is_reversing = false;
        }
        self.state = if self.delay_elapsed < self.info.delay_time {
            PlaybackState::Delayed
        } else {
            PlaybackState::Playing
        };
    }

    pub fn pause(&mut self) {
        if matches!(self.state, PlaybackState::Playing | PlaybackState::Delayed) {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn cancel(&mut self) {
        self.state = PlaybackState::Cancelled;
    }
}

fn apply_easing(a: f32, _style: &EasingStyle, _dir: &EasingDirection) -> f32 {
    // TODO: implement easing curves
    a
}

pub fn tween_update_system<T>(time: Res<Time>, mut query: Query<(&mut T, &mut Tween<T>)>)
where
    T: Tweenable + Component + Clone + Component<Mutability = Mutable>,
{
    let dt = time.delta_secs();

    for (mut value, mut tween) in query.iter_mut() {
        match tween.state {
            PlaybackState::Delayed => {
                tween.delay_elapsed += dt;
                if tween.delay_elapsed >= tween.info.delay_time {
                    tween.state = PlaybackState::Playing;
                }
                continue;
            }
            PlaybackState::Playing => {}
            _ => continue,
        }

        // Capture start value lazily on first frame
        if tween.start.is_none() {
            tween.start = Some((*value).clone());
        }

        if tween.is_reversing {
            tween.elapsed -= dt;
        } else {
            tween.elapsed += dt;
        }

        let raw_ratio = (tween.elapsed / tween.info.duration).clamp(0.0, 1.0);
        let cycle_finished = if tween.is_reversing {
            tween.elapsed <= 0.0
        } else {
            tween.elapsed >= tween.info.duration
        };

        let eased = apply_easing(
            raw_ratio,
            &tween.info.easing_style,
            &tween.info.easing_direction,
        );
        *value = T::tween(
            tween.start.as_ref().unwrap().clone(),
            tween.goal.clone(),
            eased,
        );

        if cycle_finished {
            if tween.info.reverses && !tween.is_reversing {
                tween.is_reversing = true;
            } else if tween.info.repeat_count < 0 || tween.repetitions < tween.info.repeat_count {
                tween.repetitions += 1;
                tween.is_reversing = false;
                tween.elapsed = 0.0;
            } else {
                tween.state = PlaybackState::Completed;
            }
        }
    }
}
