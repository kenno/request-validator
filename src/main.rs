use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::Html,
    routing::post,
    Json, RequestExt, Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use validator::Validate;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/user", post(handler));

    // run in
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

#[async_trait]
impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        // the error has been added  by the validator
        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }
        Ok(user)
    }
}

async fn handler(user: RequestUser) -> Html<String> {
    Html(format!("<h1>Hello, {}!</h1>", user.username))
}
