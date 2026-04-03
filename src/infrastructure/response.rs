use actix_web::HttpResponse;
use serde::Serialize;

// ----------------------------------------------------------
// Generic API response wrapper — equivalent to Go's APIResponse[T any]
//
// `T: Serialize` is the Rust equivalent of Go's `T any` constraint,
// but more specific: T must be serializable to JSON.
//
// `#[serde(skip_serializing_if = "Option::is_none")]` is equivalent
// to Go's `json:"data,omitempty"` — omits the field if it's None/null.
// ----------------------------------------------------------
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub status: u16,
    pub success: bool,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    // ----------------------------------------------------------
    // Success response — equivalent to Go's Success[T]()
    //
    // Example output:
    // {
    //   "status": 200,
    //   "success": true,
    //   "message": "Email sent successfully",
    //   "data": { ... }
    // }
    // ----------------------------------------------------------
    pub fn success(status: u16, message: &str, data: T) -> HttpResponse {
        let body = ApiResponse {
            status,
            success: true,
            message: message.to_string(),
            data: Some(data),
        };

        HttpResponse::build(
            actix_web::http::StatusCode::from_u16(status)
                .unwrap_or(actix_web::http::StatusCode::OK),
        )
            .json(body)
    }

    // ----------------------------------------------------------
    // Error response — equivalent to Go's Error()
    //
    // Uses ApiResponse<()> because there's no data to return.
    // `()` in Rust = "unit type" = roughly equivalent to `void` or `null`.
    //
    // Example output:
    // {
    //   "status": 500,
    //   "success": false,
    //   "message": "Failed to send email"
    // }
    // ----------------------------------------------------------
    pub fn error(status: u16, message: &str) -> HttpResponse {
        let body = ApiResponse::<()> {
            status,
            success: false,
            message: message.to_string(),
            data: None, // omitted from JSON output due to skip_serializing_if
        };

        HttpResponse::build(
            actix_web::http::StatusCode::from_u16(status)
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
        )
            .json(body)
    }
}