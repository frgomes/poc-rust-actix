#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(
                actix_web::web::resource("/table_update")
                    .route(actix_web::web::post()
                           .to(poc_rust_actix::apis::table::table_update)))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
