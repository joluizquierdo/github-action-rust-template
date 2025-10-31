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

```bash
cd rust-action
```

```bash
cargo run --example generate
```

Once the files are generated you can remove the `templates` folder,
the `config.yaml` the `rust-action/examples`
and the `rust-action/Cargo.lock` files:

```bash
# I'm supposing you're in the `rust-action` directory
rm -rf ../templates ../config.yaml examples Cargo.lock
```

Finally, commit the changes to your repository:

```bash
git add -A
git commit -m "chore: initialize action from template"
```

You are ready to go!
Start coding your GitHub Action in Rust.

## Features

## Configuration
