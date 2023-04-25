use chrono::Utc;
use uuid::Uuid;
use tracing::Instrument;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, _pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        _subscriber_email = %_form.email,
        _subscriber_name= %_form.name,
    );
    tracing::info!(
        "request id {} - Saving new subscriber details in the database",
        request_id
    );

    let request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
	"Saving new subscriber details in the database"
    );
    
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
	.instrument(query_span)
	.await
    {
        Ok(_) => {
            tracing::info!(
                "request id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
