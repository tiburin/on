/// this feature should be dispatch after starting the application
/// it should have all the available languages in a hashmap including their content in a inner hashmap

const MAX: usize = 5000;
use std::collections::HashMap;
use std::{io, path::Path};

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

fn start(languages: Vec<String>, data: Vec<(i32, String, String)>) {
  // let data_box = HashMap::new();

  let mut table: HashMap<i32, String> = HashMap::with_capacity(MAX);
  for (rank, word, sentence) in parse_content("..".to_owned()) {
    //TODO
    // insert data into the hashmap
  }

  for language in languages {
    // TODO insert into hashmap
  }
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::panic;

  #[test]
  fn read_languages_okay() {
    let result = read_languages();
    assert!(result.unwrap().len() >= 2);
  }
  #[test]
  fn start_test() {
    // let content = fs::read_to_string("public/spoken/english.on").unwrap();
    // start(content)
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
