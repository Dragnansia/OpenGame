use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::{Map, Value};
use std::sync::{mpsc::channel, Arc, Mutex};
use tokio::runtime::Runtime;

pub struct DownloadInfo {
    _is_download: bool,
    _download_path: String,
}

// TODO: use block_on of rt and copy the current
// program on download file
pub fn get(url: &'static str) -> Value {
    let rt = Runtime::new().unwrap();
    let url_copy = url.clone();
    let url_arc = Arc::new(Mutex::new(url_copy));
    let (tx, rx) = channel();
    let (txj, rxj) = channel();

    rt.spawn(async move {
        let u = url_arc.lock().unwrap().to_string();

        let mut headers = HeaderMap::new();
        headers.append("User-Agent", HeaderValue::from_str("").unwrap());

        let client = Client::new();

        let _ = tx.send(true);
        let response = client.get(u).headers(headers).send().await;
        let data = response.unwrap().text().await.unwrap();
        let js: Value = serde_json::from_str(&data).unwrap();
        let _ = tx.send(false);

        let _ = txj.send(js);
    });

    let mut is_download = rx.recv().unwrap();
    while is_download {
        is_download = rx.try_recv().unwrap_or(true);
    }

    rxj.recv().unwrap()
}

pub fn download_file(url: &'static str) -> DownloadInfo {
    let response = get(url);
    for r in response.as_array().unwrap() {
        println!("{}", r["tag_name"].to_string());
    }

    DownloadInfo {
        _is_download: false,
        _download_path: String::from(""),
    }
}
