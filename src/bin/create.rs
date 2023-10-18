use anyhow::Context;
use clap::Parser;
use imatree::*;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    args.render_text_to_png_data()
        .context("Failed to render text to png data")
}
