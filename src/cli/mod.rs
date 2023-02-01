// 🐻‍❄️💐 ume: Easy, self-hostable, and flexible image host made in Rust
// Copyright (c) 2020-2023 Noel <cutie@floofy.dev>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::Result;
use clap::{Parser, Subcommand};

use self::types::Execute;

pub mod completions;
pub mod config;
pub mod generate;
pub mod server;
pub mod types;
pub mod version;

#[derive(Debug, Parser)]
#[clap(
    name = "ume",
    about = "🐻‍❄️💐 Easy, self-hostable, and flexible image host made in Rust",
    author = "Noel Miko <cutie@floofy.dev>"
)]
pub struct UmeCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Completions(crate::cli::completions::Completions),
    Version(crate::cli::version::Version),
}

pub async fn execute(command: &Commands) -> Result<()> {
    match command {
        Commands::Completions(completions) => completions.execute(),
        Commands::Version(version) => version.execute(),
    }
}
