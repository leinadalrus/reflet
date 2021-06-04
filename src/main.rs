use actix_web::{get, post, web,
    App, HttpRequest, HttpResponse, HttpServer, Responder};

/*#[get("/")]*/
async fn body_request(req_body: String) -> impl Responder {
    /*let inp = req_body.match_info().get("/GET/source/repo/data/cluster/get").unwrap_or("/PUT/source/repo/data/cluster/put");*/
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
                        web::scope("/{app}")
                                        .route("/{index}", web::get().to(body_request))
                    )
    }).bind(("127.0.0.1", 3000))?
    .run()
    .await
}