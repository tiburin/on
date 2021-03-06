/// this feature should be dispatch after starting the application
/// it should have all the available languages in a hashmap including their content in a inner hashmap

const MAX: usize = 5000;
use std::collections::HashMap;
use std::{fs, io, path::Path};
type StoreType = HashMap<String, HashMap<usize, Mas>>;

fn parse_content(content: String) -> Vec<(usize, String, String)> {
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
  if list.len() < 2 {
    panic!("languages need to be more than 2 CURRENT {}", list.len())
  }
  Ok(list)
}
pub fn read_content(name: &str) -> Result<String, io::Error> {
  fs::read_to_string(format!("public/spoken/{}.on", name))
}

#[derive(Debug, Clone)]
pub struct Mas {
  pub word: String,
  pub sentence: String,
}

fn process(hash: &mut HashMap<usize, Mas>, data: Vec<(usize, String, String)>) {
  for (rank, word, sentence) in data {
    hash.insert(rank, Mas { word, sentence });
  }
}

pub fn start(store: &mut StoreType) -> StoreType {
  for name in read_languages().unwrap() {
    let content = read_content(&name).unwrap();
    let mut lang_box = HashMap::new();
    let data = parse_content(content);
    process(&mut lang_box, data);
    store.insert(name, lang_box);
  }
  store.clone()
}
#[derive(Debug)]
pub struct Storage {
  pub store: StoreType,
  pub languages: Vec<String>,
}
impl Storage {
  pub fn new() -> Self {
    Self {
      store: start(&mut HashMap::new()),
      languages: read_languages().unwrap(),
    }
  }
  pub fn get_line(hash_map: Option<&HashMap<usize, Mas>>, rank: usize) -> Option<String> {
    if let Some(lang_box) = hash_map {
      match lang_box.get(&rank) {
        Some(mas) => {
          let data = format!("{},{},{}", rank, mas.word, mas.sentence);
          Some(data)
        }
        _ => None,
      }
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::panic;

  fn languages_data() -> Vec<String> {
    vec!["english".to_string(), "espanol".to_string()]
  }

  fn content_data(name: &str) -> Vec<(usize, String, String)> {
    match name {
      "english" => vec![
        (1, "hello".to_owned(), "using hello".to_owned()),
        (2, "welcome".to_owned(), "using welcome".to_owned()),
      ],
      "espanol" => vec![
        (1, "hola".to_owned(), "usando hola".to_owned()),
        (2, "bienvenidos".to_owned(), "usando bienvenidos".to_owned()),
      ],
      _ => panic!("data doesn't exit"),
    }
  }

  fn fake_store_started(store: &mut StoreType) {
    for name in languages_data() {
      let mut lang_box = HashMap::new();

      let data = content_data(&name);
      process(&mut lang_box, data);
      store.insert(name, lang_box);
    }
  }

  fn get_language<'a>(store: &'a StoreType, name: &str) -> Option<&'a HashMap<usize, Mas>> {
    store.get(name)
  }
  fn get_content(lang_box: &HashMap<usize, Mas>, rank: usize) -> Option<&Mas> {
    lang_box.get(&rank)
  }

  #[test]
  fn app_data_exist_errors() {
    let mut store = HashMap::new();
    fake_store_started(&mut store);
    let result = Storage::get_line(store.get("english"), MAX + 1);
    assert!(result.is_none());

    let result = Storage::get_line(store.get("english"), 8);
    assert!(result.is_none());
  }
  #[test]
  fn app_data_exist_okay() {
    let mut store = HashMap::new();
    fake_store_started(&mut store);
    let result = Storage::get_line(store.get("english"), 2);
    assert_eq!(result, Some("2,welcome,using welcome".to_owned()));
    let result = Storage::get_line(store.get("english"), 1);
    assert_eq!(result, Some("1,hello,using hello".to_owned()));

    let result = Storage::get_line(store.get("espanol"), 2);
    assert_eq!(result, Some("2,bienvenidos,usando bienvenidos".to_owned()));
    let result = Storage::get_line(store.get("espanol"), 1);
    assert_eq!(result, Some("1,hola,usando hola".to_owned()));
  }

  #[test]
  fn user_getting_data_errors() {
    let mut store = HashMap::new();
    fake_store_started(&mut store);

    let noexit = get_language(&store, "noexit");
    assert!(noexit.is_none());
    let no_lang = get_language(&store, "noexit");
    assert!(no_lang.is_none());

    let espanol = get_language(&store, "espanol").unwrap();
    let mas = get_content(espanol, MAX * MAX);
    assert!(mas.is_none());

    let english = get_language(&store, "english").unwrap();
    let mas = get_content(english, MAX + MAX);
    assert!(mas.is_none());
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
    let mut store = HashMap::new();
    start(&mut store);
    for name in read_languages().unwrap() {
      let content = read_content(&name).unwrap();
      let data = parse_content(content);
      for (rank, word, sentence) in data {
        let mas = store.get(&name).unwrap().get(&rank).unwrap();
        assert_eq!(mas.word, word);
        assert_eq!(mas.sentence, sentence);
      }
    }
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
