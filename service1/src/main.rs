use actix_web::{get, App, HttpResponse, HttpServer, Responder, Error};
use actix_cors::Cors;
use awc::Client;
use mongodb::{Client as MongoClient, options::ClientOptions};
use std::time::Duration;

const MAX_RETRIES: usize = 5;
const RETRY_DELAY: u64 = 2; // seconds

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world in service 1!")
}

#[get("/ping")]
async fn ping_service2() -> Result<HttpResponse, Error> {
    // Connect to MongoDB
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI not set");
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let mongo_client = MongoClient::with_options(client_options).unwrap();
    let db_name = std::env::var("DB_NAME").expect("DB_NAME not set");
    let _db = mongo_client.database(&db_name);

    let client = Client::new();

    for _ in 0..MAX_RETRIES {
        match client.post("http://service2:8081/pong").send().await {
            Ok(mut response) => {
                let body = response.body().await?;
                // Use MongoDB here, for example, insert data into a collection
                // db.collection("your_collection_name").insert_one(...).await.unwrap();
                return Ok(HttpResponse::Ok().body(body));
            },
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(RETRY_DELAY)).await;
            }
        }
    }

    Err(actix_web::error::ErrorInternalServerError("Service2 is not responding"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Your MongoDB connection setup is here

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1")
            .allowed_origin("http://service2") // Add additional origins here
            .allowed_origin("http://pleno.earth");

        App::new()
            .wrap(cors)
            .service(hello)
            .service(ping_service2)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
