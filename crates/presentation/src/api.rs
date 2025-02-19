use crate::{error::api::ApiError, handlers::ping::ping_handler, routes};
use async_std::sync::{Arc, RwLock};
use axum::{
    extract::DefaultBodyLimit,
    http::{header, Method},
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

//axum
pub async fn api() -> Result<(), ApiError> {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // Shared Object
    let shared_state = Arc::new(RwLock::new(
        SharedStateUseCase::new(SharedState::new().await)
            .await
            .shared_state_factory,
    ));
    // CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .expose_headers([header::CONTENT_DISPOSITION])
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(Any);
    // Router
    let app: Router<()> = Router::new()
        .route("/", get(ping_handler))
        .merge(routes::root::root_route())
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)) //100MB
        .with_state(Arc::clone(&shared_state));
    // Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7000").await?;
    tracing::debug!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

// #[derive(OpenApi)]
// #[openapi(
//     info(
//         title = "dashi-image-server",
//         version = "0.0.1",
//         description = "This is a dashi-image-server API document.",
//         contact(
//             name = "Myxogastria0808",
//             email = "r.rstudio.c@gmail.com",
//             url = "https://yukiosada.work",
//         ),
//         license(
//             name = "WTFPL",
//             url = "http://www.wtfpl.net"
//         ),
//     ),
//     servers((url = "http://0.0.0.0:7000")),
//     tags(
//         (name = "Image", description = "画像を送るエンドポイント"),
//         (name = "Health Check", description = "Health Checkのエンドポイント"),
//     ),
//     paths(
//     ),
//     components(schemas(
//     ))
// )]
// struct ApiDoc;
