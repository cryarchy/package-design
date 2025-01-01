pub struct User {}

impl User {
    pub fn get_confirmation_code(&self) -> String {
        "code".to_owned()
    }

    pub fn get_email_address(&self) -> &str {
        "email"
    }
}
