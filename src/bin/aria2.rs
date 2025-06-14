use anyhow::Result;

use downloader_wrapper::config::Downloader;

fn main() -> Result<()> {
    downloader_wrapper::run(Downloader::Aria2c)
}
