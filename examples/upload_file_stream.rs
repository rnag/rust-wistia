use rust_wistia::{Result, StreamUploader};

#[macro_use]
extern crate log;

use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

use clap::Parser;

/// Upload a local file stream to Wistia
// noinspection DuplicatedCode
#[derive(Parser, Debug)]
struct Args {
    /// Path to media file
    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = "./examples/assets/sample-video.mp4"
    )]
    file_path: PathBuf,
    /// Name of the media file
    #[clap(short, long, default_value_t = String::new())]
    name: String,
    /// Description of the media file
    #[clap(short, long, default_value = "My <i>test</i><br>Message <b>here</b>.")]
    description: String,
    /// Hashed ID of the Wistia project to upload to
    #[clap(short, long, default_value_t = String::new())]
    project_id: String,
    /// A Wistia contact id, an integer value.
    #[clap(short, long, default_value_t = String::new())]
    contact_id: String,
}

// noinspection DuplicatedCode
#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let args: Args = Args::parse();

    trace!("Uploading file stream to Wistia...");

    let bytes = fs::read(args.file_path)?;
    let reader = Cursor::new(bytes);

    let reader = std::io::Cursor::new("Hello world");

    // Alternatively, we could use `StreamUploader::new(path)?` to
    // create the new `StreamUploader` instance.
    let res = StreamUploader::new("test")?
        .project_id(&args.project_id)
        .name(&args.name)
        .description(&args.description)
        .contact_id(&args.contact_id)
        .send()
        .await?;

    trace!("Response: {res:#?}");
    trace!("Video ID: {}", res.hashed_id);

    Ok(())
}
