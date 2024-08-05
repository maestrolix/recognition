use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Form, Router,
};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};

use crate::{
    middleware::authorize::{encode_jwt, verify_password},
    services::users::get_user_by_email,
};

pub async fn router() -> Router {
    Router::new().route("/", get(login_page).post(login_handler))
}

async fn login_page() -> impl IntoResponse {
    let template = AuthTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "auth.html")]
struct AuthTemplate {}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_handler(
    cookies: Cookies,
    Form(user_data): Form<AuthRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = match get_user_by_email(&user_data.email).await {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !verify_password(&user_data.password, &user.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(user.email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    cookies.add(Cookie::new("token", token));
    Ok(Redirect::to("/swagger-ui"))
}
