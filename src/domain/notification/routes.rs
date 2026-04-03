use actix_web::{post, web, HttpResponse};
use crate::domain::notification::service::NotificationService;
use crate::infrastructure::response::ApiResponse;
use crate::domain::notification::dto::RegistrationEmailRequest;

#[post("/notifications/test-email")]
pub async fn test_email(
    service: web::Data<NotificationService>,
) -> HttpResponse {
    match service.send_test_email("test@paybridge.local") {
        Ok(_) => ApiResponse::<()>::success(200, "Test email sent successfully", ()),
        Err(e) => ApiResponse::<()>::error(500, &e.to_string()),
    }
}

/// POST /notifications/registration
///
/// Request body (JSON):
/// {
///   "to": "john@example.com",
///   "name": "John Doe",
///   "verification_link": "https://paybridge.com/verify?token=abc123"
/// }
#[post("/notifications/registration")]
pub async fn registration_email(
    service: web::Data<NotificationService>,
    // `web::Json<T>` automatically deserializes the JSON request body
    // into our RegistrationEmailRequest struct.
    // If the body is missing or malformed, Actix returns 400 automatically.
    body: web::Json<RegistrationEmailRequest>,
) -> HttpResponse {

    match service.send_registration_email(&body.into_inner()) {
        Ok(_)  => ApiResponse::<()>::success(200, "Registration email sent successfully", ()),
        Err(e) => ApiResponse::<()>::error(500, &e.to_string()),
    }
}
