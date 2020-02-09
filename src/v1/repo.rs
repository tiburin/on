use super::Req;
use std::cmp;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Repo<'a> {
  pub public: String,
  pub req: &'a Req<'a>,
}

impl<'a> Repo<'a> {
  pub fn new(req: &'a Req<'a>) -> Self {
    let path = std::env::current_dir().unwrap();
    let public = format!("{}/public", path.display());
    Self { public, req }
  }
  pub fn read_line(&self, path: &str, rank: &str) -> Result<String, io::Error> {
    let content = BufReader::new(File::open(path)?);
    let result = content
      .lines()
      .filter_map(|line| line.ok())
      .filter(|line| line != "")
      .find(|line| line.trim().starts_with(rank));

    match result {
      Some(n) => Ok(n),
      None => Ok(format!("{},none: none", rank)),
    }
  }
  pub fn read_file(&self, name: &str) -> Result<String, io::Error> {
    let dir = match &self.req.file {
      Some("js") => self.path("js"),
      Some("off") => self.path("spoken"),
      Some("css") => self.path("css"),
      _ => self.public.to_string(),
    };

    let path = format!("{}/{}", dir, name);
    Ok(fs::read_to_string(path)?)
  }
  pub fn list_languages(&self, path: &str) -> Result<String, io::Error> {
    let path = Path::new(&path);

    let mut result = String::new();
    for files in path.read_dir()? {
      if let Ok(file) = files {
        let name = file.file_name();

        if let Some(lang) = Path::new(&name).file_stem() {
          let spoken = format!("{}\n", lang.to_str().unwrap_or(""));
          result.push_str(&spoken);
        }
      }
    }
    Ok(result.trim().to_string())
  }
  pub fn count(&self) -> String {
    let english_content = self.read_file("spoken/english.on");
    let espanol_content = self.read_file("spoken/espanol.on");

    match (english_content, espanol_content) {
      (Ok(english), Ok(espanol)) => {
        let total = |n: String| n.split("\n").filter(|&value| value != "").count();
        let max = cmp::max(total(english), total(espanol));
        max.to_string()
      }
      _ => 0.to_string(),
    }
  }
  pub fn path(&self, file_name: &str) -> String {
    format!("{}/{}", self.public, file_name)
  }
}

#[cfg(test)]
mod tests {
  use crate::*;
  use std::fs;
  fn read_file(file: &str) -> Vec<String> {
    let content = fs::read_to_string(file).unwrap();
    content.trim().split("\n").map(|n| n.to_string()).collect()
  }
  #[test]

  fn read_line_test() {
    let path = repo!().path("spoken/english.on");
    let left = repo!().read_line(&path, "2");
    let right = read_file(&path);
    let file = repo!().read_line(&path, "-22").unwrap();
    assert_eq!(left.unwrap(), right[1]);
    assert_eq!(file, format!("{},none: none", "-22"));
  }
  #[test]
  fn count_test() {
    assert!(repo!().count().parse::<i32>().is_ok());
  }
  #[test]
  fn list_languages_test() {
    let temp_path = format!("{}spoken", &std::env::temp_dir().display());
    if fs::read_dir(&temp_path).is_err() {
      fs::create_dir(&temp_path).unwrap();
    }
    assert_eq!(repo!().list_languages(&temp_path).unwrap(), "");

    let path = &format!("{}/{}", repo!().public, "spoken");
    let res = repo!().list_languages(path).unwrap();
    assert!(res.contains("english") && res.contains("espanol"));
    fs::remove_dir_all(temp_path).unwrap();
  }
}
