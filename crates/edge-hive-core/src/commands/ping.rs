
use clap::Args;
use anyhow::Result;

#[derive(Args, Debug)]
pub struct PingArgs {
    #[arg(short, long, default_value_t = 8000)]
    pub port: u16,
}

pub async fn run(args: PingArgs) -> Result<()> {
    let url = format!("http://localhost:{}/ping", args.port);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        println!("Server is running.");
    } else {
        anyhow::bail!("Server is not responding.");
    }

    Ok(())
}
