use std::{fs};
#[warn(unused_assignments)]
// #[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn get_config (args: &[String]) -> Result<Config, &'static str> {
  if args.len() < 3 {
      return Err("not enough args");
      // panic!("not enough args")
      // process::exit(1);
  }
  let query = args[1].clone();
  let file_path = args[2].clone();
  Ok(Config { query, file_path,})
}
pub fn search(query: &String, content: &String) -> Vec<String> {
  
  let mut result = Vec::new();
  for line in content.lines() {
      if line.contains(query) {
          result.push(line.to_string())
      }
  }
  result
}
pub fn read_file(file_path: &String) -> String{
  let content = fs::read_to_string(file_path).expect("read file err");
  content
}