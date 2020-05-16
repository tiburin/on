/// this feature should be dispatch after starting the application
/// it should have all the available languages in a hashmap including their content in a inner hashmap

const MAX: usize = 5000;
use std::collections::HashMap;
use std::{fs, io, path::Path};

fn parse_content(content: String) -> Vec<(i32, String, String)> {
  content
    .split("\n")
    .map(|line| {
      let list: Vec<&str> = line.split(",").collect();
      if list.len() != 3 {
        let msg = "line should be separated with 2 commas";
        panic!("{}  NOW HAS {}", msg, list.len() - 1)
      }
      let rank = list[0].trim().parse().expect("rank should be an integer");
      let word = match list[1].trim() {
        "" => "w",
        word => word,
      };
      let sentence = match list[2].trim() {
        "" => "... ... ...",
        sentence => sentence,
      };
      if rank as usize > MAX {
        panic!("rank TOO small or TOO BIG limit is {}", MAX)
      }
      (rank, word.to_owned(), sentence.to_owned())
    })
    .collect()
}
fn parse_languages(content: String) -> Vec<String> {
  let list: Vec<String> = content
    .split("\n")
    .filter(|n| n != &"")
    .map(|n| n.trim().to_owned())
    .collect();

  if list.len() < 2 {
    panic!("languages need to be more than 2 CURRENT {}", list.len())
  }
  list
}

pub fn read_languages() -> Result<Vec<String>, io::Error> {
  let path = Path::new("public/spoken");

  let list = path.read_dir()?.fold(Vec::new(), |mut acc, files| {
    if let Ok(file) = files {
      if let Some(os_name) = file.path().file_stem() {
        os_name.to_str().map(|name| {
          acc.push(name.trim().to_owned());
        });
      }
    };
    acc
  });
  Ok(list)
}
pub fn read_content(name: &str) -> Result<String, io::Error> {
  fs::read_to_string(format!("public/spoken/{}.on", name))
}

fn start() {
  for name in read_languages().unwrap() {
    let content = read_content(&name).unwrap();
    let n = parse_content(content);
    // ...
  }
}

fn process(
  hash: &mut HashMap<String, (i32, String, String)>,
  name: String,
  data: (i32, String, String),
) {
  hash.insert(name, data);
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::panic;

  #[test]
  fn process_okay() {
    let word = "learn".to_owned();
    let sentence = "we are going to learn a new language".to_owned();
    let name = "english".to_owned();
    let data = (1, word, sentence);
    let mut container = HashMap::new();
    process(&mut container, name, data);
    eprintln!("=--=> {:?}", container);
  }

  #[test]
  fn start_test() {
    start();
  }

  #[test]
  pub fn read_content_errors() {
    assert!(read_content("abc").is_err());
    assert!(read_content("").is_err());
  }
  #[test]
  pub fn read_content_okay() {
    assert!(read_content("english").is_ok());
    assert!(read_content("espanol").is_ok());
  }

  #[test]
  fn read_languages_okay() {
    let result = read_languages();
    assert!(result.unwrap().len() >= 2);
  }

  #[test]
  fn parse_languages_errors() {
    let result = panic::catch_unwind(|| parse_languages(format!("a\n")));
    assert!(result.is_err());

    let result = panic::catch_unwind(|| parse_languages(format!("")));
    assert!(result.is_err());
  }
  #[test]
  fn parse_languages_okay() {
    let result = parse_languages(format!("a\nb\nb"));
    assert_eq!(result, ["a", "b", "b"]);

    let stage2 = "\nenglish\nespanol\nportugues";
    let result = parse_languages(format!("a\nb\nz{}", stage2));
    let response = ["a", "b", "z", "english", "espanol", "portugues"];
    assert_eq!(result, response);
  }

  #[test]
  fn parse_content_errors() {
    let result = panic::catch_unwind(|| parse_content(format!("{} , w , s", MAX + 1)));
    assert!(result.is_err());

    let result = panic::catch_unwind(|| parse_content(format!("{} , w , s", -1)));
    assert!(result.is_err());

    let result = panic::catch_unwind(|| parse_content(format!("n , w , s",)));
    assert!(result.is_err());
    let result = panic::catch_unwind(|| parse_content(format!("w , s")));
    assert!(result.is_err());
  }
  #[test]
  fn parse_content_okay() {
    let result = parse_content(format!("5, w ,... "));
    assert_eq!(result, [(5, "w".to_owned(), "...".to_owned())]);

    let result = parse_content(format!("5, w ,... \n 99, w , ..."));
    let one = (5, "w".to_owned(), "...".to_owned());
    let two = (99, "w".to_owned(), "...".to_owned());
    assert_eq!(result, [one, two]);
  }
}
