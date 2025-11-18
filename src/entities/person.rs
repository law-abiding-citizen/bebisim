use std::sync::Arc;

use parking_lot::RwLock;
use rand::Rng;
use serde::Serialize;
use serde::ser::SerializeStruct;

use crate::entities::device::Device;

const MIN_SPEED: f32 = -1.4;
const MAX_SPEED: f32 = 1.4;

#[derive(Clone)]
pub struct Person {
    name: String,
    device: Device,
    x: Arc<RwLock<f32>>,
    y: Arc<RwLock<f32>>,
    vx: Arc<RwLock<f32>>,
    vy: Arc<RwLock<f32>>,
}

impl Person {
    pub fn new(name: String, device: Device, x: f32, y: f32) -> Self {
        Self {
            name,
            device,
            x: Arc::new(RwLock::new(x)),
            y: Arc::new(RwLock::new(y)),
            vx: Arc::new(RwLock::new(0.0)),
            vy: Arc::new(RwLock::new(0.0)),
        }
    }

    pub fn walk(&self, width: u32, height: u32) -> (f32, f32) {
        if *self.vx.read() == 0.0 && *self.vy.read() == 0.0 {
            *self.vx.write() = rand::rng().random_range(MIN_SPEED..MAX_SPEED);
            *self.vy.write() = rand::rng().random_range(MIN_SPEED..MAX_SPEED);
        }

        let new_x = *self.x.read() + *self.vx.read();
        let new_y = *self.y.read() + *self.vy.read();

        if new_x < 0.0 || new_x > (width as f32) || new_y < 0.0 || new_y > (height as f32) {
            *self.vx.write() = rand::rng().random_range(MIN_SPEED..MAX_SPEED);
            *self.vy.write() = rand::rng().random_range(MIN_SPEED..MAX_SPEED);
            return (*self.x.read(), *self.y.read());
        }

        *self.x.write() = new_x;
        *self.y.write() = new_y;

        (new_x, new_y)
    }

    pub fn x(&self) -> f32 {
        *self.x.read()
    }

    pub fn y(&self) -> f32 {
        *self.y.read()
    }
}

impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let x = *self.x.read();
        let y = *self.y.read();
        let vx = *self.vx.read();
        let vy = *self.vy.read();
        let mut state = serializer.serialize_struct("Person", 6)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("device", &self.device)?;
        state.serialize_field("x", &x)?;
        state.serialize_field("y", &y)?;
        state.serialize_field("vx", &vx)?;
        state.serialize_field("vy", &vy)?;
        state.end()
    }
}
