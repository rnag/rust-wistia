use rust_wistia::{Result, WistiaClient};

#[macro_use]
extern crate log;

use clap::Parser;
use serde_json::to_string_pretty;

/// Retrieve info on a Wistia video
#[derive(Parser, Debug)]
struct Args {
    /// Hashed ID of the Wistia video to retrieve info on
    #[clap(short, long)]
    video_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    let args: Args = Args::parse();

    // Alternatively, we could use `WistiaClient::from(token)?` to
    // create the new `WistiaClient` instance.
    let client = WistiaClient::from_env()?;

    let video_id = &args.video_id;

    let res = client.get_media(video_id).await?;

    trace!("Response: {}", to_string_pretty(&res)?);

    Ok(())
}
