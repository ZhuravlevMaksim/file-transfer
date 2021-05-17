use reqwest::multipart;
use std::fs::{File, read_dir, DirEntry};
use std::io::{Read, Write};
use std::{env, io};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    println!("Send folder?");
    println!("{:#?}", args);

    if let Some(str_path) = args.get(1) {
        let to = "192.168.1.250";
        let parent = Path::new(str_path);
        let parent_name = parent.file_stem().unwrap().to_str().unwrap().to_owned();
        let ext = Some(OsStr::new("mp3"));
        for entry in read_dir(parent).unwrap() {
            let entry = entry?;
            let name = entry.file_name();
            let path = entry.path();
            if path.is_file() {
                if path.extension() == ext  {
                    println!("{:#?}", name);
                    send_multipart(&to, path, &parent_name).await;
                }
            }
        }

        io::stdin().read_line(&mut String::new()).unwrap();
    }

    Ok(())
}

async fn send_multipart(ip: &str, path: PathBuf, folder: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    let name = path.file_name().unwrap().to_str().unwrap().to_owned();

    File::open(path).unwrap().read_to_end(&mut buffer)?;

    let file = multipart::Part::bytes(buffer)
        .file_name(name);

    let multipart = reqwest::multipart::Form::new()
        .part("files", file);

    let client = reqwest::Client::new();
    let res = client.post(format!("http://{}:12284", ip))
        .header("path", folder)
        .multipart(multipart)
        .send().await?;

    println!("{:#?}", res);

    Ok(())
}