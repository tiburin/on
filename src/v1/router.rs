use super::{repo::Repo, Conn, Method};

#[derive(Debug)]
pub struct Router<'a, 'b> {
  conn: &'a mut Conn<'b>,
  repo: Repo<'a>,
}

impl<'a, 'b> Router<'a, 'b> {
  pub fn new(conn: &'a mut Conn<'b>, repo: Repo<'a>) -> Self {
    Self { conn, repo }
  }

  pub fn route(&mut self) {
    if self.conn.req.method == Method::GET {
      self.get()
    }
  }
  fn get(&mut self) {
    let path = self.conn.req.path.as_ref().unwrap().as_ref();
    match path {
      "/" => self.html("home.html"),
      "/spoken/all" => self.html("all.html"),
      "/spoken" => self.html("spoken.html"),
      "/compute" => self.html("compute.html"),
      "/spoken/word" => self.html("word.html"),
      "/spoken/english" => self.on("spoken/english.on"),
      "/spoken/espanol" => self.on("spoken/espanol.on"),
      "/spoken/languages" => self.langs(),
      "/spoken/count" => self.put_content("off", self.repo.count()),

      _ => self.files(),
    }
  }
  fn html(&mut self, path: &str) {
    let body = self.repo.read_file(path).expect("HTML file missing");
    let content = format!("{}{}{}", self.conn.res.header, body, self.conn.res.footer);
    self.put_content("html", content);
  }
  fn on(&mut self, path: &str) {
    self.put_content("off", self.repo.read_file(path).unwrap());
  }
  fn langs(&mut self) {
    let path = format!("{}/spoken", self.repo.public);
    self.put_content("off", self.repo.list_languages(&path).unwrap());
  }
  fn put_content(&mut self, file: &str, content: String) {
    let mut res = &mut self.conn.res;
    res.status = "200 Ok";
    res.body = content;
    match file {
      "html" => res.file = "text/html",
      "css" => res.file = "text/css",
      "js" => res.file = "application/javascript",
      _ => (),
    }
  }

  fn files(&mut self) {
    let req = &self.conn.req;
    if let Some(name) = req.path_list.last() {
      if let Some(file) = req.file {
        let path = format!("{}.{}", name, file);
        if let Ok(content) = self.repo.read_file(&path) {
          self.put_content(file, content)
        }
      } else {
        if let Some(rank) = &req.rank {
          if let Some(content) = self.repo.read_line(name, rank) {
            self.put_content("off", content)
          }
        }
      }
    }
  }
}
#[cfg(test)]
mod tests {
  use crate::v1::store;
  use crate::v1::store::Storage;
  use crate::v1::Res;
  use crate::*;
  use std::sync::Arc;
  fn not_found(res: &Res) {
    assert_eq!(res.body, "Not found");
    assert_eq!(res.status, "404 NOT FOUND");
    assert_eq!(res.file, "text/plain");
  }

  fn get_storate() -> Arc<Storage> {
    Arc::new(store::Storage::new())
  }

  #[test]
  fn off_file_test() {
    let mut conn = conn!("GET", "/spoken/english");
    let storage = Arc::new(store::Storage::new());
    let res = conn.router(storage);
    assert_eq!(res.status, "200 Ok");
    assert_eq!(res.file, "text/plain");
    assert!(res.body.contains("1,"));
  }
  #[test]
  fn js_file_test() {
    let mut conn = conn!("GET", "app.js");
    let res = conn.router(get_storate());
    assert_eq!(res.status, "200 Ok");
    assert_eq!(res.file, "application/javascript");
    assert!(res.body.contains("const"));
  }
  #[test]
  fn css_file_test() {
    let mut conn = conn!("GET", "app.css");
    let res = conn.router(get_storate());
    assert_eq!(res.status, "200 Ok");
    assert_eq!(res.file, "text/css");
    assert!(res.body.contains("background-color"));
  }
  #[test]
  fn path_not_found_test() {
    for n in vec!["GET", "POST", "DELETE", "PUT"] {
      let mut conn = conn!(n, "/dev/test");
      not_found(conn.router(get_storate()));
    }
  }
}
