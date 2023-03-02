use axum::{
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, time::Instant};
use std::sync::Arc;
use tokio::sync::Mutex;


#[tokio::main]
async fn main() {
    let initialized:Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let last_get:Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    let cached_data:Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    // build our application with a route
    let app = Router::new()
        .route("/data", post(post::post_data))
        .route("/data/:id", get(get::read_data))
        .with_state((initialized, last_get, cached_data));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("RUNNING: {:?}", &addr);
    
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub mod post {
    use axum::{
        http::StatusCode,
        response::IntoResponse,
        Json
    };
    use serde::{Deserialize, Serialize};
    pub async fn post_data (
        // this argument tells axum to parse the request body
        // as JSON into a `CreateUser` type
        Json(payload): Json<Data>,
    ) -> impl IntoResponse {
        // application logic    
        let confirm = ConfirmPost {
            val: payload.val,
            id: payload.id.clone(),
        };
        let id = payload.id;
        if let Ok(_) = std::fs::write(format!("{id}.dat"), payload.val.to_le_bytes()) {
            (StatusCode::CREATED, Json(confirm))
        } else {
            (StatusCode::UNPROCESSABLE_ENTITY, Json(confirm))
        }
    }
    // the input to our `post_data` handler
    #[derive(Deserialize)]
    pub struct Data {
        val: u32,
        id: String,
    }
    // the output to our `post_data` handler
    #[derive(Serialize)]
    pub struct ConfirmPost {
        val: u32,
        id: String,
    }
}

pub mod get {
    use std::time::Instant;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use axum::{
        http::StatusCode,
        response::IntoResponse,
        extract::{Path, State},
        Json,
    };
    use serde::Serialize;

    pub async fn read_data (
        // this argument tells axum to parse the request body
        // as JSON into a `CreateUser` type
        Path(string): Path<String>,
        State((initialized, last_get, cached_data)): State<(Arc<Mutex<bool>>, Arc<Mutex<Instant>>, Arc<Mutex<u32>>)>,
    ) -> impl IntoResponse {  
        // cache
        if initialized.lock().await.clone() && last_get.lock().await.elapsed().as_secs() < 30 {
            read_data_with_cache(cached_data).await
        } else {
            let mut n = initialized.lock().await;
            *n = true;
            read_data_without_cache(string, cached_data).await
        }
    }
    
    pub async fn read_data_with_cache(cached_data: Arc<Mutex<u32>>) -> (StatusCode, axum::Json<Response>) {
        let response = Response {
            data: cached_data.lock().await.clone(),
            cached: true,
        };
        (StatusCode::OK, Json(response))
        
    }
    
    pub async fn read_data_without_cache(mut string: String, cached_data: Arc<Mutex<u32>>) -> (StatusCode, axum::Json<Response>) {
        string.push_str(".dat");
        let file_str = string.strip_prefix(":").unwrap();
        if let Ok(data) = std::fs::read_to_string(file_str) {
            let mut iter = data.as_bytes().iter();
            let (b1, b2, b3, b4) = (
                iter.next().unwrap().clone(), 
                iter.next().unwrap().clone(), 
                iter.next().unwrap().clone(), 
                iter.next().unwrap().clone(),
            );
            
            let data = u32::from_le_bytes([
                b1, b2, b3, b4
            ]);
            let mut n = cached_data.lock().await;
            *n = data.clone();
            
            let response = Response {
                data,
                cached: false,
            };
            (StatusCode::OK, Json(response))
        } else {
            let response = Response {
                data: 0,
                cached: false,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }

    
    // the output to our `post_data` handler
    #[derive(Serialize)]
    pub struct Response {
        data: u32,
        cached: bool,
    }
}
