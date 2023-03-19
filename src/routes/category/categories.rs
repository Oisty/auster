use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use actix_web::http::StatusCode;
use sqlx::{Error, SqlitePool};
use crate::models::Category;


#[tracing::instrument(name = "Get stored categories", skip(pool))]
pub async fn list_categories(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, InternalError<Error>> {
    match get_all_categories(&pool).await {
        Ok(categories) => {
            Ok(HttpResponse::Ok().json(categories))
        }
        Err(err) => {
            tracing::error!("Failed to retrieve categories: {:?}", err);
            Err(InternalError::new(
                err,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

async fn get_all_categories(pool: &SqlitePool) -> Result<Vec<Category>, Error> {
    let rows = sqlx::query!(
        r#"
        SELECT category_id as "category_id: String", name as "name: String"
        FROM categories
        "#
    )
        .fetch_all(pool)
        .await?;

    let categories = rows
        .into_iter()
        .map(|row| Category {
            category_id: row.category_id.as_ref().unwrap().to_string(),
            name: row.name,
        })
        .collect();

    Ok(categories)
}

