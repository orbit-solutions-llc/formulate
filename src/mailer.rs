use lettre::{Message, SendmailTransport, Transport};
use rocket::{
  Config,
  figment::providers::Env,
  serde::Deserialize
};

/// Potential errors which can be created when sending an email
pub enum MailConfigError {
    AppConfig(rocket::figment::error::Error),
    AddressParse(lettre::address::AddressError),
    EmailBuild(lettre::error::Error),
    SendmailTransport(lettre::transport::sendmail::Error),
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    sending_email: String,
    destination_email: String,
}

/// Provides a default subject line for emails sent, if one is not present with form data.
pub fn default_subject_line() -> String {
    String::from("You have received a new message from")
}

/// Uses sendmail application to send email containing form contents.
pub fn send_email(
    form_email: &str,
    form_full_name: &str,
    form_subject: &str,
    form_message: &str,
    form_site: &str,
) -> Result<(), MailConfigError> {
    let mail_subject = format!("{} {}!", default_subject_line(), form_site);

    // Pulls app config from [application] profile of "Rocket.toml"
    // (defined by ROCKET_CONFIG environment variable)
    // or environment variables prefixed with "FORM_SUBMISSION_"
    let config = match Config::figment()
        .select("application")
        .merge(Env::prefixed("FORM_SUBMISSION_"))
        .extract::<AppConfig>()
    {
        Ok(config) => config,
        Err(err) => {
            return Err(MailConfigError::AppConfig(err));
        }
    };

    let message = if form_subject != default_subject_line() {
        format!(
          "{form_full_name} has sent a message.\nSubject: {form_subject}\n\nMessage: {form_message}",
        )
    } else {
        format!("{form_full_name} sent you the following message:\n\n{form_message}",)
    };

    let reply_to_email = match form_email.parse::<lettre::message::Mailbox>() {
        Ok(email) => email,
        Err(err) => return Err(MailConfigError::AddressParse(err)),
    };
    let sending_email = match format!("{} <{}>", form_full_name, config.sending_email)
        .parse::<lettre::message::Mailbox>()
    {
        Ok(email) => email,
        Err(err) => return Err(MailConfigError::AddressParse(err)),
    };
    let destination_email = match config.destination_email.parse::<lettre::message::Mailbox>() {
        Ok(email) => email,
        Err(err) => return Err(MailConfigError::AddressParse(err)),
    };

    let email_msg = match Message::builder()
        .from(sending_email)
        .reply_to(reply_to_email)
        .to(destination_email)
        .subject(mail_subject)
        .body(message)
    {
        Ok(msg) => msg,
        Err(err) => return Err(MailConfigError::EmailBuild(err)),
    };

    let mailer = SendmailTransport::new();
    match mailer.send(&email_msg) {
        Ok(message) => Ok(message),
        Err(error) => return Err(MailConfigError::SendmailTransport(error)),
    }
}
