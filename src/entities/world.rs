use crate::action::Action;
use crate::constants::names::EUROPEAN_NAMES;
use crate::entities::device::Device;
use crate::entities::person::Person;
use parking_lot::RwLock;
use rand::Rng;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use tokio::sync::broadcast;
use tokio::time::{Duration, sleep};

pub const FRAME_SPEED: u64 = 50;

pub struct World {
    ws_tx: broadcast::Sender<Action>,
    frame_counter: Arc<AtomicU64>,
    persons: Arc<RwLock<Vec<Person>>>,
    width: u32,
    height: u32,
    time: Instant,
}

impl World {
    pub fn new(ws_tx: broadcast::Sender<Action>) -> Self {
        println!("World created");

        Self {
            ws_tx,
            frame_counter: Arc::new(AtomicU64::new(0)),
            persons: Arc::new(RwLock::new(vec![])),
            width: 550,
            height: 830,
            time: Instant::now(),
        }
    }

    pub async fn run(&self) {
        println!("World is running");

        for _ in 0..10 {
            self.add_person();
        }

        loop {
            self.process_frame().await;
        }
    }

    pub async fn process_frame(&self) {
        self.frame_counter.fetch_add(1, Ordering::SeqCst);

        for person in self.persons.read().iter() {
            let (x, y) = person.walk(self.width, self.height);
            let _ = self.ws_tx.send(Action::WalkPerson { x, y });
        }

        let _ = self.ws_tx.send(Action::SetTime(
            self.time.elapsed().as_millis() as u64 * FRAME_SPEED / 1000,
        ));

        sleep(Duration::from_millis(1000 / FRAME_SPEED)).await;
    }

    pub fn add_person(&self) {
        let device = Device::new(self.persons.read().len().to_string());

        let mut rng = rand::rng();
        let n = rng.random_range(0..1240);
        let name = EUROPEAN_NAMES[n].to_string();

        let x = rng.random_range(0f32..(self.width as f32));
        let y = rng.random_range(0f32..(self.height as f32));

        let person = Person::new(name, device, x, y);

        let mut persons = self.persons.write();
        persons.push(person.clone());

        let _ = self.ws_tx.send(Action::AddPerson(person));
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_all_persons(&self) -> Vec<Person> {
        self.persons.read().clone()
    }
}
