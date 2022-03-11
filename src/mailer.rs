use lettre::{Message, SendmailTransport, Transport};
use rocket::figment::providers::Env;
use rocket::serde::Deserialize;
use rocket::Config;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    sending_email: String,
    destination_email: String,
}

/// Provides a default subject line for emails sent in absence of one being submitted by the web form.
pub fn default_subject_line() -> String {
    "You have received a new message from".to_string()
}

/// Uses native sendmail functionality to send email containing form contents.
pub fn send_email(
    form_email: &str,
    form_full_name: &str,
    form_subject: &str,
    form_message: &str,
    form_site: &str,
) -> Result<(), lettre::transport::sendmail::Error> {
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

    let email = Message::builder()
        .from(
            format!("{} <{}>", form_full_name, config.sending_email)
                .parse()
                .unwrap(),
        )
        .reply_to(form_email.parse().unwrap())
        .to(config.destination_email.parse().unwrap())
        .subject(mail_subject)
        .body(message)
        .unwrap();

    // println!("{:?}", email);
    let mailer = SendmailTransport::new();
    mailer.send(&email)
}
