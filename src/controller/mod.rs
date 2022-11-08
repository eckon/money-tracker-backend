use axum::Router;

pub mod account;
pub mod cost;
pub mod payment;

pub fn app() -> Router {
    Router::new()
        .merge(account::app())
        .merge(cost::app())
        .merge(payment::app())
}
