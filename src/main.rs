#![allow(unused_assignments)]
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn get_title() -> String {
 let name: &str = env!("CARGO_PKG_NAME");
 let version: &str = env!("CARGO_PKG_VERSION");
 let desc: &str = env!("CARGO_PKG_DESCRIPTION");

 let title: String = format!("{} (v{}), {}", name, version, desc);

 return title;
}

fn parse_markdown_file(_filename: &str) {
  print_short_banner();
  println!("[ INFO ] Trying to parse {}...", _filename);

  let filepath = Path::new(_filename);
  let file = File::open(&filepath).expect("[ ERROR ] Failed to open file");

  let mut ptag: bool = false;
  let mut htag: bool = false;

  let mut tokens: Vec<String> = Vec::new();

  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line_contents: String = line.unwrap();
    let mut output_line: String = String::new();



    let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

    match first_char.pop() {
      Some('#') => {
        if ptag {
          ptag = false;
          output_line.push_str("</p>\n");
        }
        if htag {
          htag = false;
          output_line.push_str("</h1>\n");
        }

        htag = true;
        output_line.push_str("<h1>");
        output_line.push_str(&line_contents[2..]);
      },
      _ => {
        if !ptag {
          ptag = true;
          output_line.push_str("<p>");
        }
        output_line.push_str(&line_contents);
      }
    }

    if ptag {
      ptag = false;
      output_line.push_str("</p>\n");
    }

    if htag {
      htag = false;
      output_line.push_str("</h1>\n");
    }

    if output_line != "<p></p>\n" {
      tokens.push(output_line);
    }
  }

  let mut output_filename: String = String::from(&_filename[.._filename.len()-3]);
  output_filename.push_str(".html");

  let mut output_file = File::create(output_filename).expect("[ ERROR ] Could not create output file");

  for line in &tokens {
    output_file.write_all(line.as_bytes()).expect("[ ERROR ] Could not write to output file");

  }

  println!("[ INFO ] Successfully parsed Markdown!");

}

fn print_short_banner() {
  println!("{}", get_title());
}

fn print_long_banner() {
  let author: &str = env!("CARGO_PKG_AUTHORS");
  let homepage: &str = env!("CARGO_PKG_HOMEPAGE");

  let banner: String = format!("Written By: {} \nHomepage: {} \nUsage: lilmd <somefile>.md", author, homepage);

  print_short_banner();
  println!("{}", banner)
}

fn usage() {
  print_long_banner();
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  match args.len() {
    1 => {
            println!("[ ERROR ] You forgot to specify the markdown file to parse!");
            usage();
        },
    2 => parse_markdown_file(&args[1]),
    _ => usage()
  }
}