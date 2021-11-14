use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::Value;
use std::fs::File;
use std::{cmp::min, process::exit};
use std::{
    io::Write,
    sync::{mpsc::channel, Arc, Mutex},
};
use tokio::runtime::Runtime;

use crate::log;

fn basic_headers() -> HeaderMap<HeaderValue> {
    let mut headers = HeaderMap::new();
    headers.append("User-Agent", HeaderValue::from_str("").unwrap());
    headers
}

pub fn get(url: &str) -> Value {
    let rt = Runtime::new().unwrap();
    let url_arc = Arc::new(Mutex::new(url.clone()));
    let (tx, rx) = channel();

    rt.block_on(async move {
        let u = url_arc.lock().unwrap().to_string();

        let client = Client::new();
        let response = client.get(u).headers(basic_headers()).send().await;
        let data = response.unwrap().text().await.unwrap();
        let js: Value = serde_json::from_str(&data).unwrap();

        let _ = tx.send(js);
    });

    let data = rx.recv().unwrap();
    data
}

pub fn download_file(url: &str, path: &str) {
    let rt = Runtime::new().unwrap();
    let url_arc = Arc::new(Mutex::new(url.to_string()));
    let path_arc = Arc::new(Mutex::new(path.to_string()));

    rt.block_on(async move {
        let u = url_arc.lock().unwrap().to_string();
        let p = path_arc.lock().unwrap().to_string();
        let v = p.split("/").last().unwrap();

        let client = Client::new();
        let response = client.get(&u).headers(basic_headers()).send().await.unwrap();
        let size = response.content_length().unwrap();

        let pb = ProgressBar::new(size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("#>-")); 
        pb.set_message(format!("-> Downloading {}", v));

        let file_create = File::create(&p);
        if file_create.is_err() {
            log::error(&format!("Failed to create file: {}", &p));
            log::error(&file_create.err().unwrap().to_string());
            exit(-1);
        }

        let mut file = file_create.unwrap();
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("Error while downloading file"))).unwrap();

            file.write(&chunk)
                .or(Err(format!("Error while writing to file"))).unwrap();

            let new = min(downloaded + (chunk.len() as u64), size);
            downloaded = new;
            pb.set_position(new);
        }
    });
}
