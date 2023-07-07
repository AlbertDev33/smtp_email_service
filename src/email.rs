use handlebars::Handlebars;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::{config::Config, user::User};

pub struct Email {
    user: User,
    url: String,
    from: String,
    config: Config,
}

impl Email {
    pub fn new(user: User, url: String, from: String, config: Config) -> Email {
        let from = format!("Codevo <{}>", config.smtp_from.to_owned());

        return Email {
            user,
            url,
            from,
            config,
        };
    }
}
