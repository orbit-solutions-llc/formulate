use lettre::{Message, SendmailTransport, Transport};
use rocket::figment::providers::{Env, Format, Toml};
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::serde::{json::Json, Deserialize};
use rocket::{get, launch, post, routes, Config};

const SUCCESS_MSG: &str = "Thank you! We'll get in touch as soon as we're able to.";

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    sending_email: String,
    destination_email: String,
}

/// Form submission
#[derive(FromForm, Debug)]
struct FormSubmission<'r> {
    #[field(name = uncased("full_name"))]
    #[field(name = uncased("fullname"))]
    full_name: &'r str,
    #[field(name = uncased("email"))]
    #[field(name = uncased("e-mail"))]
    email: &'r str,
    #[field(default = "You have received a new message from")]
    subject: &'r str,
    message: &'r str,
    #[field(name = uncased("site"))]
    #[field(name = uncased("website"))]
    #[field(name = uncased("location"))]
    from_site: &'r str,
}

/// Form submission from JSON
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct FormSubmissionJson {
    #[serde(alias = "fullname")]
    #[serde(alias = "fullName")]
    full_name: String,
    #[serde(alias = "e-mail")]
    email: String,
    #[serde(default = "default_subject_line")]
    subject: String,
    message: String,
    #[serde(alias = "site")]
    #[serde(alias = "website")]
    #[serde(alias = "location")]
    from_site: String,
}

fn default_subject_line() -> String {
    "You have received a new message from".to_string()
}

fn send_email(
    form_email: &str,
    form_full_name: &str,
    form_subject: &str,
    form_message: &str,
    form_site: &str,
) -> Result<(), lettre::transport::sendmail::Error> {
    let mail_subject = format!("{} {}!", &default_subject_line(), form_site);

    // Pull app config from "Rocket.toml" file with "application"
    // profile, or environment variables prefixed with "FORM_SUBMISSION_"
    let config = Config::figment()
        .select("application")
        .merge(Toml::file("Rocket.toml"))
        .merge(Env::prefixed("FORM_SUBMISSION_"))
        .extract::<AppConfig>();
    let config = match config {
        Ok(config) => config,
        Err(message) => {
            println!("Error when getting config settings: {}", message);
            panic!("OOPS.")
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

#[get("/")]
fn index() -> &'static str {
    "Nothing to see here!"
}

#[post("/", data = "<form>")]
fn submit(form: Form<FormSubmission<'_>>) -> Result<(Status, &'static str), BadRequest<String>> {
    let result = send_email(
        form.email,
        form.full_name,
        form.subject,
        form.message,
        form.from_site,
    );

    if let Err(error) = result {
        Err(BadRequest(Some(error.to_string())))
    } else {
        Ok((Status::Ok, SUCCESS_MSG))
    }
}

#[post("/", format = "json", data = "<form>", rank = 2)]
fn submit_json(
    form: Json<FormSubmissionJson>,
) -> Result<(Status, &'static str), BadRequest<String>> {
    let result = send_email(
        &form.email,
        &form.full_name,
        &form.subject,
        &form.message,
        &form.from_site,
    );

    if let Err(error) = result {
        Err(BadRequest(Some(error.to_string())))
    } else {
        Ok((Status::Ok, SUCCESS_MSG))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit, submit_json])
}
