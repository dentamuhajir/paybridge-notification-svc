use actix_web::{post, web, HttpResponse};
use crate::domain::notification::service::NotificationService;
use crate::infrastructure::response::ApiResponse;

#[post("/notifications/test-email")]
pub async fn test_email(
    service: web::Data<NotificationService>,
) -> HttpResponse {
    match service.send_test_email("test@paybridge.local") {
        Ok(_) => ApiResponse::<()>::success(200, "Test email sent successfully", ()),
        Err(e) => ApiResponse::<()>::error(500, &e.to_string()),
    }
}
