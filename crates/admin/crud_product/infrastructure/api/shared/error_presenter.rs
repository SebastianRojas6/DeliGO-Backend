use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::crud_product::application::interface::ApiError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPresenter {
    pub code: u16,
    pub error: String,
    pub message: String,
}

#[derive(Error, Debug, Display)]
#[display("{:?}", error)]
pub struct ErrorResponse {
    status_code: StatusCode,
    error: String,
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorPresenter {
            code: status_code.as_u16(),
            message: status_code.to_string(),
            error: self.error.clone(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl ErrorResponse {
    pub fn map_io_error(e: ApiError) -> ErrorResponse {
        match e.get_error_code() {
            400 => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                error: e.get_error_message(),
            },
            401 => ErrorResponse {
                status_code: StatusCode::UNAUTHORIZED,
                error: e.get_error_message(),
            },
            403 => ErrorResponse {
                status_code: StatusCode::FORBIDDEN,
                error: e.get_error_message(),
            },
            _ => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: String::from("Error: an unknown error occured"),
            },
        }
    }
}