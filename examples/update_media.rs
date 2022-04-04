use rust_wistia::{Result, WistiaClient};

#[macro_use]
extern crate log;

use clap::Parser;
use rust_wistia::models::UpdateMediaRequest;
use serde_json::to_string_pretty;

/// Updates info on a Wistia video
#[derive(Parser, Debug)]
struct Args {
    /// Hashed ID of the Wistia video to update
    #[clap(short, long)]
    video_id: String,
    /// The media's new name.
    #[clap(short, long)]
    name: Option<String>,
    /// The Wistia hashed ID of an image that will replace the still that’s
    /// displayed before the player starts playing. Will return failure message
    /// unless media to update is a video, and new still is an image.
    #[clap(short, long)]
    pub still_media_id: Option<String>,
    /// A new description for this media. Accepts plain text or markdown.
    #[clap(short, long)]
    pub description: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    let args: Args = Args::parse();

    // Alternatively, we could use `WistiaClient::from(token)?` to
    // create the new `WistiaClient` instance.
    let client = WistiaClient::from_env()?;

    let req = UpdateMediaRequest {
        id: args.video_id,
        name: args.name,
        new_still_media_id: args.still_media_id,
        description: args.description,
        ..Default::default()
    };

    let res = client.update_media(req).await?;

    trace!("Response: {}", to_string_pretty(&res)?);

    Ok(())
}
