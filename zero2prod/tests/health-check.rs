//! tests/health_check.rs
use zero2prod::configuration::get_configuration;
use zero2prod::configuration::Settings;
use sqlx::{PgConnection, Connection};
use std::{fmt::format, net::TcpListener};

#[tokio::test]
async fn health_check_test() {
    let url = spawn_app();
    // Reqwest client for http requests against the application
    let client = reqwest::Client::new();

    let response = client
    .get(format!("{}/health_check", url))
    .send()
    .await.expect("Failed to execute request");

    //Assertions
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    let url: String = spawn_app();
    
    let configuration: Settings = get_configuration().expect("Failed to read configuration");
    let connection_string: String = configuration.database.connection_string();
     println!("connection_string: {}", connection_string);
    
    let _connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    
    let client: reqwest::Client = reqwest::Client::new();
    let body: &str = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response: reqwest::Response = client
        .post(format!("{}/subscriptions", url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request. ");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_missing() {
    let url = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
        .post(format!("{}/subscriptions", url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not fail with Bad Request when payload was {}. ",
        error_message
    )
    }
}

// Launch our application in a background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind to port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::startup::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}