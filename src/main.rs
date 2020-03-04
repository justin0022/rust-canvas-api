use std::io;
use actix_web::{App, HttpServer, HttpResponse, Responder, get};
use dotenv::dotenv;
use std::env;
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use reqwest::Error;
mod api_error;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
    created_at: String,
    sortable_name: String,
    short_name: String,
    avatar_url: String
}

// fn construct_headers() -> HeaderMap {
//     let mut headers = HeaderMap::new();
//     headers.insert()
// }

async fn get_canvas_self() -> Result<User, Error> {
    dotenv().ok();

    let base_url = env::var("CANVAS_API_DOMAIN").expect("Canvas API Domain not set in .env");
    let token = env::var("CANVAS_API_TOKEN").expect("Canvas Token not set in .env");
    let endpoint = "/users/self";

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}{}", base_url, endpoint))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?
        .json::<User>()
        .await?;

    Ok(response)
}

// #[get("/canvas_self")]
// async fn canvas_self() -> Result<HttpResponse, Error> {
//     let self_response = get_canvas_self().await;
//     Ok(HttpResponse::Ok().json(self_response))
// }

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let self_response = get_canvas_self().await;
    println!("{:?}", self_response.unwrap());

    dotenv().ok();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(||
        App::new()
            // .service(canvas_self)
    );
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
