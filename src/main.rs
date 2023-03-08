
use std::{env, process};
use search_content::{get_config, read_file, search};

#[warn(unused_assignments)]
// #[derive(Debug)]
// struct Config {
//     query: String,
//     file_path: String,
// }

fn main() {
    
    let mut args  = Vec::new();
    args = env::args().collect();

    

    if let Err(e) = get_config(&args) {
        println!("{e}");
        process::exit(1);
    } 
    if let Ok(config) = get_config(&args){
        let content = read_file(&config.file_path);
        let result = search(&config.query, &content);
        for i in result.iter() {
            println!("{i} \n")
        }
    }
        // println!("{:?}", config)
    

//     let config = get_config(&args);

// println!("{:?}", config)

    // let query = &args[1];
    // let file_path = &args[2];

    // println!("query: {query}");
    // println!("file path: {file_path}");

    // for a in args {
    //     println!("{a}")
    // }
}

// fn get_config (args: &[String]) -> Result<Config, &'static str> {
//     if args.len() < 3 {
//         return Err("not enough args");
//         // panic!("not enough args")
//         // process::exit(1);
//     }
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     Ok(Config { query, file_path,})
// }
// fn search(query: &String, content: &String) -> Vec<String> {
    
//     let mut result = Vec::new();
//     for line in content.lines() {
//         if line.contains(query) {
//             result.push(line.to_string())
//         }
//     }
//     result
// }
// fn read_file(file_path: &String) -> String{
//     let content = fs::read_to_string(file_path).expect("read file err");
//     content
// }
