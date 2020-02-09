use super::{Method, MiError, Req};

pub struct Parser<'a, 'b> {
  pub req: &'a mut Req<'b>,
}

impl<'a, 'b> Parser<'a, 'b> {
  pub fn new(req: &'a mut Req<'b>) -> Self {
    Self { req }
  }

  pub fn parse(&mut self, buffer: &[u8]) -> Result<&Req, MiError> {
    let data = std::str::from_utf8(&buffer[..]).unwrap();
    let mut request = data.split("\r\n\r\n").map(|n| n.to_owned());

    self.req.raw.header = request.next();
    self.req.raw.body = request.next();
    self.header()?;
    Ok(&self.req)
  }

  fn header(&mut self) -> Result<&Req, MiError> {
    let header = self.req.raw.header.clone().unwrap();
    let lines: Vec<&str> = header.split("\n").collect();
    if lines.len() > 1 {
      let line: Vec<&str> = lines[0].split_whitespace().collect();
      match line.len() {
        3 => {
          self.method(line[0]);
          self.path(line[1])?;
          Ok(&self.req)
        }
        _ => Err(MiError::new("request is invalid")),
      }
    } else {
      Ok(&self.req)
    }
  }

  fn method(&mut self, method: &str) {
    if method == "GET" {
      self.req.method = Method::GET;
    } else if method == "POST" {
      self.req.method = Method::POST;
    } else if method == "DELETE" {
      self.req.method = Method::DELETE;
    } else if method == "PUT" {
      self.req.method = Method::PUT;
    }
  }
  fn path(&mut self, path: &str) -> Result<&Req, MiError> {
    self.req.path = Some(path.to_string());
    let mut list: Vec<&str> = path.split("/").filter(|&n| n != "").collect();
    if list.len() > 0 {
      let end = list.pop().unwrap();
      if end.contains(".") {
        let files: Vec<&str> = end.split(".").collect();
        list.push(files[0]);
        match files[1] {
          "css" => self.req.file = Some("css"),
          "js" => self.req.file = Some("js"),
          "off" => self.req.file = Some("off"),
          _ => (),
        }
      }
      if let Ok(number) = end.parse::<i32>() {
        self.req.rank = Some(number.to_string());
      }
      self.req.path_list = list.iter().map(|v| v.to_string()).collect();
    }
    Ok(&self.req)
  }
}

#[cfg(test)]
mod tests {
  use crate::v1::{Parser, Req};
  use crate::*;

  #[test]
  fn path_test() {
    let mut conn = conn!("new");
    let mut parser = Parser::new(&mut conn.req);
    let new = Req::new();
    let req = parser.path("/").unwrap();
    assert_eq!(req.path.as_ref().unwrap(), "/");
    assert_eq!(req.path_list, new.path_list);
    assert_eq!(req.file, new.file);
    assert_eq!(req.rank, new.rank);

    let req = parser.path("/dev/content/app.js").unwrap();
    assert_eq!(req.path.as_ref().unwrap(), "/dev/content/app.js");
    assert_eq!(req.path_list, ["dev", "content", "app"]);
    assert_eq!(req.file.unwrap(), "js");

    let req = parser.path("/english/spoken/55").unwrap();
    assert_eq!(req.rank.as_ref().unwrap(), "55");
  }
}
