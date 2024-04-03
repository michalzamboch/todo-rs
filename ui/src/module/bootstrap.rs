use crate::include::*;
use slint::*;

pub fn show() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.run()
}
