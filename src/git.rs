use std::{process::Command, env, path::Path};

pub fn clone(url: &str, path: &str, branch: &str) -> std::io::Result<()> {
    if Path::new(format!("{}/.git/HEAD", path).as_str()).is_file() {
        // Git repo already exists, assume it's correct.
        Ok(())
    } else {
        Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(path)
            .arg("-b")
            .arg(branch)
            .status()?;
        Ok(())
    }
}

pub fn pull(repo_path: &str) -> std::io::Result<()> {
    let cwd = env::current_dir().unwrap();
    env::set_current_dir(repo_path).expect(format!("Failed to move into {}", repo_path).as_str());
    
    Command::new("git")
        .arg("pull")
        .status()?;

    // Move back to current dir
    env::set_current_dir(cwd).unwrap();

    Ok(())
}

pub fn add(repo_path: &str, file: &str) -> std::io::Result<()> {
    let cwd = env::current_dir().unwrap();
    env::set_current_dir(repo_path).expect(format!("Failed to move into {}", repo_path).as_str());
    
    Command::new("git")
        .arg("add")
        .arg(file)
        .status()?;

    // Move back to current dir
    env::set_current_dir(cwd).unwrap();

    Ok(())
}

pub fn commit(repo_path: &str, msg: &str) -> std::io::Result<()> {
    let cwd = env::current_dir().unwrap();
    env::set_current_dir(repo_path).expect(format!("Failed to move into {}", repo_path).as_str());
    
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .status()?;

    // Move back to current dir
    env::set_current_dir(cwd).unwrap();

    Ok(())
}

pub fn push(repo_path: &str) -> std::io::Result<()> {
    let cwd = env::current_dir().unwrap();
    env::set_current_dir(repo_path).expect(format!("Failed to move into {}", repo_path).as_str());
    
    Command::new("git")
        .arg("push")
        .status()?;

    // Move back to current dir
    env::set_current_dir(cwd).unwrap();

    Ok(())
}
