use rust_wistia::{Result, WistiaClient};

#[macro_use]
extern crate log;

use clap::Parser;

/// Retrieve the media [Asset URL] on a Wistia video
///
/// [Asset URL]: https://wistia.com/support/developers/asset-urls
#[derive(Parser, Debug)]
struct Args {
    /// Hashed ID of the Wistia video to retrieve info on
    #[clap(short, long)]
    video_id: String,

    /// Type of the media asset to retrieve. Defaults to the original asset that was uploaded.
    #[clap(short = 't', long, default_value = "OriginalFile")]
    asset_type: String,

    /// Retrieve only the HTTP asset url, rather than the SSL (HTTPS) variant
    #[clap(short, long)]
    no_ssl: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    let args: Args = Args::parse();
    let asset_type = args.asset_type.as_str();

    // Alternatively, we could use `WistiaClient::from(token)?` to
    // create the new `WistiaClient` instance.
    let client = WistiaClient::from_env()?;

    let video_id = &args.video_id;

    let media = client.get_media(video_id).await?;

    let url = if args.no_ssl {
        media.asset_url_insecure(asset_type)?.to_owned()
    } else {
        // as a shorthand, we could just call `media.source_url()?` in this case
        media.asset_url(asset_type)?
    };

    trace!("Asset URL: {url}");

    Ok(())
}
