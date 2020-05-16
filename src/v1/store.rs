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

struct Mas {
  word: String,
  sentence: String,
}

fn process(hash: &mut HashMap<i32, Mas>, data: Vec<(i32, String, String)>) {
  for (rank, word, sentence) in data {
    hash.insert(rank, Mas { word, sentence });
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::panic;
  fn content_data(name: &str) -> Vec<(i32, String, String)> {
    match name {
      "english" => vec![
        (1, "hello".to_owned(), "using hello".to_owned()),
        (2, "welcome".to_owned(), "using welcome".to_owned()),
      ],
      "espanol" => vec![
        (1, "hola".to_owned(), "usando hola".to_owned()),
        (2, "bienvenidos".to_owned(), "usando bienbenidos".to_owned()),
      ],
      _ => panic!("data doesn't exit"),
    }
  }

  fn languages_data() -> Vec<String> {
    vec!["english".to_string(), "espanol".to_string()]
  }

  #[test]
  fn process_okay() {
    let mut store = HashMap::new();
    for name in languages_data() {
      let mut lang_box = HashMap::new();

      let data = content_data(&name);
      process(&mut lang_box, data);
      store.insert(name, lang_box);
    }

    for name in languages_data() {
      for (rank, word, sentence) in content_data(&name) {
        let lang_box = store.get(&name);
        let mas = &lang_box.unwrap().get(&rank).unwrap();
        assert_eq!(mas.word, word);
        assert_eq!(mas.sentence, sentence);
      }
    }
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
