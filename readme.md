# Azure-blob-downloader in Rust

This is a simple Azure blob downloader written in Rust. It is a simple command line tool that can be used to download blobs from Azure storage account.

## Usage

- First, set the environment variables `AZURE_STORAGE_ACCOUNT` and
`AZURE_SAS_TOKEN` with the values of your Azure storage account
and SAS token respectively.
- If you don't pass the file name as the last argument, it will download the entire directory.

## Example for directory download

```bash
cargo run mycontainer mydirectory /home/user/downloads
```

This will download from the container the directory `mydirectory` 
in the container `mycontainer` to the directory `/home/user/downloads`.
in the local machine.

## Example for single file download

```bash
cargo run mycontainer mydirectory /home/user/downloads myfile.txt
```

If you want to download a single file, you can specify the file name as the last argument.
This will download the file `myfile.txt` from the directory `mydirectory` in the container `mycontainer`
to the directory `/home/user/downloads` in the local machine.

## Note

- It works by making a GET request to the Azure REST API to get the list of
  blobs in the specified container and directory.
  It then downloads the blobs one by one to the specified destination path.
- The documentation for the Azure REST API can be found [here](https://docs.microsoft.com/en-us/rest/api/storageservices/).