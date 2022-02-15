# Examples

This folder contains example scripts that can be used
to make calls to the [Wistia API](https://wistia.com/support/developers).

To start out with, you'll need to create an *access token*
as mentioned in the [Getting Started](https://wistia.com/support/developers/data-api#creating-and-managing-access-tokens)
guide.

After that, you will need to ensure that the **WISTIA_API_TOKEN**
env variable is properly set.

*On Mac/Linux*, this would be like:

```shell
❯❯ export WISTIA_API_TOKEN='MY-TOKEN'
```

*On Windows*, that would instead take the following form:

```shell
❯❯ $env:WISTIA_API_TOKEN = 'MY-TOKEN'
```

Once that is done, you should be able to use
any of the examples to make sample calls to the Wistia
API. 

## Quickstart

First, start out by cloning the GitHub project:

```shell
❯❯ git clone https://github.com/rnag/rust-wistia.git
```

When running the examples, we'll often want to see the DEBUG logs from the library under test,
`rust-wistia` in this case. Therefore, remember to ensure that the **RUST_LOG** env variable
is properly set.

For example, on *Mac/Linux*:

```shell
❯❯ export RUST_LOG='rust_wistia=TRACE'
```

On *Windows*:

```shell
❯❯ $env:RUST_LOG='rust_wistia=TRACE'
```

Next, simply just `cd` into the project folder:

```shell
❯❯ cd rust-wistia
```

From here, you can use `cargo` to build and run
any of the examples individually.

In particular, here's a simple example
of uploading a sample file to a default project on Wistia:

```shell
❯❯ cargo run --example upload_file --features upload-file
```

To pass arguments to a script, you can include them after the `--`.

For example, here's an example of passing arguments to the `upload_url` script,
which uploads a media file using a [publicly-accessible](https://gist.github.com/jsturgis/3b19447b304616f18657?permalink_comment_id=3448015#gistcomment-3448015)
URL link:

```shell
❯❯ cargo run --example upload_url -- \
     -n "My Video Name" \
     -d "Test <i>description</i>"
```
