use actix_web::{App, get, HttpResponse, HttpServer, post, Result, web};
use actix_web::body::ResponseBody::Body;
use deadpool_postgres::Pool;
use dotenv::dotenv;
use tokio_postgres::NoTls;

use user_repository::UserRepository;

#[path = "model/user.rs"]
mod user;
mod user_repository;
mod user_dao;
mod config;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let config = crate::config::Config::from_env().unwrap();
  let pool = config.pg.create_pool(NoTls).unwrap();

  let server_address = format!("{}:{}", config.host, config.port);

  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .configure(UserRepository::get_routes())
  })
    .bind(server_address)?
    .run()
    .await
}