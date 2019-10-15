use actix_web::{self, middleware, web, App, HttpResponse, HttpRequest, HttpServer};
use futures::future::Future;
use reqwest;
use reqwest::{Client};

static POSTS: &str = "https://www.reddit.com/r/rust.json";
static POSTS_SLOWWLY: &str = "http://slowwly.robertomurray.co.uk/delay/5000/url/https://www.reddit.com/r/rust.json";

fn get_request(builder: reqwest::RequestBuilder) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    actix_web::web::block(move || builder.send())
    .from_err()
    .and_then(|mut resp| {
        match resp.text() {
            Ok(body) => HttpResponse::Ok().content_type("application/json").body(body),
            Err(err) => {
                println!("get_request error: {}", err);
                HttpResponse::InternalServerError().content_type("application/json").body("{{\"error\": \"Error with get request.\"}}")
            }
        }
        
    })
}

fn get_rust_posts(
    _req: HttpRequest,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let builder = client.get(POSTS);
    get_request(builder)
}

fn get_rust_posts_slowwly(
    _req: HttpRequest,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let builder = client.get(POSTS_SLOWWLY);
    get_request(builder)
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .data(Client::new())
            .wrap(middleware::Logger::default())
            .service(web::resource("/get/rust/posts").route(web::get().to_async(get_rust_posts)))
            .service(web::resource("/get/rust/posts/slowwly").route(web::get().to_async(get_rust_posts_slowwly)))
    });
    server.bind(("0.0.0.0", 8000)).unwrap().run().unwrap();
}
