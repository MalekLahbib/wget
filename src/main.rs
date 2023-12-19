pub mod mirror;

extern crate reqwest;
use crate::mirror::mirror_url;

use std::thread::sleep;
use chrono::{ Utc, Datelike, Timelike };
use indicatif::{ ProgressBar, ProgressStyle };
use std::env::args;
use std::fs::{ create_dir_all, File };
use std::io::{ copy, Read, Write, BufReader, BufRead };
use std::time::Duration;
use std::process::{ Command, Stdio };

fn main() {
    let args = args().collect::<Vec<String>>();
    let mut url = String::new();
    let mut to_file = false;
    let mut filename = String::new();
    let mut pathname = String::new();
    let mut limit_rate = false;
    let mut ratelimit = String::new();
    let mut multiurl = false;
    let mut urls_file = String::new();
    let mut mirror = false;
    let mut reject = false;
    let mut reject_value = String::new();

    if args.len() == 1 {
        println!("No arguments given");
        return;
    } else if args.len() > 2 {
        for arg in args {
            if arg == "./wget" {
                continue;
            }
            match arg {
                arg if arg.contains("http://") || arg.contains("https://") => {
                    url = arg;
                }
                arg if arg.contains("-B") => {
                    to_file = true;
                }
                arg if arg.contains("-O") => {
                    let filen = arg.split("=").last().to_owned().unwrap();
                    filename = filen.to_string();
                }
                arg if arg.contains("-P") => {
                    let pathn = arg.split("=~/").last().to_owned().unwrap();
                    pathname = pathn.to_string();
                    println!("{}", pathname);
                }
                arg if arg.contains("--rate-limit") => {
                    ratelimit = arg.split("=").last().to_owned().unwrap().to_string();
                    limit_rate = true;
                }
                arg if arg.contains("--mirror") => {
                    mirror = true;
                }
                arg if arg.contains("-X") || arg.contains("--reject") => {
                    reject = true;
                    reject_value = arg.split("=").last().to_owned().unwrap().to_string();
                }
                _ => {
                    println!("bad argument: {:?}", arg);
                    return;
                }
            }
        }
    } else {
        if args[1].contains("-i") {
            let filem = args[1].split("=").last().to_owned().unwrap();
            urls_file = filem.to_string();
            multiurl = true;
        } else {
            url = args[1].to_owned();
        }
    }

    if limit_rate {
        let _ = execute_wget(&url, &ratelimit);
        return;
    }

    if multiurl {
        let input = File::open(urls_file).unwrap();
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            if
                let Err(err) = download_file(
                    &line.unwrap(),
                    to_file,
                    &filename,
                    &pathname,
                    &ratelimit
                )
            {
                eprintln!("Error: {}", err);
            }
        }
        return;
    }

    if mirror {
        mirror_url(&url, reject, reject_value);
        return;
    }

    if let Err(err) = download_file(&url, to_file, &filename, &pathname, &ratelimit) {
        eprintln!("Error: {}", err);
    }
}

fn download_file(
    url: &str,
    to_file: bool,
    file_name: &str,
    path_name: &str,
    _ratelimit: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut filename;
    let mut output_file = File::create("wget-log")?;
    if file_name.is_empty() {
        let filen = url.split("/").last().unwrap();
        filename = filen.to_string();
    } else {
        filename = file_name.to_string();
    }
    if !path_name.is_empty() {
        create_dir_all(path_name)?;
        filename = format!("./{}/{}", path_name, filename);
    } else {
        filename = format!("./{}", filename);
    }
    let now = Utc::now();
    if to_file {
        println!("Output will be written to \"wget-log\".");
        write!(
            output_file,
            "start at {}-{:02}-{:02} {:02}:{:02}:{:02}\n",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        )?;
        write!(output_file, "sending request, awaiting response...")?;
    } else {
        println!(
            "start at {}-{:02}-{:02} {:02}:{:02}:{:02}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        );
        print!("sending request, awaiting response...");
    }
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).send()?;
    let total_size = response.content_length().unwrap_or(0);

    let status = &response.status();
    if status.is_success() {
        if !to_file {
            println!(" status {status}");
            println!("content size: {:?} [~{:.2}MB]", total_size, (total_size as f32) / 1000000.0);
            println!("saving file to: {}", filename);
        } else {
            write!(output_file, " status {status}\n")?;
            write!(
                output_file,
                "content size: {:?} [~{:.2}MB]\n",
                total_size,
                (total_size as f32) / 1000000.0
            )?;
            write!(output_file, "saving file to: {}\n", filename)?;
        }
    }

    if !to_file {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{bytes}/{total_bytes} [{bar:50}] {percent}% {bytes_per_sec} {eta}")?
                .progress_chars("=> ")
        );

        let mut dest_file = File::create(filename)?;

        let mut downloaded = 0;
        let mut buffer = [0; 1024];
        //let mut rate_down = 0;
        loop {
            let bytes_read = response.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            dest_file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read as u64;
            pb.set_position(downloaded);

            //rate limit experiment

            // rate_down += bytes_read as u64;
            // if rate_down > 10 * 1024 * 1024 {
            //     pb.set_position(downloaded);
            //     sleep(Duration::from_secs(1));
            //     rate_down = 0;
            // }
        }

        //pb.finish_with_message("File downloaded successfully!");
        println!(" ");
        println!("Downloaded [{}]", url);
        let now = Utc::now();
        println!(
            "Finished at {}-{:02}-{:02} {:02}:{:02}:{:02}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        );
    } else {
        let mut dest_file = File::create(filename)?;

        // Copy the response body to the destination file
        copy(&mut response, &mut dest_file)?;
        write!(output_file, "Downloaded [{}]\n", url)?;
        let now = Utc::now();
        write!(
            output_file,
            "Finished at {}-{:02}-{:02} {:02}:{:02}:{:02}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        )?;
    }

    Ok(())
}

fn execute_wget(url: &str, rate: &str) -> std::io::Result<()> {
    let limit_rate = format!("--limit-rate={}", rate);
    let mut child = Command::new("wget").arg(url).arg(limit_rate).stdout(Stdio::piped()).spawn()?;

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
