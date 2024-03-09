use axum::Router;
use std::net::SocketAddr;
use tracing::info;

pub async fn serve_app(app: Router, port: u16) {
  info!("Server listening on 127, 0, 0, 1: {}", &port);
  let local_address = SocketAddr::from(([127, 0, 0, 1], port));
  let container_address = SocketAddr::from(([0, 0, 0, 0], port));
  let addr = if cfg!(target_os = "linux") {
    container_address
  } else {
    local_address
  };
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app)
    .await
    .expect("Failed to start server");
}
