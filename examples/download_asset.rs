use rust_wistia::models::DownloadAssetRequest;
use rust_wistia::{Result, WistiaClient};
use std::path::PathBuf;

#[macro_use]
extern crate log;

use clap::Parser;

/// Retrieve info on a Wistia video
#[derive(Parser, Debug)]
struct Args {
    /// Hashed ID of the Wistia video to retrieve info on
    #[clap(short, long)]
    video_id: String,

    /// Type of the media asset to download
    #[clap(short = 't', long, default_value = "OriginalFile")]
    asset_type: String,

    /// Path to media file
    #[clap(short, long, parse(from_os_str), default_value = "./my-video.mp4")]
    file_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let args: Args = Args::parse();

    trace!("Creating a Wistia client");

    // Alternatively, we could use `WistiaClient::from(token)?` to
    // create the new `WistiaClient` instance.
    let client = WistiaClient::from_env()?;

    trace!(
        "Downloading the `{}` asset for the video..",
        args.asset_type
    );

    let info = DownloadAssetRequest::from(&args.video_id)
        .asset_type(&args.asset_type)
        .file_path(&args.file_path);

    let _content = client.download_asset(info).await?;

    trace!("Downloaded the asset successfully!");
    trace!("Saved the asset to file: {:?}", args.file_path);

    Ok(())
}
