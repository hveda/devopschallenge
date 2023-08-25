use actix_web::{get, App, HttpResponse, HttpServer, Responder, Error};
use actix_cors::Cors;
use awc::Client;
use std::time::Duration;

const MAX_RETRIES: usize = 5;
const RETRY_DELAY: u64 = 2; // seconds

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world in service 1!")
}

#[get("/ping")]
async fn ping_service2() -> Result<HttpResponse, Error>  {
    let client = Client::new();

    for _ in 0..MAX_RETRIES {
        match client.post("http://127.0.0.1:8081/pong").send().await {
            Ok(mut response) => {
                let body = response.body().await?;
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
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8081");

        App::new()
            .wrap(cors)
            .service(hello)
            .service(ping_service2)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::{self, TestRequest};

    #[actix_rt::test]
    async fn test_hello() {
        let mut app = test::init_service(App::new().service(hello)).await;
        
        let req = TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
        
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello world in service 1!");
    }
}