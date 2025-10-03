use uuid::Uuid;
use diesel::prelude::*;
use crate::schema::users::dsl::*;
use serde::{Deserialize, Serialize};
use crate::db::connection::establish_connection;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub is_disabled: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub is_disabled: bool,
    pub id: Option<String>,
    pub password_hash: String,
}

impl User {
    pub fn get_all_users() -> Vec<User> {
        let connection = &mut establish_connection();

        let results = users
            .limit(10)
            .select(User::as_select())
            .load(connection)
            .expect("Error loading users!");

        results
    }

    pub fn insert_user(user: NewUser) -> User {
        let connection = &mut establish_connection();

        let new_user = User {
            is_disabled: false,
            username: user.username,
            id: Uuid::new_v4().into(),
            password_hash: user.password_hash,
        };

        let inserted_user = diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(connection)
            .expect("Failed to insert user!");

        inserted_user
    }

    pub fn disable_user(disable_id: String) -> User {
        let connection = &mut establish_connection();

        let disabled_user = diesel::update(diesel::QueryDsl::filter(users,  id.eq(disable_id)))
            .set(is_disabled.eq(true))
            .get_result::<User>(connection).expect("Failed to disable user!");

        disabled_user
    }

    pub fn get_user_by_id(fetch_id: String) -> User {
        let connection = &mut establish_connection();

        let selected_user = users
            .select(User::as_select())
            .filter(id.eq(&fetch_id))
            .first::<User>(connection).expect(format!("Failed to get user by id {}", &fetch_id).as_str());

        selected_user
    }
}
