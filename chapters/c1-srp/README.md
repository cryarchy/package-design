# Single Responsibility Principle (SRP)

```mermaid
---
title: Initial Implementation
---
classDiagram
    ConfirmationMailMailer ..> Mailer: uses
    ConfirmationMailFactory ..> TempalateEngine: uses
    ConfirmationMailFactory ..> Translator: uses
    ConfirmationMailMailer ..> ConfirmationMailFactory: uses
```

ConfirmationMailMailer has been reimplemented as:

```rust
pub struct ConfirmationMailMailer<TE, T, M>
where
    TE: TemplatingEngine,
    T: Translator,
    M: Mailer,
{
    templating_engine: TE,
    translator: T,
    mailer: M,
}

impl<TE, T, M> ConfirmationMailMailer<TE, T, M>
where
    TE: TemplatingEngine,
    T: Translator,
    M: Mailer,
{
    pub fn new(templating_engine: TE, translator: T, mailer: M) -> Self { ... }

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
```

and now delegates the responsibility of creating the confirmation mail to `ConfirmationMailFactory` which is implemented as:

```rust
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
    pub fn new(templating_engine: TE, translator: T) -> Self { ... }

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
```
