#[cfg(test)]
use super::rocket;
use super::strings::WELCOME_MSG;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};

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
    match Client::tracked(rocket()) {
        Ok(client) => {
            let lowercase_form_body = "fullname=Full+Name&email=test%40test.com&subject=mail+title&message=You+have+a+new+inquiry+from&site=site.com";
            let response = create_response(&client, ContentType::Form, lowercase_form_body);
            assert!(response.status() == Status::Ok);
            assert!(response.into_string() == Some(super::SUCCESS_MSG.into()));

            let malformed_email_body = "fullname=Full+Name&email=testtest.com&subject=mail+title&message=You+have+a+new+inquiry+from&site=site.com";
            let response = create_response(&client, ContentType::Form, malformed_email_body);
            assert!(response.status() == Status::BadRequest);
        }
        Err(error) => panic!("Invalid rocket instance: {error}"),
    }
}

#[test]
fn test_submit_json() {
    match Client::tracked(rocket()) {
        Ok(client) => {
            let form_body = "{\"fullname\":\"Named\",\"email\":\"publico@thebennettproject.com\",\"subject\":\"mail\",\"message\":\"You have a new inquiry from\",\"site\":\"site.com\"}";
            let response = create_response(&client, ContentType::JSON, form_body);
            assert!(response.status() == Status::Ok);
            assert!(response.into_string() == Some(super::SUCCESS_MSG.into()));

            let malformed_email_body = "{\"fullname\":\"Named\",\"email\":\"testtest.com\",\"subject\":\"mail\",\"message\":\"You have a new inquiry from\",\"site\":\"site.com\"}";
            let response = create_response(&client, ContentType::JSON, malformed_email_body);
            assert!(response.status() == Status::BadRequest);
            assert!(response.into_string().unwrap() == String::from("Missing domain or user"));
        }
        Err(error) => panic!("Invalid rocket instance: {error}"),
    }
}
