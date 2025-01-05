use std::io::Result;

use wi_tui::app::App;

fn main() -> Result<()> {
    let mut app = App::new();
    app.run()?;

    Ok(())
}
