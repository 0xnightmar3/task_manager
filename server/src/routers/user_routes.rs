use axum::{extract::{Json, Path}, routing::{get, patch, post}, Router};

use crate::models::user::{NewUser, User};
use crate::controllers::users_controller;

async fn get_user_list() -> Json<Vec<User>> {
    Json::from(users_controller::get_all())
}

async fn create_user(Json(user): Json<NewUser>) -> Json<User> {
    Json::from(users_controller::insert_user(user))
}

async fn delete_user(Json(disable_id): Json<String>) -> Json<User> {
    Json::from(users_controller::disable_user(disable_id))
}

async fn get_user_by_id(Path(user_id): Path<String>) -> Json<User> {
    Json::from(users_controller::get_user_by_id(user_id))
}

pub fn user_router() -> Router {
    Router::new()
        .route("/", get(get_user_list))
        .route("/", post(create_user))
        .route("/", patch(delete_user))
        .route("/{id}", get(get_user_by_id))
}
