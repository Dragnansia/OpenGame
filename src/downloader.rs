use crate::log::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::Value;
use std::{cmp::min, fs::File, io::Write, process::exit};

fn basic_headers() -> HeaderMap<HeaderValue> {
    let mut headers = HeaderMap::new();
    headers.append("User-Agent", HeaderValue::from_str("").unwrap());
    headers
}

pub async fn get(url: &str) -> Value {
    let client = Client::new();
    let response = client.get(url).headers(basic_headers()).send().await;
    let data = response.unwrap().text().await.unwrap();

    serde_json::from_str(&data).unwrap()
}

// Todo: Add a Result for return for check if this download is ok
pub async fn download_file(url: &str, path: &str) {
    let v = path.split("/").last().unwrap();

    let client = Client::new();
    let response = client
        .get(url)
        .headers(basic_headers())
        .send()
        .await
        .unwrap();
    let size = response.content_length().unwrap();

    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("#>-"));
    pb.set_message(format!("-> Downloading {}", v));

    let mut file = match File::create(&path) {
        Ok(fc) => fc,
        Err(err) => {
            error!("Failed to create file: {}", &path);
            error!("{}", err.to_string());
            exit(-1);
        }
    };
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item
            .or(Err(format!("Error while downloading file")))
            .unwrap();

        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))
            .unwrap();

        downloaded = min(downloaded + (chunk.len() as u64), size);
        pb.set_position(downloaded);
    }
}
