use actix_web::{middleware, web, App, HttpRequest, HttpServer};
// use db::{Migrator, MigratorTrait};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let connection = sea_orm::Database::connect(&database_url).await?;
    // Migrator::up(&connection, None).await?;


    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

postgres://kontrol:kontrolpwd@localhost:5432/kontroldb
