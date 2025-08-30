use axum::http::StatusCode;
use serde::Serialize;

pub struct ErrorSpecs {
    pub offset: u32,
    pub status_code: StatusCode,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub error_type: String,
    pub error_code: u32,
    pub status_code: u16,
    pub message: String,
}

pub trait ReturnableError: std::fmt::Debug + Send + Sync + 'static {
    fn error_type(&self) -> &'static str;
    fn base_code(&self) -> u32;

    /**
     Method to define the specs of the error enum's possible values.
    */
    fn specs(&self) -> ErrorSpecs;

    fn offset(&self) -> u32 {
        self.specs().offset
    }

    fn status_code(&self) -> StatusCode {
        self.specs().status_code
    }

    fn message(&self) -> String {
        self.specs().message
    }

    fn full_error_code(&self) -> u32 {
        self.base_code() + self.offset()
    }

    fn body(&self) -> ErrorBody {
        ErrorBody {
            error_type: self.error_type().to_string(),
            error_code: self.full_error_code(),
            status_code: self.status_code().as_u16(),
            message: self.message(),
        }
    }
}

/**
  Boxed ReturnableError. To use when match branches have different concrete enum types.
*/
pub type DynReturnableError = Box<dyn ReturnableError>;

pub trait IntoBoxedError {
    fn boxed(self) -> Box<dyn ReturnableError>;
}

impl<T: ReturnableError> IntoBoxedError for T {
    fn boxed(self) -> Box<dyn ReturnableError> {
        Box::new(self)
    }
}
