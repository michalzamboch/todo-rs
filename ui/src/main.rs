mod bootstrap;
mod constants;
mod enums;
mod todo;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    bootstrap::run()
}
