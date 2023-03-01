use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/data", post(post::post_data))
        .route("/data/:id", get(get::read_data));

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
        Json,
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
    use axum::{
        http::StatusCode,
        response::IntoResponse,
        extract::Path,
        Json,
    };
    use serde::Serialize;
    pub async fn read_data (
        // this argument tells axum to parse the request body
        // as JSON into a `CreateUser` type
        Path(mut string): Path<String>,
    ) -> impl IntoResponse {  
        // application logic   
        string.push_str(".dat");
        let file_str = string.strip_prefix(":").unwrap();
        println!("{}", &file_str);
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
