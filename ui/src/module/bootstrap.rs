use crate::include::*;
use slint::*;

pub fn run() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.run()
}
