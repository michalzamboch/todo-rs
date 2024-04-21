mod bootstrap;
mod constants;
mod todo_cache;
mod todo_handler;
mod todo_pipeline;
mod todo_ui;

fn main() -> Result<(), eframe::Error> {
    bootstrap::run()
}
