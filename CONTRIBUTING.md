# Contribution Guidelines

You want to help contribute? Awesome! Thanks for taking the time to look at the
guidelines for this repo. Here's what you need to know!

## License

**rust-wistia** is proudly licenced under the MIT license, and so are all
contributions. Please see the [`LICENSE`] file in this directory for more details.

[`LICENSE`]: https://github.com/rnag/rust-wistia/blob/main/LICENSE

## Pull Requests

To make changes to **rust-wistia**, please send in pull requests on GitHub to
the `main` branch. I'll review them and either merge or request changes. GitHub Actions
tests everything as well, so you may get feedback from it too.

If you make additions or other changes to a pull request, feel free to either amend
previous commits or only add new ones, however you prefer. I may ask you to squash
your commits before merging, depending.

## Issue Tracker

You can find the issue tracker [on
GitHub](https://github.com/rnag/rust-wistia/issues). If you've found a
problem with **rust-wistia**, please open an issue there.

<!--
We use the following labels:

* `enhancement`: This is for any request for new sections or functionality.
* `bug`: This is for anything that's in `rust-wistia`, but incorrect or not working.
* `discussion`: A discussion about improving something in `rust-wistia`; this may
* lead to new enhancement or bug issues.
-->

## Development workflow

Check out the `README.md` under the [examples/ folder](examples/README.md) for details
on the development process.

## Examples
Do you want to help show off some ways for how the library works? Feel free to
work on an example and open up a PR!

[install Rust]: http://rust-lang.org/install.html

To run the tests:

```bash
$ cargo test
```

## Code

<!-- If you want to contribute code but don't know how everything works check out the
[design docs](./docs/design.md) for the library. -->

If you want to contribute code but don't know how everything works check out the
source code in the library; specifically check out the `api.rs` which contains the client
implementation logic. If you want to know what
endpoints still need to be implemented see the [endpoints](./docs/endpoints.md)
docs.

Due to the use of certain features rust-wistia requires rustc version 1.18 or higher.

To run tests, you will need an access token from Wistia. You can follow the
official [Wistia documentation][wistia-access-token-docs] to get a personal access
token for testing.

Once you have an access token, set the `WISTIA_API_TOKEN` env variable and
work on creating an example under the `examples` folder.

Finally, run `cargo test` to make sure all the tests pass.

[wistia-access-token-docs]: https://wistia.com/support/developers/data-api#getting-started

## Documentation
As with any project, documentation is a key part that can make or break usage of
a library. Why use the best library ever if it has no documentation?

With that in mind, `rust-wistia` strives to document every aspect of it in order to make it
easier for the user. In particular, it would be a great help to write out clear documentation
and - for example - annotate the parameters and their sample usage for each newly implemented
function or method.
