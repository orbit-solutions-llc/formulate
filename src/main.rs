mod mailer;
use mailer::{default_subject_line, send_email};
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::serde::{json::Json, Deserialize};
use rocket::{get, launch, post, routes};

const SUCCESS_MSG: &str = "Thank you! We'll get in touch as soon as we're able to.";

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

#[get("/")]
fn index() -> &'static str {
    "Nothing to see here!"
}

#[post("/", data = "<form>")]
fn submit(form: Form<FormSubmission<'_>>) -> Result<(Status, &str), BadRequest<String>> {
    let result = send_email(
        form.email,
        form.full_name,
        form.subject,
        form.message,
        form.from_site,
    );

    if let Err(error) = result {
        Err(error)
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
        Err(error)
    } else {
        Ok((Status::Ok, SUCCESS_MSG))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit, submit_json])
}
