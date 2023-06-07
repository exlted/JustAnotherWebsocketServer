use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{Filter, Rejection};
mod handlers;
mod ws;
use warp::ws::Message;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type ClientMap = Arc<Mutex<HashMap<String, Client>>>;
type Result<T> = std::result::Result<T, Rejection>;

fn with_clients(clients: Arc<Mutex<HashMap<String, Client>>>) -> impl Filter<Extract = (Arc<Mutex<HashMap<String, Client>>>,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn webserver_loop(clients: ClientMap) {

    println!("Configuring websocket route");
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(handlers::ws_handler);

    println!("Starting server");
    let routes = ws_route.with(warp::cors().allow_any_origin());
    let public = warp::fs::dir("./public/");
    let server = warp::serve(routes.or(public)).run(([0, 0, 0, 0], 8080));
    
    server.await;
}

fn main() {

    let clients: ClientMap = Arc::new(Mutex::new(HashMap::new()));
    
    let webserver_clients = clients.clone();
    tokio::spawn(async move {
        webserver_loop(webserver_clients).await;
    });
}
