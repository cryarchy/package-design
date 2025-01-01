mod confirmation_mail_factory;
mod confirmation_mail_mailer;
mod message;
mod user;

pub use confirmation_mail_factory::{ConfirmationMailFactory, TemplatingEngine, Translator};
pub use confirmation_mail_mailer::{ConfirmationMailMailer, Mailer};
