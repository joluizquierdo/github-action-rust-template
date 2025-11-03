# GitHub Rust action template

<!---
This file is part of joluizquierdo/github-action-rust-template.
Copyright (C) 2025 joluizquierdo
Licensed under the GNU GPL v3. See LICENSE file in root directory.
--->

> [!WARNING]
> This is a Work In Progress. Features and documentation may change drastically
> until the first stable release.

Oxidize your GitHub Actions with Rust! This repository provides a template for
creating GitHub Actions using Rust, allowing you to leverage Rust's performance
and safety features in your CI/CD workflows.

## Usage

To use this template, simply click the "Use this template" button on the GitHub
page and create a new repository based on it. Optionally, you can clone the
repository.

Once you have your repository in place, you need to fill the configuration file
called `config.yaml` located in the root directory.

> [!NOTE]
> Refer to the [Configuration](#configuration) section for more details.

Generate the template files by running the following commands:

First, compile the scaffold project to generate the files:

```bash
cargo build --release --target-dir /tmp/rust --manifest-path scaffold/Cargo.toml
```

> [!NOTE]
> The `--target-dir /tmp/rust` flag is used to avoid polluting your project's with build artifacts. You can change the path to any temporary directory of your choice.

Second, run the compiled binary to scaffold your action:

```bash
/tmp/release/scaffold
```

Finally, commit the changes to your repository:

```bash
git add -A
git commit -m "chore: initialize action from template"
```

You are ready to go!
Start coding your GitHub Action in Rust.

## Features

### Write Github Actions outputs in rust

You just need to write to the file path specified in the `GITHUB_OUTPUT` environment
variable.

```rust
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
fn main() -> std::io::Result<()> {
    let output_path = env::var("GITHUB_OUTPUT").expect("GITHUB_OUTPUT not set");
    let mut file = OpenOptions::new()
        .append(true)
        .open(output_path)?;

    writeln!(file, "my_output_key=my_output_value")?;
    Ok(())
}
```

Finally, in the composite action (`action.yml`) you can expose the output:

```yaml
outputs:
  my_output_key: ${{ steps.rust_action_step.outputs.my_output_key }}
```

## Configuration

## Gotchas

- You can use self-hosted runners, but you need to ensure that the build runner has
  the necessary Rust toolchain installed. If you are using windows you will also
  requite to install `7z` to be able to extract the compiled binaries. On the
  other hand, if you are using linux or macOS self-hosted runners, you will
  require to install `tar` to be able to extract the compiled binaries.
- You will require curl in the self-hosted runner to download the compiled binaries from the release assets.
  Make sure that curl is installed on the runner.
- The workflow to compile the rust binaries do it natively.
  No cross-compiling is supported at the moment.
  Therefore, ensure that the runner's OS
  matches the target OS of your action.
- workflows are a little verbose
- windows runner must install `bash` script interpreter to be able to run the action.
- Is this worth it? Rust is a great language, but is it worth the complexity of
  using it for GitHub Actions? Evaluate your needs before choosing this template. Take into account that running this action will require two http calls to download the binaries from the release assets. Maybe, implementing caching in some way? If you have a better idea, please open an issue or a PR.
- I tested the action for github official runners (ubuntu-latest, windows-latest and macos-latest) and it works fine. I don't tested it on self-hosted runners extensively. Please open an issue if you find any problem.
