use axum::{
    routing::{delete, get, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::user::{
    change_password, change_role, delete_user, get_current_user, list_users,
    register_user, get_checkouts,
};

pub fn build_user_router() -> Router<AppRegistry> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/me/passoword", put(change_password))
        .route("/users/me/checkouts", get(get_checkouts))
        .route("/users", get(list_users).post(register_user))
        .route("/users/:user_id", delete(delete_user))
        .route("/users/:user_id/role", put(change_role))
}