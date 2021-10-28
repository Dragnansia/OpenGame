use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::Value;
use std::{io::Write, sync::{mpsc::channel, Arc, Mutex}};
use tokio::runtime::Runtime;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;
use std::cmp::min;
use std::fs::File;

fn basic_hearders() -> HeaderMap<HeaderValue> {
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
        let response = client.get(u).headers(basic_hearders()).send().await;
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

        let client = Client::new();
        let response = client.get(&u).headers(basic_hearders()).send().await.unwrap();
        let size = response.content_length().unwrap();

        let pb = ProgressBar::new(size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("#>-"));
        // TODO change download message and just print proton version and not url
        pb.set_message(format!("-> Downloading {}", &u));

        let mut file = File::create(path).or(Err(format!("-> Failed to create file '{}'", p))).unwrap();
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
