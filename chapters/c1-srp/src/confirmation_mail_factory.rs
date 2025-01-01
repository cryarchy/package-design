use std::collections::HashMap;

use crate::{message::Message, user::User};

pub trait TemplatingEngine {
    fn render(&self, _template_name: &str, _context: HashMap<String, String>) -> String {
        "rendered template".to_owned()
    }
}

pub trait Translator {
    fn translate(&self, text: &str) -> String;
}

pub struct ConfirmationMailFactory<TE, T>
where
    TE: TemplatingEngine,
    T: Translator,
{
    templating_engine: TE,
    translator: T,
}

impl<TE, T> ConfirmationMailFactory<TE, T>
where
    TE: TemplatingEngine,
    T: Translator,
{
    pub fn new(templating_engine: TE, translator: T) -> Self {
        Self {
            templating_engine,
            translator,
        }
    }

    pub fn create_message_for(&self, user: &User) -> Message {
        let subject = self.translator.translate("Confirm your mail address");
        let template_ctx =
            HashMap::from([("confirmation_code".to_owned(), user.get_confirmation_code())]);
        let body = self
            .templating_engine
            .render("confirmation_mail.html.tpl", template_ctx);
        let mut message = Message::new(subject, body);
        message.set_to(user.get_email_address().to_owned());
        message
    }
}
