// #[macro_use]
// extern crate rocket;
extern crate sendmail;
use sendmail::email;

use rocket::form::{Form, FromForm};
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

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<form>")]
fn submit(form: Form<Submission<'_>>) -> Result<(), std::io::Error> {
    println!("{:?}", form);
    let result = email::send(
        form.email,
        [RETURN_EMAIL],
        form.subject,
        &format!("From {},\n{}", form.full_name, form.message),
    );
    Ok(result?)
}

#[post("/", format = "json", data = "<form>", rank = 2)]
fn submit_json(form: Json<SubmitAsJson>) -> Result<(), std::io::Error> {
    println!("{:?}", form);
    let result = email::send(
        &form.email,
        [RETURN_EMAIL],
        &form.subject,
        &format!("From {},\n{}", form.full_name, form.message),
    );
    Ok(result?)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit, submit_json])
}
