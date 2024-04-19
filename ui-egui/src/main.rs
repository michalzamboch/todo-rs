mod bootstrap;
mod constants;
mod pipeline;

fn main() -> Result<(), eframe::Error> {
    bootstrap::run()
}
