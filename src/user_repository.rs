use actix_web::{delete, get, HttpResponse, post, ResponseError, Result, web};
use deadpool_postgres::{Client, Pool};
use uuid::Uuid;

use crate::errors::MyError;
use crate::user::UserDraft;
use crate::user_dao::{add_user, delete_user, get_user};

#[derive(Debug)]
pub struct UserRepository {}

impl UserRepository {
  pub fn get_routes() -> impl Fn(&mut web::ServiceConfig) -> () {
    move |cfg: &mut web::ServiceConfig| {
      cfg
        .service(get_user_by_id)
        .service(create_user)
        .service(delete_user_by_id);
    }
  }
}

#[get("/user/{uuid}")]
async fn get_user_by_id(
  uuid: web::Path<String>,
  db_pool: web::Data<Pool>,
) -> Result<HttpResponse> {
  let uuid = Uuid::parse_str(uuid.into_inner().as_str()).unwrap();
  let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
  let maybe_user = get_user(&client, uuid).await;

  match maybe_user {
    Ok(user) => Ok(HttpResponse::Ok().json(user)),
    Err(error) => Ok(match error {
      MyError::NotFound => HttpResponse::NotFound().body(format!("The resource with the id {} was not found.", uuid)),
      _ => HttpResponse::InternalServerError().body(error.status_code().to_string()),
    })
  }
}

#[post("/user/create")]
async fn create_user(
  user_draft: web::Json<UserDraft>,
  db_pool: web::Data<Pool>,
) -> Result<HttpResponse> {
  let user_d = user_draft.into_inner();
  let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
  let maybe_added_user = add_user(&client, user_d).await;

  match maybe_added_user {
    Ok(user) => Ok(HttpResponse::Ok().json(user)),
    Err(_) => Ok(HttpResponse::InternalServerError().body("Something went wrong."))
  }
}

#[delete("/user/{uuid}")]
async fn delete_user_by_id(
  uuid: web::Path<String>,
  db_pool: web::Data<Pool>,
) -> Result<HttpResponse> {
  let uuid = Uuid::parse_str(uuid.into_inner().as_str()).unwrap();
  let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
  let maybe_user = delete_user(&client, uuid).await;

  match maybe_user {
    Ok(_) => Ok(HttpResponse::Ok().json(uuid)),
    _ => Ok(HttpResponse::NotFound().body(format!("User with given id {} not found. Not able to delete this resource.", uuid.to_string())))
  }
}
