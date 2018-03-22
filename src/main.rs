extern crate actix;
extern crate actix_web;

use actix_web::*;

fn main() {
    let sys = actix::System::new("Hello actix");
    let _addr = HttpServer::new(
        || Application::new()
            .resource("/", |r| r.f(|_| "Hello actix-web")))
    .bind("127.0.0.1:8080").unwrap()
    .start();
    let _ = sys.run();
}
