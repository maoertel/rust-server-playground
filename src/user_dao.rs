use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError};
use crate::user::{User, UserDraft};
use tokio_postgres::{Statement, Error};
use uuid::Uuid;


pub async fn add_user(client: &Client, user: UserDraft) -> Result<User, MyError> {
  let _stmt = include_str!("sql/add_user.sql");
  let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
  let stmt = client.prepare(&_stmt).await.unwrap();

  client
    .query(
      &stmt,
      &[
        &user.first_name,
        &user.last_name,
        &user.age,
      ],
    )
    .await?
    .iter()
    .map(|row| User::from_row_ref(row).unwrap())
    .collect::<Vec<User>>()
    .pop()
    .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn get_user(client: &Client, uuid: Uuid) -> Result<Option<User>, MyError> {
  let _stmt = include_str!("sql/get_user.sql");
  let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
  let stmt = client.prepare(&_stmt).await.unwrap();

  client
    .query(
      &stmt,
      &[&uuid],
    )
    .await?
    .iter()
    .map(|row| Some(User::from_row_ref(row).unwrap()))
    .take(1)
    .collect::<Vec<Option<User>>>()
    .pop()
    .ok_or(MyError::NotFound)
}