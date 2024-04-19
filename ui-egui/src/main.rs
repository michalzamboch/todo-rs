mod bootstrap;
mod constants;

fn main() -> Result<(), eframe::Error> {
    bootstrap::run()
}
