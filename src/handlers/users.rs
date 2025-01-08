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

pub async fn fetch_user_by_id(pool: &sqlx::Pool<sqlx::Postgres>, user_id: i32) -> Result<User, sqlx::Error> {
    let row = sqlx::query_as!(
        User,
        r#"
        SELECT id, google_id, email, created_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

#[handler]
pub async fn get_users(res: &mut Response) {
    let pool = db::init_db().await.unwrap();
    let users = fetch_all_users(&pool).await.unwrap();
    res.render(Json(users));
}

#[handler]
pub async fn get_user_by_id(req: &mut Request, res: &mut Response) {
    let user_id: i32 = req.param("id").unwrap().parse().unwrap();
    let pool = db::init_db().await.unwrap();
    match fetch_user_by_id(&pool, user_id).await {
        Ok(user) => {
            res.render(Json(user));
        }
        Err(e) => {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({"error": e.to_string()})));
        }
    }
}
