use reqwest::Client;
use std::{env, fs};
use std::fs::File;
use std::io::copy;
use std::path::Path;
use tokio::runtime::Runtime;
use futures_util::stream::StreamExt;
use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Blobs {
    blob: Vec<Blob>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Blob {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ListBlobsResponse {
    blobs: Blobs,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        return Err("Usage: azure_blob_download <container_name> <directory_name> <download_directory>".into());
    }
    let account_name = env::var("AZURE_STORAGE_ACCOUNT").expect("Set AZURE_STORAGE_ACCOUNT env variable");
    let sas_token = env::var("AZURE_SAS_TOKEN").expect("Set AZURE_SAS_TOKEN env variable");

    let container_name = &args[1];
    let directory_name = &args[2];
    let download_directory = Path::new(&args[3]);


    if !download_directory.is_absolute() {
        return Err("The download directory must be an absolute path.".into());
    }

    let list_url = format!(
        "https://{}.blob.core.windows.net/{}/?restype=container&comp=list&prefix={}&{}",
        account_name, container_name, directory_name, sas_token
    );

    let mut rt = Runtime::new()?;
    let client = Client::new();

    let list_response = rt.block_on(client.get(&list_url).send())?;
    let list_body = rt.block_on(list_response.text())?;
    let list_blobs_response: ListBlobsResponse = from_str(&list_body)?;

    for blob in list_blobs_response.blobs.blob {
        let blob_name = &blob.name;
        let url = format!(
            "https://{}.blob.core.windows.net/{}/{}?{}",
            account_name, container_name, blob_name, sas_token
        );

        let response = rt.block_on(client.get(&url).send())?;

        // Create directories if they don't exist
        let path = download_directory.join(blob_name);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let mut out = File::create(&path)?;
        let mut content = response.bytes_stream();
        rt.block_on(async {
            while let Some(item) = content.next().await {
                let chunk = item?;
                copy(&mut chunk.as_ref(), &mut out)?;
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        })?;

        println!("Downloaded {}", path.display());
    }

    Ok(())
}
