use actix_web::{self, middleware, web, App, HttpRequest, HttpServer};
use futures::future::Future;
use reqwest::{
    self,
    r#async::{Client, Response},
};

fn get_rust_posts(
    _req: HttpRequest,
    client: web::Data<Client>,
) -> impl Future<Item = String, Error = reqwest::Error> {
    client
        .get("http://www.reddit.com/r/rust.json")
        .send()
        .and_then(|resp| resp.text())
        .map_err(|err| {
            println!("Error in get rust posts: {}", err);
            err
        })
}

static CLIENT: Client = Client::new();

fn main() {
    let mut server = HttpServer::new(|| {
        App::new()
            .data(CLIENT)
            .wrap(middleware::Logger::default())
            .service(web::resource("/get/rust/posts").route(web::get().to_async(get_rust_posts)))
    });
    server.bind(("0.0.0.0", 8000)).unwrap().run().unwrap();
}
