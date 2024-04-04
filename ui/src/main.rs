mod include;
mod module;

fn main() -> Result<(), slint::PlatformError> {
    module::bootstrap::run()
}
