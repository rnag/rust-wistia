use rust_wistia::{FileUploader, Result, UploadClient};

use std::path::Path;

use docopt::Docopt;
use serde::Deserialize;

// Write the Docopt usage string.
const USAGE: &str = "
Usage: upload_file [options]

Options:
    -h, --help                 Display this message
    -p, --path=<file>          Path to media file
                                 [default: ./examples/assets/sample-video.mp4]
    -n, --name=<media_name>    Name of the media file
    -d, --description=<desc>   Description of the media file
                                 [default: My <i>test</i><br>Message <b>here</b>.]
    -i, --project-id=<id>      Hashed ID of the Wistia project to upload to
    -c, --contact-id=<id>      A Wistia contact id, an integer value.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_path: String,
    flag_name: String,
    flag_description: String,
    flag_project_id: String,
    flag_contact_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let client = UploadClient::from_env()?;
    let file_path = Path::new(&args.flag_path);

    // Alternatively, we could use `FileUploader::new(path)?` to
    // create the new `FileUploader` instance.
    let res = FileUploader::with_client(file_path, client)
        .project_id(&args.flag_project_id)
        .name(&args.flag_name)
        .description(&args.flag_description)
        .contact_id(&args.flag_contact_id)
        .send()
        .await?;

    println!("Response: {res:#?}");
    println!("Video ID: {}", res.hashed_id);

    Ok(())
}
