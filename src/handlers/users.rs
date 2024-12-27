use salvo::prelude::*;
use crate::db;
use crate::models::user::User;

pub async fn fetch_all_users(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<User>, sqlx::Error> {
  let rows = sqlx::query_as!(
    User,
    r#"
    SELECT id, google_id, email, created_at
    FROM users
    "#
  )
  .fetch_all(pool)
  .await?;
  Ok(rows)
}

#[handler]
pub async fn get_users(res: &mut Response) {
    let pool = db::init_db().await.unwrap();
    let users = fetch_all_users(&pool).await.unwrap();
    res.render(Json(users));
}
