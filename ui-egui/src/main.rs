mod bootstrap;
mod constants;
mod pipeline;
mod todo_handler;

fn main() -> Result<(), eframe::Error> {
    bootstrap::run()
}
