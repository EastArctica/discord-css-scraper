pub mod git;
use std::collections::HashSet;
use regex::Regex;
use time::OffsetDateTime;

fn download_current_css(repo_url: &str, repo_path: &str) -> Result<(), ()> {
    git::clone(repo_url, repo_path).expect("Failed to clone repo");
    git::pull(repo_path).unwrap();

    let css_regex = Regex::new(r#"<link rel=\"stylesheet\" href=\"(\/assets\/([0-9a-f]{3}\.[0-9a-f]{20}\.css))\">"#).unwrap();
    let req = reqwest::blocking::get("https://discord.com/@me");
    if req.is_ok() {
        let res = req.unwrap().text().unwrap();

        let files_str = std::fs::read_to_string(format!("{}/files.txt", repo_path)).expect("No repo/files.txt");
        let mut files: HashSet<String> = files_str.lines().map(String::from).collect();
        let mut has_new_files = false;

        let now = OffsetDateTime::now_utc();
        let current_dir = format!("{}/{}/{}/{}", repo_path, now.year(), now.month(), now.day());
        
        // There should only ever be 1 file, but if there were ever multiple we could handle it.
        for cap in css_regex.captures_iter(res.as_str()) {
            // Don't re-add any files
            if files.contains(&cap[1].to_string()) {
                continue;
            }

            // Since this file is new, download it into repo/{year}/{month}/{day}/{file}
            std::fs::create_dir_all(current_dir.clone()).unwrap();
            
            // If our download fails, there's not much we can do so we just hope it get's downloaded on the next go
            let req = reqwest::blocking::get(format!("https://discord.com/{}", &cap[1]));
            
            if req.is_err() {
                println!("Failed to download '{}'!", &cap[1]);
                continue;
            }

            has_new_files = true;

            // Save the download
            let res = req.unwrap().text().unwrap();
            let file_path = format!("{}/{}", current_dir, &cap[2]);

            std::fs::write(file_path.clone(), res).expect(format!("Failed to save {}", &cap[1]).as_str());
            // Add this file to the files list
            files.insert(cap[1].to_string());
            git::add(repo_path, format!("{}/{}/{}/{}", now.year(), now.month(), now.day(), &cap[2]).as_str()).unwrap();
        }

        // If we've downloaded anything, update the repo
        if has_new_files {
            // Lol this is dumb but chatgpt told me to
            let updated_files_str = files.iter().cloned().collect::<Vec<String>>().join("\n");
            std::fs::write(format!("{}/files.txt", repo_path), updated_files_str).expect("Failed to update files.txt");
            git::add(repo_path, "files.txt").unwrap();

            // Create commit and push
            git::commit(repo_path, format!("{}-{}-{} - {}:{}", now.year(), now.month().to_string(), now.day(), now.hour(), now.minute()).as_str()).unwrap();
            git::push(repo_path).unwrap();
        }

        Ok(())
    } else {
        println!("{}", req.unwrap_err());
        Err(())
    }
}

fn main() {
    const REPO_URL: &str = "https://github.com/EastArctica/discord-css-files";
    const REPO_PATH: &str = "discord-css-files";
    loop {
        download_current_css(REPO_URL, REPO_PATH).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(60 * 5));
    }
}
