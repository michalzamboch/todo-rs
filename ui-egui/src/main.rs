mod bootstrap;
mod constants;
mod todo;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    bootstrap::run()
}
