mod action;
mod constants;
mod entities;
mod server;

use std::sync::Arc;
use std::time::Duration;

use crate::action::Action;
use crate::entities::world::World;
use crate::server::Server;
use tokio::sync::broadcast;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("Running server on port 25555 ...");

    let (ws_tx, _rx) = broadcast::channel::<Action>(100);

    let world = Arc::new(World::new(ws_tx.clone()));
    let world_for_server = Arc::clone(&world);

    let server = Server::new(25555, ws_tx.clone(), world_for_server);
    let server_handle = tokio::spawn(async move {
        server.start().await;
    });

    sleep(Duration::from_secs(3)).await;

    let world_handle = tokio::spawn(async move {
        world.run().await;
    });

    match tokio::join!(server_handle, world_handle) {
        (Ok(_), Ok(_)) => println!("Gracefully shutdown"),
        (Err(e), _) => println!("Server error: {:?}", e),
        (_, Err(e)) => println!("World error: {:?}", e),
    }
}
