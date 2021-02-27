use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Error, Row, Statement};
use uuid::Uuid;

use crate::{errors::MyError};
use crate::user::{User, UserDraft};

pub async fn get_user(client: &Client, uuid: Uuid) -> Result<User, MyError> {
  let statement = create_statement(&client, include_str!("sql/get_user.sql")).await;

  client
    .query_opt(&statement, &[&uuid])
    .await?
    .map(&convert_row_to_user)
    .ok_or(MyError::NotFound)
}

pub async fn add_user(client: &Client, user: UserDraft) -> Result<User, Error> {
  let statement = create_statement(&client, include_str!("sql/add_user.sql")).await;

  client
    .query_one(&statement, &[&user.first_name, &user.last_name, &user.age])
    .await
    .map(&convert_row_to_user)
}

pub async fn delete_user(client: &Client, uuid: Uuid) -> Result<u64, Error> {
  let statement = create_statement(&client, include_str!("sql/delete_user.sql")).await;
  client.execute(&statement, &[&uuid]).await
}

fn convert_row_to_user(row: Row) -> User {
  User::from_row_ref(&row).unwrap()
}

async fn create_statement(client: &Client, sql_query: &str) -> Statement {
  let sql_query = sql_query.replace("$table_fields", &User::sql_table_fields());
  client.prepare(&sql_query).await.unwrap()
}
