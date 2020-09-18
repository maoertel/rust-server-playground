use actix_web::{App, get, HttpResponse, HttpServer, post, Result, web, ResponseError};
use crate::user_dao::{add_user, get_user};
use deadpool_postgres::{Client, Pool};
use crate::errors::MyError;
use crate::user::{UserDraft, User};
use uuid::Uuid;


#[derive(Debug)]
pub struct UserRepository {
  pub data_base: String,
}

impl UserRepository {
  pub fn get_routes() -> impl Fn(&mut web::ServiceConfig) -> () {
    move |cfg: &mut web::ServiceConfig| {
      cfg
        .service(get_user_by_id)
        .service(create_user);
    }
  }
}

#[post("/user/create")]
async fn create_user(
  user_draft: web::Json<UserDraft>,
  db_pool: web::Data<Pool>
) -> Result<HttpResponse> {
  let user_d = user_draft.into_inner();
  let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
  let maybe_added_user = add_user(&client, user_d).await;

  match maybe_added_user {
    Ok(user) => Ok(HttpResponse::Ok().json(user)),
    Err(error) => Ok(HttpResponse::NotFound().body(error.status_code().to_string()))
  }
}

#[get("/user/{uuid_path}")]
async fn get_user_by_id(
  uuid_path: web::Path<String>,
  db_pool: web::Data<Pool>
) -> Result<HttpResponse> {
  let uuid = Uuid::parse_str(uuid_path.into_inner().as_str()).unwrap();
  let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
  let maybe_user = get_user(&client, uuid).await;

  match maybe_user {
    Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
    _ => Ok(HttpResponse::NotFound().body(format!("User with given id {} not found.", uuid.to_string())))
  }
}
