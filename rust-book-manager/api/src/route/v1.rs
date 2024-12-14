use axum::Router;
use registry::AppRegistry;

use super::{
    book::build_book_routers, health::build_healtth_check_routers,
    user::build_user_router,
};

pub fn routes() -> Router<AppRegistry> {
    let router = Router::new()
        .merge(build_healtth_check_routers())
        .merge(build_book_routers())
        .merge(build_user_router());

    Router::new().nest("/api/v1", router)
}