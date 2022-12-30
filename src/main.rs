use clap::Parser;
use flexi_logger::Logger;
use wiser::Hub;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The secret for the hub to be accessed.
    #[arg(long, env)]
    secret: String,

    /// The IP or hostname of the hub.
    #[arg(long, env)]
    hub: String,
}

#[tokio::main]
async fn main() {
    if let Err(e) = Logger::try_with_env_or_str("wiser=info,warn").and_then(|logger| logger.start())
    {
        eprintln!("Warning, failed to start logging: {}", e);
    }

    let cli = Cli::parse();

    let hub = Hub::new(&cli.hub, &cli.secret);
    hub.domain().await.unwrap();
}
