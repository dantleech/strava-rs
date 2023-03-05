pub struct App {
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut app::App,
}
