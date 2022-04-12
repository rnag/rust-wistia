use rust_wistia::models::DownloadAssetRequest;
use rust_wistia::{Result, WistiaClient};

#[macro_use]
extern crate log;

use std::path::PathBuf;

use atty::is;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

/// Download a source asset for a Wistia video, and save it to a local file path.
#[derive(Parser, Debug)]
struct Args {
    /// Hashed ID of the Wistia video to retrieve info on
    #[clap(short, long)]
    video_id: String,

    /// Type of the media asset to download. Defaults to the original asset that was uploaded.
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

    // (Optional) retrieve the media info

    let pb = Progress::start(format_args!(
        "{}: retrieving media info for the video ...",
        &args.video_id
    ));

    let media = client.get_media(&args.video_id).await?;

    pb.finish("downloaded the media info");

    let pb = Progress::start(format_args!(
        "downloading the `{}` asset for the video ...",
        &args.asset_type
    ));

    // Normally, we'd just use `DownloadAssetRequest::from(&args.video_id)`, but
    // here we've just retrieved the media info above.
    let info = DownloadAssetRequest::from(media)
        .asset_type(&args.asset_type)
        .file_path(&args.file_path);

    let _content = client.download_asset(info).await?;

    pb.finish(format_args!(
        "downloaded the asset, and saved it to a file: {:?}",
        args.file_path,
    ));

    Ok(())
}

pub struct Progress {
    bar: Option<ProgressBar>,
}

impl Progress {
    pub fn start(msg: impl ToString) -> Progress {
        let bar = if is(atty::Stream::Stdout) {
            Some(show_progress(msg))
        } else {
            println!("▹▹▹▹▹ {}", msg.to_string());
            None
        };
        Progress { bar }
    }

    pub fn finish(&self, msg: impl ToString) {
        if let Some(bar) = &self.bar {
            bar.finish_with_message(msg.to_string());
        } else {
            println!("▪▪▪▪▪ {}", msg.to_string());
        }
    }
}

fn show_progress(msg: impl ToString) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message(msg.to_string());
    pb
}
