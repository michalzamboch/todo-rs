mod bootstrap;
mod constants;
mod todo;

fn main() -> Result<(), eframe::Error> {
    bootstrap::run()
}
