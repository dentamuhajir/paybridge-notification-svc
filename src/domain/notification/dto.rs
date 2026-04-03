/// Parameters required to send a registration success email.
/// This is passed in from the route handler (deserialized from JSON body).
#[derive(serde::Deserialize)]
pub struct RegistrationEmailRequest {
    pub to: String,
    pub name: String,
    pub verification_link: String,
}