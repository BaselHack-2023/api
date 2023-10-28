use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: i16,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    pub status: i16,
    pub message: String,
    pub data: T,
}
