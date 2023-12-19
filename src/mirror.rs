use std::io::{ copy, Write };
use reqwest;
use std::fs::{ File, create_dir_all };

pub fn mirror_url(url: &String, reject: bool, reject_value: String) {
    let path = url.split("//").last().unwrap().to_string();
    create_dir_all(&path).expect("unable to create directory");
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let indexpath = format!("./{path}/index.html");
    let mut dest_file = File::create(indexpath).expect("unable to create index file");
    write!(dest_file, "{}", response).expect("unable to write to index file");
    let lines = response.split(">");
    for s in lines {
        //println!("s: {s}");
        if s.contains("url(") {
            let vec = s.split("url(").collect::<Vec<_>>();
            for v in vec {
                if v.contains("'") {
                    let name = v.split("'").collect::<Vec<_>>()[1];
                    let relative_url = format!("{}/{}", url, name);
                    let destf = format!("{path}{name}");
                    download_a_file(relative_url, destf);
                }
            }
        }

        if (s.contains("img ") && s.contains("src=")) || s.contains("img src=") {
            let vec = s.split_whitespace().collect::<Vec<_>>();
            for v in vec {
                let x: &[_] = &['\'', '"'];
                if v.contains("src=") && (!reject || (reject && !v.contains(&reject_value))) {
                    let mut res = v.split("src=").collect::<Vec<_>>()[1];
                    if res.split("/").collect::<Vec<_>>().len() > 1 {
                        let mut vec = res.split("/").collect::<Vec<_>>();
                        let len = vec.len() - 1;
                        vec[len] = vec[len].trim_matches(x);
                        for i in 0..vec.len() {
                            if vec[i].chars().all(char::is_alphanumeric) {
                                let dir = format!("{}{}", path, vec[i]);
                                create_dir_all(dir).expect("unable to create directory");
                                let relative_url = format!("{}/{}/{}", url, vec[i], vec[i + 1]);
                                let destf = format!("{path}{}/{}", vec[i], vec[i + 1]);
                                download_a_file(relative_url, destf);
                                break;
                            }
                        }
                    } else {
                        res = res.trim_matches(x);
                        let relative_url = format!("{}/{}", url, res);
                        let destf = format!("{path}{}", res);
                        download_a_file(relative_url, destf);
                    }
                }
            }
        }

        if
            (s.contains("a href") || s.contains("link href")) &&
            !s.contains("http") &&
            !s.contains(".com")
        {
            let mut link: &str = s.split("href=").collect::<Vec<_>>()[1];
            link = link.split("\"").collect::<Vec<_>>()[1];
            let destf;
            if link.split("/").collect::<Vec<_>>().len() > 1 {
                let res = link.split("/").collect::<Vec<_>>();
                let dir = format!("{}{}", path, res[0]);
                create_dir_all(dir).expect("unable to create directory");
                destf = format!("./{}{}/{}", path, res[0], res[1]);
            } else {
                destf = format!("./{}/{}", path, link);
            }
            let relative_url = format!("{}{}", url.clone(), link);
            download_a_file(relative_url, destf);
        }
    }
}

pub fn download_a_file(path: String, destf: String) {
    let mut resp = reqwest::blocking::get(path).unwrap();
    let mut dest_file = File::create(destf).expect("error creating file");
    // Copy the response body to the destination file
    copy(&mut resp, &mut dest_file).expect("error copying response");
}
