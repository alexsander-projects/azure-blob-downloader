# Azure-blob-downloader in Rust

This is a simple Azure blob downloader written in Rust. It is a simple command line tool that can be used to download blobs from Azure storage account.

## Usage

- First, set the environment variables `AZURE_STORAGE_ACCOUNT` and
`AZURE_SAS_TOKEN` with the values of your Azure storage account
and SAS token respectively.
- Run the command `cargo run <container_name> <directory_name> <download_directory>` to download the blob to the
specified destination path.

## Example

```bash
cargo run mycontainer mydirectory /home/user/downloads
```

This will download the blob from the container `mycontainer` and the directory `mydirectory` to the directory `/home/user/downloads`.

