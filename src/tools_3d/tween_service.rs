use bevy::{ecs::component::Mutable, prelude::*};

use crate::tools_3d::utils::{EasingDirection, EasingStyle};

pub struct TweenPlugin;

impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TweenService>()
            .add_systems(Update, tween_update_system::<TweenFloat>);
    }
}

#[derive(Resource, Default)]
pub struct TweenService;

impl TweenService {
    pub fn create<T: Tweenable>(value: T, tween_info: TweenInfo, goal: T) -> Tween<T> {
        Tween::new(value, goal, tween_info)
    }
}

pub struct TweenInfo {
    /// Duration of the tween in seconds
    pub duration: f32,
    /// The style of the tween execution
    pub easing_style: EasingStyle,
    /// The direction of the tween execution
    pub easing_direction: EasingDirection,
    /// Number of the tween's repetition
    pub repeat_count: i32,
    /// If the tween need to play back when it's finished
    pub reverses: bool,
    /// The time before the tween starts
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

pub trait Tweenable {
    fn tween(start: Self, end: Self, a: f32) -> Self;
}

impl Tweenable for f64 {
    fn tween(start: Self, end: Self, a: f32) -> Self {
        start + (end - start) * a as f64
    }
}

#[derive(Component, Clone, Copy)]
pub struct TweenFloat(pub f32);

impl Tweenable for TweenFloat {
    fn tween(start: Self, end: Self, a: f32) -> Self {
        Self(start.0 + (end.0 - start.0) * a)
    }
}

impl Tweenable for i128 {
    fn tween(start: Self, end: Self, a: f32) -> Self {
        (start as f32 + ((end - start) as f32 * a)) as i128
    }
}

impl Tweenable for i64 {
    fn tween(start: Self, end: Self, a: f32) -> Self {
        (start as f32 + ((end - start) as f32 * a)) as i64
    }
}

impl Tweenable for i32 {
    fn tween(start: Self, end: Self, a: f32) -> Self {
        (start as f32 + ((end - start) as f32 * a)) as i32
    }
}

impl Tweenable for i16 {
    fn tween(start: Self, end: Self, a: f32) -> Self {
        (start as f32 + ((end - start) as f32 * a)) as i16
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
pub struct Tween<T: Tweenable> {
    pub start: T,
    pub goal: T,
    pub info: TweenInfo,
    pub state: PlaybackState,
    pub elapsed: f32,
    pub delay_elapsed: f32,
    pub repetitions: i32,
    pub is_reversing: bool,
}

impl<T: Tweenable> Tween<T> {
    pub fn new(start: T, goal: T, info: TweenInfo) -> Self {
        let state = if info.delay_time > 0.0 {
            PlaybackState::Delayed
        } else {
            PlaybackState::Playing
        };
        Self {
            start,
            goal,
            info,
            state,
            elapsed: 0.0,
            delay_elapsed: 0.0,
            repetitions: 0,
            is_reversing: false,
        }
    }

    pub fn play(&mut self) {
        if self.state == PlaybackState::Completed || self.state == PlaybackState::Cancelled {
            self.elapsed = 0.0;
            self.delay_elapsed = 0.0;
            self.repetitions = 0;
            self.is_reversing = false;
        }
        if self.delay_elapsed < self.info.delay_time {
            self.state = PlaybackState::Delayed;
        } else {
            self.state = PlaybackState::Playing;
        }
    }

    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing || self.state == PlaybackState::Delayed {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn cancel(&mut self) {
        self.state = PlaybackState::Cancelled;
    }
}

fn apply_easing(a: f32, _style: &EasingStyle, _dir: &EasingDirection) -> f32 {
    a // TODO: Math logic
}

pub fn tween_update_system<T>(time: Res<Time>, mut query: Query<(&mut T, &mut Tween<T>)>)
where
    T: Tweenable + Component + Copy + Component<Mutability = Mutable>,
{
    let dt = time.delta_secs();
    for (mut value, mut tween) in query.iter_mut() {
        if tween.state == PlaybackState::Delayed {
            tween.delay_elapsed += dt;
            if tween.delay_elapsed >= tween.info.delay_time {
                tween.state = PlaybackState::Playing;
            }
            continue;
        }
        if tween.state != PlaybackState::Playing {
            continue;
        }
        if tween.is_reversing {
            tween.elapsed -= dt;
        } else {
            tween.elapsed += dt;
        }
        let mut raw_ratio = tween.elapsed / tween.info.duration;
        let mut cycle_finished = false;
        if !tween.is_reversing && raw_ratio >= 1.0 {
            raw_ratio = 1.0;
            cycle_finished = true;
        } else if tween.is_reversing && raw_ratio <= 0.0 {
            raw_ratio = 0.0;
            cycle_finished = true;
        }
        let eased_ratio = apply_easing(
            raw_ratio,
            &tween.info.easing_style,
            &tween.info.easing_direction,
        );
        *value = T::tween(tween.start, tween.goal, eased_ratio);
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
