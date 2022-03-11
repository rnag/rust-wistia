use rust_wistia::{Result, WistiaClient};

use docopt::Docopt;
use serde::Deserialize;
use serde_json::to_string_pretty;

// Write the Docopt usage string.
const USAGE: &str = "
Usage: get_media [options]

Options:
    -h, --help                 Display this message
    -i, --video-id=<id>        Hashed ID of the Wistia video to retrieve info on
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_video_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // Alternatively, we could use `WistiaClient::from(token)?` to
    // create the new `WistiaClient` instance.
    let client = WistiaClient::from_env()?;

    let video_id = &args.flag_video_id;

    let res = client.get_media(video_id).await?;

    println!("Response: {}", to_string_pretty(&res)?);

    Ok(())
}
