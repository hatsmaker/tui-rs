use crate::tui::App;

#[cfg(test)]
mod tests;
mod tui;

fn main() -> color_eyre::eyre::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}
