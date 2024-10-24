use std::path::PathBuf;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncReadExt;
use tokio::sync::watch;
use tokio::time::{sleep, Duration};

async fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = AsyncFile::open(filename).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
// Action 6
pub(crate) async fn file_demo() {
    let filename = "data.txt";
    let contents = read_file(filename).await.unwrap();
    let (tx, mut rx) = watch::channel(false); // single producer, multiple consumers channel
    tokio::spawn(async { watch_file_changes(tx).await });
    loop {
        if rx.changed().await.is_ok() {
            let new_contents = read_file(filename).await.unwrap();
            println!("File contents: {}", new_contents);
        }
        sleep(Duration::from_millis(100)).await;
    }
}

// We can now move onto our loop that periodically checks the metadata of our file with the following code:
async fn watch_file_changes(tx: watch::Sender<bool>) {
    let path = PathBuf::from("data.txt");

    let mut last_modified = None;
    loop {
        if let Ok(metadata) = path.metadata() {
            let modified = metadata.modified().unwrap();

            if last_modified != Some(modified) {
                last_modified = Some(modified);
                let _ = tx.send(true);
            }
        }
        sleep(Duration::from_millis(100)).await;
    }
}