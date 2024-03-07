use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn create_route() -> Router {
    Router::new()
        .route(
            "/users",
            post(|| async {
                println!("create_user");
            }),
        )
        .route(
            "/users",
            get(|| async {
                println!("query_users");
            }),
        )
        .route(
            "/users/:id",
            get(|| async {
                println!("get_user_by_id");
            }),
        )
        .route(
            "/users/:id",
            delete(|| async {
                println!("remove_user_by_id");
            }),
        )
        .route(
            "/users/:id",
            put(|| async {
                println!("update_user_by_id");
            }),
        )
}
