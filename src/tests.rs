#[cfg(test)]
use super::rocket;
use super::strings::{EMAIL_VALIDATION_MSG, WELCOME_MSG};
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};

mod mail_variants {
    pub const email_missing_parts: (&str, &str) = ("testtest.com", "Missing domain or user");
    pub const email_unbalanced: (&str, &str) =
        ("Test User <testtest.com", "Unbalanced angle bracket");
    pub const email_invalid_user: (&str, &str) =
        ("(publico@thebennettproject.com", "Invalid email user");
    pub const email_invalid_domain: (&str, &str) =
        ("publico@#thebennettproject.com", "Invalid email domain");
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

            test_email_validity(mail_variants::email_missing_parts, &client);
            test_email_validity(mail_variants::email_unbalanced, &client);
            test_email_validity(mail_variants::email_invalid_user, &client);
            test_email_validity(mail_variants::email_invalid_domain, &client);
        }
        Err(error) => panic!("Invalid rocket instance: {error}"),
    }
}

#[test]
fn test_submit_json() {
    let email_valid = "publico@thebennettproject.com";

    match Client::tracked(rocket()) {
        Ok(client) => {
            let form_body = create_json_body(email_valid);
            let response = create_response(&client, ContentType::JSON, &form_body);
            assert!(response.status() == Status::Ok);
            assert!(response.into_string().unwrap() == super::SUCCESS_MSG);

            test_email_validity_json(mail_variants::email_missing_parts, &client);
            test_email_validity_json(mail_variants::email_unbalanced, &client);
            test_email_validity_json(mail_variants::email_invalid_user, &client);
            test_email_validity_json(mail_variants::email_invalid_domain, &client);
        }
        Err(error) => panic!("Invalid rocket instance: {error}"),
    }
}
