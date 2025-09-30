use uuid;
use serde::{Serialize, Deserialize};
use axum::{extract::Json, routing::{get, post}, Router};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Option<String>,
    username: String,
}

async fn get_user_list() -> Json<Vec<User>> {
    Json::from(vec![
        User {
            username: "NightMare".into(),
            id: Some(uuid::Uuid::new_v4().into()),
        }
    ])
}

async fn create_user(Json(mut user): Json<User>) -> Json<User> {
    user.id = Some(uuid::Uuid::new_v4().into());
    Json::from(user)
}

pub fn user_router() -> Router {
    Router::new()
        .route("/", get(get_user_list))
        .route("/", post(create_user))
}
