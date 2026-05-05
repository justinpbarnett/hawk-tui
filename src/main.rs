use clap::Parser;
use hawk_tui::{
    config::{Cli, Options},
    engine::ReviewEngine,
};
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let no_tui = cli.no_tui;
    let opts = Options::from_cli_at(cli, std::env::current_dir()?);
    let engine = ReviewEngine::open(opts);
    if no_tui {
        for r in engine.document.rows() {
            println!("{r:?}");
        }
        Ok(())
    } else {
        hawk_tui::tui::run(engine)
    }
}
