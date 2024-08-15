use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::{
    models::{ListPhoto, SearchQuery},
    services::photos::{get_photos_by_filters, search_by_text_service},
};

pub async fn router() -> Router {
    Router::new().route("/", get(gallery_page))
}

async fn gallery_page(Query(searh_photo): Query<SearchQuery>) -> impl IntoResponse {
    let mut photos = vec![];
    if let Some(text) = searh_photo.text {
        photos.push(search_by_text_service(text, 1).await);
    } else {
        photos = get_photos_by_filters(1).await;
    }
    let template = GalleryTemplate { photos };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "gallery.html")]
struct GalleryTemplate {
    photos: Vec<ListPhoto>,
}

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
