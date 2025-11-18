use crate::entities::person::Person;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum Action {
    ConfigureWorld { width: u32, height: u32 },
    SetTime(u64),
    AddPerson(Person),
    WalkPerson { x: f32, y: f32 },
}
