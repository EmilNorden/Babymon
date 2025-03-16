mod types;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::http::Response;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use crate::types::{Person, Camera};
use libcamera::{properties, camera_manager::CameraManager, logging::LoggingLevel, stream::StreamRole};

//use rascam::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    //let info = info().unwrap();
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/people", get(get_people))
        .route("/cameras", get(get_cameras))
        .layer(cors);



    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_people() -> (StatusCode, Json<Vec<Person>>) {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        },
    ];

    (StatusCode::OK, Json(people))
}

async fn get_cameras() -> (StatusCode, Json<Vec<Camera>>) {
    let mgr = CameraManager::new().unwrap();
    mgr.log_set_level("Camera", LoggingLevel::Error);
    let cameras = mgr.cameras();

    let mut cameras_result = Vec::new();
    for i in 0..cameras.len() {
        cameras_result.push(Camera {
            model: cameras.get(i).unwrap().properties().get::<properties::Model>().unwrap().to_string(),
        });
    }


    (StatusCode::OK, Json(cameras_result))
}
