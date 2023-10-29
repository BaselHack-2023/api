use actix_web::{get, web, Error, HttpResponse};
use futures::{future::ok, stream::once};

#[get("/stable")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}
