use axum::{routing::get, Router, Server, extract::State, middleware};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use dotenvy::dotenv;
use sqlx::SqlitePool;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;

mod app;
mod api;
mod db;
mod models;
mod utils;

async fn leptos_routes_handler(State(options): State<LeptosOptions>, req: Request) -> Response {
    let handler = leptos_axum::render_app_to_stream(options.to_owned(), move || view! { <app::App/> });
    handler(req).await
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(app::App);

    //  Load JWT secret from environment variable
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let jwt_secret_arc = Arc::new(jwt_secret);


    let api_routes = Router::new()
        .nest("/auth", api::auth::auth_router(jwt_secret_arc.clone())) // Mount the auth routes
        .with_state(pool.clone())
        .with_state(jwt_secret_arc.clone());

    let router = Router::new()
        .nest_service("/api", api_routes)
        .leptos_routes(&leptos_options, routes, || view! { <app::App/> })
        .with_state(leptos_options)
        .with_state(pool.clone()); // Add the database connection pool to the state


    info!("listening on http://{}", &addr);
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Task Management App (Auth)"</h1>
    }
}
