use rust_wistia::{FileUploader, Result, UploadClient};

#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};

use clap::Parser;

/// Upload a local thumbnail image to Wistia
#[derive(Parser, Debug)]
struct Args {
    /// Path to image file
    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = "./examples/assets/sample-thumbnail.png"
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

//noinspection DuplicatedCode
#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let args: Args = Args::parse();

    trace!("Uploading thumbnail to Wistia...");

    let client = UploadClient::from_env()?;
    let file_path = Path::new(&args.file_path);

    // Alternatively, we could use `FileUploader::new(path)?` to
    // create the new `FileUploader` instance.
    let res = FileUploader::with_client(file_path, client)
        .project_id(&args.project_id)
        .name(&args.name)
        .description(&args.description)
        .contact_id(&args.contact_id)
        .send()
        .await?;

    trace!("Response: {res:#?}");
    trace!("Media ID: {}", res.hashed_id);

    Ok(())
}
