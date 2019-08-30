use aws_uploader::watch;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let watch_dir: &str = &args[1];
    if !Path::new(watch_dir).is_dir() {
        panic!("Expected path to directory");
    }

    watch(watch_dir);
}
