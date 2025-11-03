// This file is part of joluizquierdo/github-action-template.
// Copyright (C) 2025 joluizquierdo
// Licensed under the GNU GPL v3. See LICENSE file in root directory.
mod actions;
use actions::github::GitHubContext;
use actions::utils::log_notice;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
struct Args {
    #[clap(long)]
    name: String,
    #[clap(long)]
    surname: String,
}

fn main() {
    let args = Args::parse();
    let mut github_context = GitHubContext::new();

    // Action logic
    let full_greeting = format!(
        "Hello {} {}, this is an action in rust 🦀!",
        args.name, args.surname
    );
    println!("{full_greeting}");
    // you can use the token if needed
    log_notice(
        format!(
            "Using GitHub token of length {}",
            github_context.token.len()
        )
        .as_str(),
    );

    // create outputs
    github_context.set_output("greeting-message", &full_greeting);
}
