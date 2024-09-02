use rust_template::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
