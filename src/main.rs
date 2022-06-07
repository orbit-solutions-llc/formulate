mod mailer;
use mailer::{default_subject_line, send_email, MailConfigError};
mod strings;
use strings::{SUCCESS_MSG, WELCOME_MSG};

use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::serde::{json::Json, Deserialize};
use rocket::{get, launch, post, routes};
use validator::Validate;

/// Form submission
#[derive(Debug, FromForm, Validate)]
struct FormSubmission {
    #[field(name = uncased("full_name"))]
    #[field(name = uncased("fullname"))]
    full_name: String,
    #[validate(email(
        message = "Invalid email address provided. Please check email and try again."
    ))]
    #[field(name = uncased("email"))]
    #[field(name = uncased("e-mail"))]
    email: String,
    #[field(default = "You have received a new message from")]
    subject: String,
    message: String,
    #[field(name = uncased("site"))]
    #[field(name = uncased("website"))]
    #[field(name = uncased("location"))]
    from_site: String,
}

/// Form submission from JSON
#[derive(Debug, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
struct FormSubmissionJson {
    #[serde(alias = "fullname")]
    #[serde(alias = "fullName")]
    full_name: String,
    #[validate(email(
        message = "Invalid email address provided. Please check email and try again."
    ))]
    #[serde(alias = "e-mail")]
    email: String,
    #[serde(default = "default_subject_line")]
    subject: String,
    // #[validate(length(min = 4))]
    message: String,
    #[serde(alias = "site")]
    #[serde(alias = "website")]
    #[serde(alias = "location")]
    from_site: String,
}

#[get("/")]
fn index() -> &'static str {
    WELCOME_MSG
}

#[post("/", data = "<form>")]
fn submit(form: Form<FormSubmission>) -> Result<(Status, &'static str), BadRequest<String>> {
    let validated = form.validate();

    if let Err(error) = validated {
        Err(BadRequest(Some(error.to_string())))
    } else {
        let result = send_email(
            &form.email,
            &form.full_name,
            &form.subject,
            &form.message,
            &form.from_site,
        );

        if let Err(error) = result {
            match error {
                // We should never get here because of the earlier validation which takes place
                MailConfigError::AddressParse(err) => Err(BadRequest(Some(err.to_string()))),
                MailConfigError::AppConfig(err) => Err(BadRequest(Some(err.to_string()))),
                MailConfigError::EmailBuild(err) => Err(BadRequest(Some(err.to_string()))),
                MailConfigError::SendmailTransport(err) => Err(BadRequest(Some(err.to_string()))),
            }
        } else {
            Ok((Status::Ok, SUCCESS_MSG))
        }
    }
}

#[post("/", format = "json", data = "<form>", rank = 2)]
fn submit_json(
    form: Json<FormSubmissionJson>,
) -> Result<(Status, &'static str), BadRequest<String>> {
    let validated = form.validate();

    if let Err(error) = validated {
        Err(BadRequest(Some(error.to_string())))
    } else {
        let result = send_email(
            &form.email,
            &form.full_name,
            &form.subject,
            &form.message,
            &form.from_site,
        );

        if let Err(error) = result {
            match error {
                // We should never get here because of the earlier validation which takes place
                MailConfigError::AddressParse(err) => Err(BadRequest(Some(err.to_string()))),
                MailConfigError::AppConfig(err) => Err(BadRequest(Some(err.to_string()))),
                MailConfigError::EmailBuild(err) => Err(BadRequest(Some(err.to_string()))),
                MailConfigError::SendmailTransport(err) => Err(BadRequest(Some(err.to_string()))),
            }
        } else {
            Ok((Status::Ok, SUCCESS_MSG))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit, submit_json])
}

#[cfg(test)]
mod tests;
