// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use actix_files;
use actix_web::{web, get, App, HttpRequest, HttpServer, Responder, HttpResponse, post, Result};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body + "OKB'S test!!!")
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }



#[get("/test")]
async fn test(_req: HttpRequest) -> impl Responder {
    "hello Welcome!Test!!"
}

async fn index() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("target/public/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load ssl keys
    // to create a self-signed temporary cert for testing:
    // $ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
    //   -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
    // $ openssl rsa -in key.pem -out nopass.pem
    // $ cargo watch -x 'run --bin test_web1'
    // https://actix.rs/docs/getting-started/
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("./src/cert/nopass.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("./src/cert/cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
        // ルートにアクセスされたときは`index.html`を返す
        .route("/", web::get().to(index))
        .service(test)
        .service(echo)
    })
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}