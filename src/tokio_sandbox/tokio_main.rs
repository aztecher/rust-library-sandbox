use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::fs::{File, remove_file};
use tracing::{info, instrument};

#[instrument]
async fn create_file(filepath: &str) -> io::Result<()>{
    info!("create {}", filepath);
    let _ = File::create(filepath).await?;
    Ok(())
}

#[instrument]
async fn delete_file(filepath: &str) -> io::Result<()>{
    info!("delete {}", filepath);
    let _ = remove_file(filepath).await?;
    Ok(())
}

#[instrument]
async fn write(filepath: &str, bytes: &[u8]) -> io::Result<()> {
    info!("write {:?} in file {}", bytes, filepath);
    let mut file = File::create(filepath).await?; // open a file in write-only mode
    file.write_all(bytes).await?; // need to use tokio::io::AsyncWriteExt
    Ok(())
}

#[instrument]
async fn read(filepath: &str, bytes: &mut Vec<u8>) -> io::Result<()> {
    let mut file = File::open(filepath).await?; // open a file in read-only mode
    file.read_to_end(bytes).await?; // need to use tokio::io::AsyncReadExt
    info!("read {:?} from file {}", bytes, filepath);
    Ok(())
} #[tokio::main]
async fn main() -> io::Result<()> {
    // setup tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let file: &str = "test.txt";
    let _ = create_file(file).await?;
    let _ = write(file, b"this is input").await?;
    let contents: &mut Vec<u8> = &mut Vec::new();
    let _ = read(file, contents).await?;
    info!("raw contents: {:?}", contents);
    match std::str::from_utf8(contents.as_slice()) {
        Ok(s) => {
            info!("contents from read function: {}", s);
            let _ = delete_file(file).await?;
            Ok(())
        },
        Err(e) => {
            info!("read error!");
            delete_file(file).await?;
            panic!("Invalid UTF-8 sequence {}", e)
        }
    }
}
