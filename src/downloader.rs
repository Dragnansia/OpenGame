use crate::error::unv;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::Value;
use std::{cmp::min, fs::File, io::Write};

fn basic_headers() -> HeaderMap<HeaderValue> {
    let mut headers = HeaderMap::new();
    headers.append("User-Agent", HeaderValue::from_str("").unwrap());
    headers
}

pub async fn get(url: &str) -> Result<Value, unv::Error> {
    let client = Client::new();
    let response = client.get(url).headers(basic_headers()).send().await?;
    let data = response.text().await.unwrap_or_default();

    Ok(serde_json::from_str(&data)?)
}

// Todo: Add a Result for return for check if this download is ok
pub async fn download_file(url: &str, path: &str) -> Result<(), unv::Error> {
    let v = path.split('/').last().unwrap_or("download.tmp");

    let client = Client::new();
    let response = client.get(url).headers(basic_headers()).send().await?;
    let size = response.content_length().unwrap_or_default();

    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("#>-"));
    pb.set_message(format!("-> Downloading {}", &v));

    let mut file = File::create(&path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;

        downloaded = min(downloaded + (chunk.len() as u64), size);
        pb.set_position(downloaded);
    }

    Ok(())
}
