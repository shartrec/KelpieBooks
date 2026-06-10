/*
 * Copyright (c) 2026.
 *
 * This file is part of KelpieBooks. For terms of use, please see the file
 * called LICENSE at the top level of the KelpieBooks source tree
 *  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
 */

use std::env;
use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport,
    AsyncTransport,
    Message,
    Tokio1Executor,
};

use crate::config::load_config;
use crate::util::locale_context::LocaleContext;
use fluent::fluent_args;

pub async fn send_reset_email(to_email: &str, token_id: i32, raw_secret: &str, locale: &str) -> Result<(), String> {
    let config = load_config();
    let i18n = LocaleContext::new(locale);

    // 1. Construct the single-use verification hyperlink
    let reset_link = format!(
        "{}/reset-password?id={}&token={}",
        config.app.base_url, token_id, raw_secret
    );

    // 2. Build the email headers and body segments safely
    let email = Message::builder()
        .from(config.smtp.from.parse().map_err(|_| "Invalid sender syntax")?)
        .to(to_email.parse().map_err(|_| "Invalid recipient syntax")?)
        .subject(i18n.t("email-reset-subject"))
        .multipart(
            lettre::message::MultiPart::alternative()
                .singlepart(
                    lettre::message::SinglePart::plain(
                        i18n.t_args("email-reset-body-plain", &fluent_args!["reset_link" => reset_link.clone()])
                    )
                )
                .singlepart(
                    lettre::message::SinglePart::html(
                        i18n.t_args("email-reset-body-html", &fluent_args!["reset_link" => reset_link])
                    )
                )
        )
        .map_err(|e| e.to_string())?;

    // 3. Instantiate the secure asynchronous SMTP transport engine
    let mut mailer_builder = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp.server)
        .map_err(|e| e.to_string())?
        .port(config.smtp.port);

    // Inject credentials if the target agent requires user authentication
    let username = env::var("SMTP_USERNAME").ok();
    let password = env::var("SMTP_PASSWORD").ok();
    if let (Some(user), Some(pass)) = (username, password) {
        mailer_builder = mailer_builder.credentials(Credentials::new(user, pass));
    }

    let mailer = mailer_builder.build();

    // 4. Fire the transmission asynchronously over the network link
    mailer.send(email).await.map_err(|e| e.to_string())?;

    Ok(())
}
