use askama::Template;
use axum::{
    debug_handler,
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use axum_typed_multipart::TypedMultipart;

use crate::{
    models::{Album, ListPhoto, PhotosFilters},
    routes::PhotoForm,
    services::{
        albums::get_albums_with_filters,
        photos::{create_photo, get_photos_by_filters},
    },
};

pub async fn router() -> Router {
    Router::new().route("/", get(gallery_page).post(post_photo))
}

async fn gallery_page(Query(filters): Query<PhotosFilters>) -> impl IntoResponse {
    let photos = get_photos_by_filters(filters).await;
    let albums = get_albums_with_filters().await;
    let template = GalleryTemplate { photos, albums };
    HtmlTemplate(template)
}

#[debug_handler]
pub async fn post_photo(
    TypedMultipart(photo_form): TypedMultipart<PhotoForm>,
) -> impl IntoResponse {
    create_photo(photo_form, 1).await;
    Redirect::to("/page/gallery")
}

#[derive(Template)]
#[template(path = "gallery.html")]
struct GalleryTemplate {
    photos: Vec<ListPhoto>,
    albums: Vec<Album>,
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
