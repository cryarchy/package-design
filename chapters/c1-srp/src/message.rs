pub struct Message {}

impl Message {
    pub fn new(_subject: String, _body: String) -> Self {
        Self {}
    }

    pub fn set_to(&mut self, _email_address: String) {}
}
