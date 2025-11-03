# GitHub Action Rust Template

**Oxidize your GitHub Actions with Rust! 🦀**  
A template repository for building native GitHub Actions using Rust.

## Overview

This repository lets you create GitHub Actions using **Rust**, taking advantage of its speed and reliability.  
Supports official GitHub runners (`ubuntu-latest`, `windows-latest`, `macos-latest`) and can be tailored for self-hosted environments.

The approach uses a **composite GitHub workflow** to download and execute
pre-compiled Rust binaries, ensuring compatibility across platforms without
requiring Rust installation on the caller github runner or dealing with docker images.

## Getting Started

### 1. Create Your Action Repository

- Click **"Use this template"** on GitHub  
  or clone with:

  ```bash
  git clone https://github.com/joluizquierdo/github-action-rust-template.git
  ```

### 2. Configure

Edit the `config.yaml` at the root of your new repository.  
See [Configuration](#configuration) for details.

### 3. Generate Template Files

Compile the scaffold tool and generate your action files:

```bash
cargo build --release --target-dir /tmp/rust --manifest-path scaffold/Cargo.toml
/tmp/rust/release/scaffold
```

_You may change `/tmp/rust` to any temporary directory of your choice._

### 4. Commit and Push

```bash
git add -A
git commit -m "chore: initialize action from template"
git push origin main
```

### 5. Tag Releases

To create a release, tag and push:

```bash
git tag v0.1.0
git push origin v0.1.0
```

Here is the **final Features section** for your README, precisely describing how the template works, including how inputs are passed via CLI arguments and how outputs, logging, secrets, and platform support are handled:

## Features

- **Define Action Inputs (YAML + Rust):**
  - Inputs are declared in your `action.yml` (or the template `action.yml.j2`) under the `inputs:` section, specifying `description`, `required`, and `default` values.
  - Inputs are _not_ available as environment variables; instead, each input is forwarded as a CLI argument when invoking your Rust binary.
  - You must declare matching fields in your Rust code’s `Args` struct, using the `clap` crate to parse arguments (e.g., `--name`, `--surname`).

    **Example (in action.yml):**

    ```yaml
    - name: "Run the rust binary"
      shell: bash
      id: rust_binary
      run: |
        "${BINARY_PATH}" --name "${{ inputs.name }}" --surname "${{ inputs.surname }}"
    ```

    **Example (in Rust):**

    ```rust
    #[derive(Parser, Debug)]
    struct Args {
        #[clap(long)]
        name: String,
        #[clap(long)]
        surname: String,
    }
    ```

- **Produce GitHub Action Outputs (Rust):**
  - Action outputs are created by writing `key=value` lines to the file path specified by the `GITHUB_OUTPUT` environment variable.
  - The template provides helper functions (`set_output`) to simplify writing outputs in Rust.
  - Outputs are exposed to subsequent workflow steps via the `outputs` section in `action.yml`.

    **Example (in Rust):**

    ```rust
    github_context.set_output("greeting-message", &full_greeting);
    ```

    **Example (in action.yml):**

    ```yaml
    outputs:
      greeting-message:
        description: "The complete greeting message."
        value: "${{ steps.rust_binary.outputs.greeting-message }}"
    ```

- **Logging and Workflow Notices (Rust):**
  - Use logging helpers to emit workflow notices:

    ```rust
    log_notice("Your message here");
    ```

  - Displays messages in the GitHub Action logs.

- **Masking Secrets (Rust):**
  - Hide sensitive values from logs using mask helpers:

    ```rust
    set_secret(&token);
    ```

  - Uses the GitHub Actions `::add-mask::` command.

- **Robust Error Handling:**
  - Rust code uses `Result<T>`, `?`, and `.expect()` for safe error handling of missing environment variables and IO errors.

- **Platform-Agnostic Composite Workflow:**
  - The template supports Linux, macOS, and Windows runners.
  - Handles platform-specific environment setup and binary extraction, enabling cross-platform workflows.

- **Download and Invoke Rust Binaries:**
  - Composite workflow steps handle downloading pre-compiled binaries from GitHub releases, extracting artifacts, setting execution permissions, and running the binary with provided inputs.

## Configuration

Customize your action via the `config.yaml`. The following table explains each configurable setting:

| Field         | Description                | Example Value                            |
| ------------- | -------------------------- | ---------------------------------------- |
| action_name   | Name of your GitHub Action | "Your action name here"                  |
| description   | Brief description          | "A brief description of your action."    |
| author        | Author name                | "Your Name"                              |
| rust.edition  | Rust edition               | "2024"                                   |
| rust.version  | Rust project version       | "0.1.0"                                  |
| rust.name     | Rust project name          | "rust-action"                            |
| rust.target   | Target platform            | "x86_64-unknown-linux-gnu"               |
| github.runner | GitHub runner OS or label  | "ubuntu-latest" or "[self-hosted, XXXX]" |

> [!NOTE]
> When setting the `github.runner` field for self-hosted runner don't forget to include the '[]' brackets in the
> string value.

## Gotchas & Requirements

- **Self-hosted runners:**
  - _Rust toolchain must be installed_.
  - Windows: install `7z` and `bash`.
  - Linux/macOS: install `tar`.
  - All runners: ensure `curl` is installed.
- **No cross-compilation:**  
  Runner OS must match the action’s target OS.
- **Action setup requires two HTTP calls** to download binaries from release assets. Consider caching if needed.
- **Limited self-hosted runner testing:**  
  Template tested on official runners; please report self-hosted issues via GitHub.

## Should I use Rust for GitHub Actions?

Rust delivers excellent performance and safety, but using it for Actions does add complexity. Ensure the benefits fit your project needs.

---

## Contributing

Improvements are welcome—whether for caching, runners, documentation, or functionality.
Open an [issue](https://github.com/joluizquierdo/github-action-rust-template/issues) or pull request.

---

## License

This project is licensed under the [GPL-3.0](./LICENSE.txt).
