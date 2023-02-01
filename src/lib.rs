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

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const COMMIT_HASH: &str = env!("UME_COMMIT_HASH");
pub const BUILD_DATE: &str = env!("UME_BUILD_DATE");

#[macro_use]
extern crate log;

pub mod catchers;
pub mod cli;
pub mod config;
pub mod null_writer;
pub mod providers;
pub mod routes;
pub mod server;
pub mod setup_utils;
