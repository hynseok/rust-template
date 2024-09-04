use std::net::TcpListener;
use sqlx::PgPool;
use rust_template::startup::run;
use rust_template::config::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(
            &config.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
