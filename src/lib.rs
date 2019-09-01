use inotify::{EventMask, Inotify, WatchMask};
use rusoto_core::{Region, RusotoError};
use rusoto_s3::{PutObjectError, PutObjectOutput, PutObjectRequest, S3Client, S3};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn watch(watch_dir: &str) {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    println!(
        "Watching current directory for activity {:?}...",
        &watch_dir
    );

    inotify
        .add_watch(
            watch_dir,
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        )
        .expect("Failed to add inotify watch");

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if event.mask.contains(EventMask::CREATE) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory created: {:?}", event.name);
                } else {
                    upload_file(event.name.unwrap().to_str().unwrap(), watch_dir)?;
                    println!("File created: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::DELETE) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory deleted: {:?}", event.name);
                } else {
                    println!("File deleted: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::MODIFY) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory modified: {:?}", event.name);
                } else {
                    println!("File modified: {:?}", event.name);
                }
            }
        }
    }
}

fn upload_file(
    event_name: &str,
    watch_dir: &str,
) -> Result<PutObjectOutput, RusotoError<PutObjectError>> {
    let image_path = Path::new(watch_dir).join(event_name);
    let mut image_file = File::open(image_path)?;
    let mut content = Vec::new();
    image_file.read_to_end(&mut content)?;
    let s3_client = S3Client::new(Region::UsEast2);
    let put_request = PutObjectRequest {
        bucket: String::from("myglasseye.studio.photos"),
        key: event_name.to_string(),
        body: Some(content.into()),
        ..Default::default()
    };

    s3_client
        .put_object(put_request)
        .sync()
        .map_err(|e| e.into())
}
