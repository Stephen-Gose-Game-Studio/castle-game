use std::time::Duration;

#[derive(Component, Debug, Copy, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }

    pub fn as_i32(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Velocity {
    pub x: f64,
    pub y: f64
}

impl Velocity {
    pub fn new(x: f64, y: f64) -> Self {
        Velocity { x, y }
    }
}

pub struct DeltaTime(pub Duration);

impl DeltaTime {
    pub fn new(time: f64) -> Self {
        DeltaTime(Duration::from_millis((time * 1000.0) as u64))
    }

    pub fn to_seconds(&self) -> f64 {
        self.0.as_secs() as f64 + self.0.subsec_nanos() as f64 * 1e-9
    }
}

pub struct Gravity(pub f64);
