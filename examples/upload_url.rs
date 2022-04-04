use rust_wistia::{Result, UrlUploader};
#[macro_use]
extern crate log;

use clap::Parser;

/// Upload a local file to Wistia
///
/// You can find links to public test videos here:
///   https://gist.github.com/jsturgis/3b19447b304616f18657?permalink_comment_id=3448015#gistcomment-3448015
#[derive(Parser, Debug)]
struct Args {
    /// A publicly-accessible URL link to the media file
    #[clap(
        short,
        long,
        default_value = "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerMeltdowns.mp4"
    )]
    url: String,
    /// Name of the media file
    #[clap(short, long)]
    name: Option<String>,
    /// Description of the media file
    #[clap(short, long, default_value = "My <i>test</i><br>Message <b>here</b>.")]
    description: String,
    /// Hashed ID of the Wistia project to upload to
    #[clap(short, long)]
    project_id: Option<String>,
    /// A Wistia contact id, an integer value.
    #[clap(short, long)]
    contact_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let args: Args = Args::parse();

    trace!("Uploading link to Wistia...");

    // Alternatively, we could use `UrlUploader::with_client(url, client)` to
    // create the new `UrlUploader` instance.
    let mut uploader = UrlUploader::new(&args.url)?;

    // Normally we'll just chain together the methods like below, but here we
    // need to explicitly exclude any empty string values.
    //
    //   UrlUploader::new(&args.url)?
    //     .name(&args.name)
    //     .description(&args.description)

    if let Some(ref project_id) = args.project_id {
        uploader = uploader.project_id(project_id);
    };
    if let Some(ref name) = args.name {
        uploader = uploader.name(name);
    };
    if !args.description.is_empty() {
        uploader = uploader.description(&args.description);
    };
    if let Some(ref contact_id) = args.contact_id {
        uploader = uploader.contact_id(contact_id);
    };

    let res = uploader.send().await?;

    trace!("Response: {res:#?}");
    trace!("Video ID: {}", res.hashed_id);

    Ok(())
}
