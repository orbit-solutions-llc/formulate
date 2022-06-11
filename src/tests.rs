#[cfg(test)]
use super::rocket;
use super::strings::WELCOME_MSG;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};

const EMAIL_VALIDATION_MSG: &str =
    "Invalid email address provided. Please check email and try again.";

mod mail_variants {
    pub const EMAIL_MISSING_PARTS: (&str, &str) = ("testtest.com", "Missing domain or user");
    pub const EMAIL_UNBALANCED: (&str, &str) =
        ("Test User <testtest.com", "Unbalanced angle bracket");
    pub const EMAIL_INVALID_USER: (&str, &str) = ("(test@test.com", "Invalid email user");
    pub const EMAIL_INVALID_DOMAIN: (&str, &str) = ("test@#test.com", "Invalid email domain");
}

fn create_response<'c, 'r>(
    client: &'c Client,
    response_type: ContentType,
    response_body: &'r str,
) -> LocalResponse<'c> {
    client
        .post(rocket::uri!(super::submit))
        .header(response_type)
        .body(response_body)
        .dispatch()
}

fn create_form_body(email: &str) -> String {
    format!("fullname=Full+Name&email={email}&subject=mail+title&message=You+have+a+new+inquiry+from&site=site.com")
}

fn test_email_validity(validation_input: (&str, &str), client: &Client) {
    let form_body = create_form_body(validation_input.0);
    let response = create_response(client, ContentType::Form, &form_body);
    assert!(response.status() == Status::BadRequest);
    // assert!(response.into_string().unwrap() == String::from(validation_input.1));
    assert!(response.into_string().unwrap() == format!("email: {EMAIL_VALIDATION_MSG}"));
}

fn create_json_body(email: &str) -> String {
    format!("{{\"fullname\":\"Named\",\"email\":\"{email}\",\"subject\":\"mail\",\"message\":\"You have a new inquiry from\",\"site\":\"site.com\"}}")
}

fn test_email_validity_json(validation_input: (&str, &str), client: &Client) {
    let form_body = create_json_body(validation_input.0);
    let response = create_response(client, ContentType::JSON, &form_body);
    assert!(response.status() == Status::BadRequest);
    // println!("{}", response.into_string().unwrap());
    // assert!(response.into_string().unwrap() == String::from(validation_input.1));
    assert!(response.into_string().unwrap() == format!("email: {EMAIL_VALIDATION_MSG}"));
}

#[test]
fn test_index() {
    match Client::tracked(rocket()) {
        Ok(client) => {
            let response = client.get(rocket::uri!(super::index)).dispatch();
            assert!(response.status() == Status::Ok);
            assert!(response.into_string() == Some(WELCOME_MSG.into()));
        }
        Err(error) => panic!("Invalid rocket instance: {error}"),
    }
}

#[test]
fn test_submit() {
    let email_valid = "test%40test.com";
    match Client::tracked(rocket()) {
        Ok(client) => {
            let form_body = create_form_body(email_valid);
            let response = create_response(&client, ContentType::Form, &form_body);
            assert!(response.status() == Status::Ok);
            assert!(response.into_string() == Some(super::SUCCESS_MSG.into()));

            test_email_validity(mail_variants::EMAIL_MISSING_PARTS, &client);
            test_email_validity(mail_variants::EMAIL_UNBALANCED, &client);
            test_email_validity(mail_variants::EMAIL_INVALID_USER, &client);
            test_email_validity(mail_variants::EMAIL_INVALID_DOMAIN, &client);
        }
        Err(error) => panic!("Invalid rocket instance: {error}"),
    }
}

#[test]
fn test_submit_json() {
    let email_valid = "test@test.com";

    match Client::tracked(rocket()) {
        Ok(client) => {
            let form_body = create_json_body(email_valid);
            let response = create_response(&client, ContentType::JSON, &form_body);
            assert!(response.status() == Status::Ok);
            assert!(response.into_string().unwrap() == super::SUCCESS_MSG);

            test_email_validity_json(mail_variants::EMAIL_MISSING_PARTS, &client);
            test_email_validity_json(mail_variants::EMAIL_UNBALANCED, &client);
            test_email_validity_json(mail_variants::EMAIL_INVALID_USER, &client);
            test_email_validity_json(mail_variants::EMAIL_INVALID_DOMAIN, &client);
        }
        Err(error) => panic!("Invalid rocket instance: {error}"),
    }
}
