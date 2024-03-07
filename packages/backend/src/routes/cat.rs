use axum::{
    routing::{delete, get, post, put},
    Router,
};
pub fn create_route() -> Router {
    Router::new()
        .route(
            "/cats",
            post(|| async {
                println!("create_cat");
            }),
        )
        .route(
            "/cats",
            get(|| async {
                println!("query_cats");
            }),
        )
        .route(
            "/cats/:id",
            get(|| async {
                println!("get_cat_by_id");
            }),
        )
        .route(
            "/cats/:id",
            delete(|| async {
                println!("remove_cat_by_id");
            }),
        )
        .route(
            "/cats/:id",
            put(|| async {
                println!("update_cat_by_id");
            }),
        )
}
