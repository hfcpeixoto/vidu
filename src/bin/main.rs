use loco_rs::cli;
use migration::Migrator;
use vidu::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App, Migrator>().await
}
