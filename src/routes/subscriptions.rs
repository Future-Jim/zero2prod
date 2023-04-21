
use chrono::Utc;
use uuid::Uuid;

use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    _form: web::Form<FormData>,
    _pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
	"request_id {} - Adding '{}' '{}' as a new subscriber.",
	request_id,
	_form.email,
	_form.name,
    );
    log::info!("request id {} - Saving new subscriber details in the database",
	       request_id);
    
    match sqlx::query!(
	r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_pool.get_ref())
    .await
    {
	Ok(_) => {
	    log::info!("request id {} - New subscriber details have been saved",
	    request_id);
	    HttpResponse::Ok().finish()},
	Err(e) => {
	    log::error!("request id {} - Failed to execute query: {:?}",
			request_id,
			e);
	    HttpResponse::InternalServerError().finish()
	}
    }
}
