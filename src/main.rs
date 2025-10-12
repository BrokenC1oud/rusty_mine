use clap::Parser;
use eyre::Result;
use tracing::Level;
use tracing_subscriber::EnvFilter;
use rusty_mine::config::load_config;
use rusty_mine::server::Server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 0)]
    verbose: u8,
    #[arg(default_value = "server.properties")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    enable_logging(args.verbose);

    let config = load_config(args.config).await?;

    let server = Server::new(config).await?;

    server.run().await?;

    Ok(())
}

fn enable_logging(verbose: u8) {
    let level = match verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        _ => Level::TRACE,
    };

    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive(level.into())
        )
        .init();
}
