use parse::Parser;
use router::Router;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::{fmt, thread};
mod parse;
pub mod repo;
mod router;
mod store;

#[derive(Debug, Clone)]
pub struct Config {
  host: &'static str,
  port: i32,
  pub header: String,
  pub footer: String,
}
impl Config {
  pub fn new() -> Self {
    Self {
      host: "127.0.0.1",
      port: 4000,
      header: std::fs::read_to_string("public/header.html").unwrap(),
      footer: std::fs::read_to_string("public/footer.html").unwrap(),
    }
  }
  pub fn address(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Res<'a> {
  status: &'a str,
  body: String,
  file: &'a str,
  header: String,
  footer: String,
}
impl Res<'_> {
  fn new(config: Arc<Config>) -> Self {
    Self {
      status: "404 NOT FOUND",
      body: "Not found".to_owned(),
      file: "text/plain",
      header: config.header.to_owned(),
      footer: config.footer.to_owned(),
    }
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Raw {
  pub header: Option<String>,
  pub body: Option<String>,
}
impl Raw {
  fn new() -> Self {
    Self {
      header: None,
      body: None,
    }
  }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Method {
  GET,
  POST,
  DELETE,
  PUT,
  None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Req<'a> {
  pub raw: Raw,
  pub path: Option<String>,
  pub path_list: Vec<String>,
  pub rank: Option<String>,
  pub method: Method,
  pub file: Option<&'a str>,
}

impl Req<'_> {
  fn new() -> Self {
    Self {
      raw: Raw::new(),
      method: Method::None,
      path: None,
      path_list: Vec::new(),
      rank: None,
      file: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Conn<'a> {
  pub req: Req<'a>,
  pub res: Res<'a>,
}
impl Conn<'_> {
  pub fn new(config: Arc<Config>) -> Self {
    Self {
      req: Req::new(),
      res: Res::new(config),
    }
  }
  fn parse(&mut self, buffer: &[u8], end: usize) -> Result<&Req, MiError> {
    if end > 16 {
      Parser::new(&mut self.req).parse(&buffer[..end])?;
      Ok(&self.req)
    } else {
      Err(MiError::new("Not enough data to parse"))
    }
  }
  fn router(&mut self) -> &Res {
    Router::new(self, repo::Repo::new(&self.req.clone())).route();
    &self.res
  }
  fn send(&mut self) -> String {
    let tipo = format!("Content-Type: {}; charset=utf-8", self.res.file);
    let header = format!("HTTP/1.1 {}\n{}", self.res.status, tipo);
    format!("{}\r\n\r\n{}", header, self.res.body)
  }
}

pub struct Server {
  buffer: [u8; 1024],
}
impl Server {
  pub fn new() -> Self {
    Self { buffer: [0; 1024] }
  }
  pub fn start() {
    let config = Arc::new(Config::new());
    let listener = TcpListener::bind(config.address()).unwrap();
    for stream in listener.incoming() {
      let config = config.clone();
      thread::spawn(move || {
        let mut server = Server::new();
        server.handle_connn(stream.unwrap(), config);
      });
    }
  }
  pub fn handle_connn(&mut self, mut stream: TcpStream, config: Arc<Config>) {
    let end = stream.read(&mut self.buffer).unwrap();

    let mut conn = Conn::new(config);

    conn.parse(&self.buffer, end).unwrap();
    conn.router();
    Server::send(&mut conn, &mut stream);
  }
  fn send(conn: &mut Conn, stream: &mut TcpStream) {
    stream.write(conn.send().as_bytes()).unwrap();
  }
}

#[derive(Debug)]
pub struct MiError {
  msg: &'static str,
}
impl MiError {
  pub fn new(msg: &'static str) -> Self {
    MiError { msg }
  }
}
impl fmt::Display for MiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.msg)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::v1::Config;
  use crate::*;
  pub mod share;

  #[test]
  fn conn_parse_test() {
    let req = conn!().req;
    assert_eq!(req.path.as_ref().unwrap(), "/spoken/espanol/1");
    assert_eq!(req.path_list, ["spoken", "espanol"]);
    assert_eq!(req.rank.as_ref().unwrap(), "1");
    assert!(req.method == Method::GET);
    assert!(req.raw.header.as_ref().unwrap().contains("HTTP/1.1"));
    assert_eq!(req.raw.body.as_ref().unwrap(), &"");
    assert!(req.file.is_none());
  }
  #[test]
  fn conn_router_test() {
    let res = conn!().res;
    assert_eq!(res.status, "200 Ok");
    assert_eq!(res.file, "text/plain");
    assert!(res.body.contains("1,"));
    assert!(res.header.contains("<!DOCTYPE"));
    assert!(res.footer.contains("<script"));
  }

  #[test]
  fn conn_test() {
    let conn = conn!("new");
    assert_eq!(conn.req, Req::new());
    assert_eq!(conn.res, Res::new(Arc::new(Config::new())));
    assert_eq!(conn.req.raw, Raw::new());
  }
}
