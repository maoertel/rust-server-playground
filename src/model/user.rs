use actix_web::{Error, HttpRequest, HttpResponse, Responder, Result};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserDraft {
  pub first_name: String,
  pub last_name: String,
  pub age: i32,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
  pub uuid: Uuid,
  pub first_name: String,
  pub last_name: String,
  pub age: i32,
}

impl Responder for User {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();

    // Create response and set content type
    ready(Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(body)))
  }
}