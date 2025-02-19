use crate::api::api;

mod api;
pub mod error;
pub mod handlers;
pub mod routes;

#[tokio::main]
async fn main() {
    let _ = api().await;
}
