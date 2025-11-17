use tiny_http::{Server, Response};

#[tokio::main]
async fn main() {
    println!("Running server on port 25555 ...");

    let server = Server::http("0.0.0.0:25555").unwrap();

    for request in server.incoming_requests() {
        let text = match request.url() {
            "/hello" => "Hello, world!".to_string(),
            "/naber" => "iyidir".to_string(),
            _ => "404 Not Found".to_string(),
        };

        if request.url() != "/favicon.ico" {
            println!("Received request: {:?}", request.url());
        }

        let response = Response::from_string(text);
        request.respond(response).unwrap();
    }

    println!("Server stopped.");
}
