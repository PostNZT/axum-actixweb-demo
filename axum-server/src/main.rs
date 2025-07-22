use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use async_graphql::{Schema, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::Html;
use shared::{
    models::*,
    auth::*,
    graphql::{Query as GraphQLQuery, Mutation, Subscription, GraphQLSchema},
};
use serde_json::{json, Value};
use std::collections::HashMap;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub schema: GraphQLSchema,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let schema = Schema::build(GraphQLQuery, Mutation, Subscription).finish();
    let state = AppState { schema };

    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/api/users", get(get_users))
        .route("/api/users/{id}", get(get_user))
        .route("/api/products", get(get_products).post(create_product))
        .route("/api/products/{id}", get(get_product).put(update_product).delete(delete_product))
        .route("/api/webhooks/shopify", post(handle_shopify_webhook))
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Axum server running on http://localhost:3000");
    tracing::info!("GraphiQL playground available at http://localhost:3000/graphiql");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "framework": "axum",
        "timestamp": chrono::Utc::now()
    }))
}

async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    // Mock implementation - in real app would validate against database
    if payload.email == "test@example.com" && payload.password == "password" {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(user_id, "testuser".to_string(), payload.email.clone());
        
        match create_jwt(&claims) {
            Ok(token) => {
                let response = LoginResponse {
                    token,
                    user: UserResponse {
                        id: user_id,
                        username: "testuser".to_string(),
                        email: payload.email,
                        created_at: chrono::Utc::now(),
                    },
                };
                Ok(Json(response))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn register(Json(payload): Json<CreateUser>) -> Result<Json<UserResponse>, StatusCode> {
    // Mock implementation
    let user_id = Uuid::new_v4();
    let user_response = UserResponse {
        id: user_id,
        username: payload.username,
        email: payload.email,
        created_at: chrono::Utc::now(),
    };
    Ok(Json(user_response))
}

async fn get_users() -> Json<Vec<UserResponse>> {
    // Mock implementation
    Json(vec![])
}

async fn get_user(Path(_id): Path<Uuid>) -> Result<Json<UserResponse>, StatusCode> {
    // Mock implementation
    Err(StatusCode::NOT_FOUND)
}

async fn get_products(Query(_params): Query<HashMap<String, String>>) -> Json<Vec<Product>> {
    // Mock implementation
    Json(vec![])
}

async fn get_product(Path(_id): Path<Uuid>) -> Result<Json<Product>, StatusCode> {
    // Mock implementation
    Err(StatusCode::NOT_FOUND)
}

async fn create_product(Json(payload): Json<CreateProduct>) -> Result<Json<Product>, StatusCode> {
    // Mock implementation
    let product = Product {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        price: payload.price,
        inventory: payload.inventory,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    Ok(Json(product))
}

async fn update_product(
    Path(_id): Path<Uuid>,
    Json(_payload): Json<UpdateProduct>,
) -> Result<Json<Product>, StatusCode> {
    // Mock implementation
    Err(StatusCode::NOT_FOUND)
}

async fn delete_product(Path(_id): Path<Uuid>) -> Result<StatusCode, StatusCode> {
    // Mock implementation
    Ok(StatusCode::NO_CONTENT)
}

async fn handle_shopify_webhook(Json(payload): Json<Value>) -> Result<StatusCode, StatusCode> {
    // Mock implementation
    tracing::info!("Received Shopify webhook: {:?}", payload);
    Ok(StatusCode::OK)
}

async fn graphql_handler(
    State(state): State<AppState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}