use std::time::UNIX_EPOCH;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

const COPY_DIR: &'static str = "assets";

pub fn file_modified_time_in_seconds(path: &str) -> u64 {
    fs::metadata(path)
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// A helper function for recursively copying a directory.
fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            if to.exists() {
                if file_modified_time_in_seconds(path.as_os_str().to_str().unwrap())
                    != file_modified_time_in_seconds(to.as_os_str().to_str().unwrap())
                {
                    fs::copy(&path, to).unwrap();
                }
            } else {
                fs::copy(&path, to).unwrap();
            }
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        } else { /* Skip other content */
        }
    }
}

fn main() {
    // Request the output directory
    let out = env::var("PROFILE").unwrap();
    let out = PathBuf::from(format!("target/{}/{}", out, COPY_DIR));

    // If it is already in the output directory, delete it and start over
    if out.exists() {
        fs::remove_dir_all(&out).unwrap();
    }

    // Create the out directory
    fs::create_dir(&out).unwrap();

    // Copy the directory
    copy_dir(COPY_DIR, &out);
}
