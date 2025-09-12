mod app;
use app::run;

fn main() -> anyhow::Result<()> {

    env_logger::init();
    run()?;
    Ok(())
}
