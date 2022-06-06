use lettre::{Message, SendmailTransport, Transport};
use rocket::figment::providers::Env;
use rocket::response::status::BadRequest;
use rocket::serde::Deserialize;
use rocket::Config;

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

/// Uses native sendmail functionality to send email containing form contents.
pub fn send_email(
    form_email: &str,
    form_full_name: &str,
    form_subject: &str,
    form_message: &str,
    form_site: &str,
) -> Result<(), BadRequest<String>> {
    let mail_subject = format!("{} {}!", &default_subject_line(), form_site);

    // Pull app config from [application] profile of "Rocket.toml"
    // file (defined by ROCKET_CONFIG environment variable)
    // or environment variables prefixed with "FORM_SUBMISSION_"
    let config = Config::figment()
        .select("application")
        .merge(Env::prefixed("FORM_SUBMISSION_"))
        .extract::<AppConfig>();
    let config = match config {
        Ok(config) => config,
        Err(message) => {
            panic!("Error when getting config settings: {}", message)
        }
    };

    let message = if form_subject != &default_subject_line() {
        format!(
            "{} has sent a message.\nSubject: {}\n\nMessage: {}",
            form_full_name, form_subject, form_message
        )
    } else {
        format!(
            "{} sent you the following message:\n\n{}",
            form_full_name, form_message
        )
    };

    let reply_to_email = form_email.parse::<lettre::message::Mailbox>();
    let reply_to_email = match reply_to_email {
        Ok(email) => email,
        Err(reason) => return Err(BadRequest(Some(format!("Problem with email address: {reason}") ))),
    };

    let email_msg = Message::builder()
        .from(
            format!("{} <{}>", form_full_name, config.sending_email)
                .parse()
                .unwrap(),
        )
        .reply_to(reply_to_email)
        .to(config.destination_email.parse().unwrap())
        .subject(mail_subject)
        .body(message)
        .unwrap();

    // println!("{:?}", email_msg);
    let mailer = SendmailTransport::new();
    match mailer.send(&email_msg) {
        Ok(message) => Ok(message),
        Err(error) => Err(BadRequest(Some(error.to_string()))),
    }
}
