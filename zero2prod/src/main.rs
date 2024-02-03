//! src/main.rs
use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuratioin = get_configuration().expect("failed to read configuration. ");
    let address = format!("127.0.0.1:{}", configuratioin.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await

}


#[cfg(test)]
mod tests {
    // use zero2prod::routes::health_check;

    // #[tokio::test]
    // async fn health_check_suceeds() {
    //     let response = health_check().await;
    //     assert!(response.status().is_success());
    // }
}