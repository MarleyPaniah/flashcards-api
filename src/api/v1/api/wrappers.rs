use axum::extract::FromRequest;
use axum::response::{IntoResponse, Response};

use super::errors::app::AppError;

/*
   As `axum::Json` responds with plain text if the input is invalid,
   we customize the JSON extractor (`axum::Json`) to return an
   enum from AppError, making the app more cohesive.
*/
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJsonResponse<T>(pub T);

impl<T> IntoResponse for AppJsonResponse<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
