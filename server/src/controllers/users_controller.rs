use crate::models::user::{NewUser, User};

pub fn get_all() -> Vec<User> {
    User::get_all_users()
}

pub fn get_user_by_id(fetch_id: String) -> User {
    User::get_user_by_id(fetch_id)
}

pub fn disable_user(disable_id: String) -> User {
    User::disable_user(disable_id)
}

pub fn insert_user(new_user: NewUser) -> User {
    User::insert_user(new_user)
}
