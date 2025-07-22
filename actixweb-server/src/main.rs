use actix_web::{
    web, App, HttpResponse, HttpServer, Result, middleware::Logger,
};
use actix_cors::Cors;
use async_graphql::{Schema, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use shared::{
    models::*,
    auth::*,
    graphql::*,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

pub struct AppState {
    pub schema: GraphQLSchema,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "actixweb_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let schema = Schema::build(Query, Mutation, Subscription).finish();
    let app_state = web::Data::new(AppState { schema });

    tracing::info!("ActixWeb server running on http://localhost:3001");
    tracing::info!("GraphiQL playground available at http://localhost:3001/graphiql");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .route("/", web::get().to(health_check))
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(login))
                            .route("/register", web::post().to(register))
                    )
                    .service(
                        web::scope("/users")
                            .route("", web::get().to(get_users))
                            .route("/{id}", web::get().to(get_user))
                    )
                    .service(
                        web::scope("/products")
                            .route("", web::get().to(get_products))
                            .route("", web::post().to(create_product))
                            .route("/{id}", web::get().to(get_product))
                            .route("/{id}", web::put().to(update_product))
                            .route("/{id}", web::delete().to(delete_product))
                    )
                    .service(
                        web::scope("/webhooks")
                            .route("/shopify", web::post().to(handle_shopify_webhook))
                    )
            )
            .route("/graphql", web::post().to(graphql_handler))
            .route("/graphiql", web::get().to(graphiql))
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "ok",
        "framework": "actix-web",
        "timestamp": chrono::Utc::now()
    })))
}

async fn login(payload: web::Json<LoginRequest>) -> Result<HttpResponse> {
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
                        email: payload.email.clone(),
                        created_at: chrono::Utc::now(),
                    },
                };
                Ok(HttpResponse::Ok().json(response))
            }
            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        }
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

async fn register(payload: web::Json<CreateUser>) -> Result<HttpResponse> {
    // Mock implementation
    let user_id = Uuid::new_v4();
    let user_response = UserResponse {
        id: user_id,
        username: payload.username.clone(),
        email: payload.email.clone(),
        created_at: chrono::Utc::now(),
    };
    Ok(HttpResponse::Ok().json(user_response))
}

async fn get_users() -> Result<HttpResponse> {
    // Mock implementation
    let users: Vec<UserResponse> = vec![];
    Ok(HttpResponse::Ok().json(users))
}

async fn get_user(_path: web::Path<Uuid>) -> Result<HttpResponse> {
    // Mock implementation
    Ok(HttpResponse::NotFound().finish())
}

async fn get_products(_query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    // Mock implementation
    let products: Vec<Product> = vec![];
    Ok(HttpResponse::Ok().json(products))
}

async fn get_product(_path: web::Path<Uuid>) -> Result<HttpResponse> {
    // Mock implementation
    Ok(HttpResponse::NotFound().finish())
}

async fn create_product(payload: web::Json<CreateProduct>) -> Result<HttpResponse> {
    // Mock implementation
    let product = Product {
        id: Uuid::new_v4(),
        name: payload.name.clone(),
        description: payload.description.clone(),
        price: payload.price,
        inventory: payload.inventory,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    Ok(HttpResponse::Ok().json(product))
}

async fn update_product(
    _path: web::Path<Uuid>,
    _payload: web::Json<UpdateProduct>,
) -> Result<HttpResponse> {
    // Mock implementation
    Ok(HttpResponse::NotFound().finish())
}

async fn delete_product(_path: web::Path<Uuid>) -> Result<HttpResponse> {
    // Mock implementation
    Ok(HttpResponse::NoContent().finish())
}

async fn handle_shopify_webhook(payload: web::Json<Value>) -> Result<HttpResponse> {
    // Mock implementation
    tracing::info!("Received Shopify webhook: {:?}", payload);
    Ok(HttpResponse::Ok().finish())
}

async fn graphql_handler(
    schema: web::Data<AppState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}