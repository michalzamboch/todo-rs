mod bootstrap;
mod constants;
mod todo;
mod enums;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    bootstrap::run()
}
