use crate::action::Action;
use crate::entities::world::World;
use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

pub struct Server {
    port: u16,
    ws_tx: broadcast::Sender<Action>,
    world: Arc<World>,
}

impl Server {
    pub fn new(port: u16, ws_tx: broadcast::Sender<Action>, world: Arc<World>) -> Self {
        Self { port, ws_tx, world }
    }

    pub async fn start(&self) {
        println!("Starting server on port {}", self.port);

        // Create router with static file serving and WebSocket endpoint
        let world = Arc::clone(&self.world);
        let app: Router = Router::new()
            .route("/ws", get(websocket_handler))
            .nest_service("/", ServeDir::new("ui"))
            .with_state((self.ws_tx.clone(), world));

        let address = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(&address)
            .await
            .expect("Failed to bind to address");

        println!("Server listening on http://{}", address);

        axum::serve(listener, app)
            .await
            .expect("Failed to start server");
    }
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    axum::extract::State((ws_tx, world)): axum::extract::State<(
        broadcast::Sender<Action>,
        Arc<World>,
    )>,
) -> Response {
    ws.on_upgrade(move |socket| handle_websocket(socket, ws_tx.subscribe(), world))
}

async fn handle_websocket(
    socket: WebSocket,
    mut ws_rx: broadcast::Receiver<Action>,
    world: Arc<World>,
) {
    println!("WebSocket client connected");

    let (mut sender, mut receiver) = socket.split();

    // Send initial state: ConfigureWorld and all existing persons
    let configure_world = Action::ConfigureWorld {
        width: world.width(),
        height: world.height(),
    };
    if let Err(e) = sender
        .send(Message::Text(json!(&configure_world).to_string()))
        .await
    {
        eprintln!("Failed to send ConfigureWorld: {:?}", e);
        return;
    }

    // Send all existing persons
    for person in world.get_all_persons() {
        let add_person = Action::AddPerson(person);
        if let Err(e) = sender
            .send(Message::Text(json!(&add_person).to_string()))
            .await
        {
            eprintln!("Failed to send initial person: {:?}", e);
            return;
        }
    }

    let mut send_task = tokio::spawn(async move {
        while let Ok(message) = ws_rx.recv().await {
            let json = json!(&message);
            match sender.send(Message::Text(json.to_string())).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("WebSocket send error: {:?}", e);
                    break;
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Close(_)) => {
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("WebSocket receive error: {:?}", e);
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
        }
        _ = &mut recv_task => {
            send_task.abort();
        }
    }

    println!("WebSocket client disconnected");
}
