// #[macro_use]
// extern crate rocket;
extern crate sendmail;
use sendmail::email;

use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::serde::{json::Json, Deserialize};
use rocket::{get, launch, post, routes};

const RETURN_EMAIL: &'static str = "test@test.com";

#[derive(FromForm, Debug)]
struct Submission<'r> {
    #[field(name = uncased("full_name"))]
    #[field(name = uncased("fullname"))]
    full_name: &'r str,
    #[field(name = uncased("email"))]
    #[field(name = uncased("e-mail"))]
    email: &'r str,
    subject: &'r str,
    message: &'r str,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SubmitAsJson {
    #[serde(alias = "fullname")]
    #[serde(alias = "fullName")]
    full_name: String,
    #[serde(alias = "e-mail")]
    email: String,
    subject: String,
    message: String,
}

fn send_email(
    form_email: &str,
    form_full_name: &str,
    form_subject: &str,
    form_message: &str,
) -> Result<(), std::io::Error> {
    let mail_subject = format!("You have a new inquiry from {}!", DEST_DOMAIN);

    let message = format!(
        "{} has sent a message.\n Subject: {}\n Message: {}",
        form_full_name, form_subject, form_message
    );

    email::send(form_email, [RETURN_EMAIL], &mail_subject, &message)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<form>")]
fn submit(form: Form<Submission<'_>>) -> Result<(Status, &'static str), BadRequest<String>> {
    let result = send_email(form.email, form.full_name, form.subject, form.message);

    if let Err(error) = result {
        return Err(BadRequest(Some(error.to_string())))
    } else {
        return Ok((
            Status::Ok,
            "Thank you! We'll get in touch as soon as we have a response.",
        ))
    }
}

#[post("/", format = "json", data = "<form>", rank = 2)]
fn submit_json(form: Json<SubmitAsJson>) -> Result<(Status, &'static str), BadRequest<String>> {
    let result = send_email(&form.email, &form.full_name, &form.subject, &form.message);

    if let Err(error) = result {
        return Err(BadRequest(Some(error.to_string())))
    } else {
        return Ok((
            Status::Ok,
            "Thank you! We'll get in touch as soon as we have a response.",
        ))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit, submit_json])
}
