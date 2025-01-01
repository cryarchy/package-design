use crate::confirmation_mail_factory::{ConfirmationMailFactory, TemplatingEngine, Translator};
use crate::message::Message;
use crate::user::User;

pub trait Mailer {
    fn send(&self, message: Message);
}

pub struct ConfirmationMailMailer<M, TE, T>
where
    M: Mailer,
    TE: TemplatingEngine,
    T: Translator,
{
    confirmation_mail_factory: ConfirmationMailFactory<TE, T>,
    mailer: M,
}

impl<TE, T, M> ConfirmationMailMailer<M, TE, T>
where
    TE: TemplatingEngine,
    T: Translator,
    M: Mailer,
{
    pub fn new(mailer: M, confirmation_mail_factory: ConfirmationMailFactory<TE, T>) -> Self {
        Self {
            confirmation_mail_factory,
            mailer,
        }
    }

    pub fn send_to(&self, user: User) {
        let message = self.create_message_for(user);
        self.send_message(message);
    }

    fn create_message_for(&self, user: User) -> Message {
        let mut message = self.confirmation_mail_factory.create_message_for(&user);
        message.set_to(user.get_email_address().to_owned());
        message
    }

    fn send_message(&self, message: Message) {
        self.mailer.send(message);
    }
}
