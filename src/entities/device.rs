use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Device {
    pub id: String,
}

impl Device {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
