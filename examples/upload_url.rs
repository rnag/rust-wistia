use rust_wistia::{Result, UrlUploader};
#[macro_use]
extern crate log;

use docopt::Docopt;
use serde::Deserialize;

// Write the Docopt usage string.
const USAGE: &str = "
Usage: upload_url [options]

You can find links to public test videos here:
  https://gist.github.com/jsturgis/3b19447b304616f18657?permalink_comment_id=3448015#gistcomment-3448015

Options:
    -h, --help                 Display this message
    -u, --url=<link>           A publicly-accessible URL link to the media file
              [default: https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerMeltdowns.mp4]
    -n, --name=<media_name>    Name of the media file
    -d, --description=<desc>   Description of the media file
                                 [default: My <i>test</i><br>Message <b>here</b>.]
    -i, --project-id=<id>      Hashed ID of the Wistia project to upload to
    -c, --contact-id=<id>      A Wistia contact id, an integer value.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_url: String,
    flag_name: String,
    flag_description: String,
    flag_project_id: String,
    flag_contact_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    trace!("Uploading link to Wistia...");

    // Alternatively, we could use `UrlUploader::with_client(url, client)` to
    // create the new `UrlUploader` instance.
    let mut uploader = UrlUploader::new(&args.flag_url)?;

    // Normally we'll just chain together the methods like below, but here we
    // need to explicitly exclude any empty string values.
    //
    //   UrlUploader::new(&args.flag_url)?
    //     .name(&args.flag_name)
    //     .description(&args.flag_description)

    if !args.flag_project_id.is_empty() {
        uploader = uploader.project_id(&args.flag_project_id);
    };
    if !args.flag_name.is_empty() {
        uploader = uploader.name(&args.flag_name);
    };
    if !args.flag_description.is_empty() {
        uploader = uploader.description(&args.flag_description);
    };
    if !args.flag_contact_id.is_empty() {
        uploader = uploader.contact_id(&args.flag_contact_id);
    };

    let res = uploader.send().await?;

    trace!("Response: {res:#?}");
    trace!("Video ID: {}", res.hashed_id);

    Ok(())
}
