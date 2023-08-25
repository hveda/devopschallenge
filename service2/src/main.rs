use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world in service 2!")
}

#[post("/pong")]
async fn pong() -> impl Responder {
    HttpResponse::Ok().body("Pong from service2!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080");

        App::new()
            .wrap(cors)
            .service(hello)
            .service(pong)
    })
    .bind("127.0.0.1:8081")?
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
        assert_eq!(body, "Hello world in service 2!");
    }
}
