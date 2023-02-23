#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use oxipng::{optimize, Headers, InFile, Options, OutFile};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str;
use svgcleaner::cleaner;

mod png;
mod svg;

#[napi(js_name = "pngCompress")]
pub fn optimize_png(path: String, out_path: String) -> HashMap<&'static str, f64> {
  let mut res = HashMap::new();
  let before_size = png::get_file_size(&path);

  res.insert("before_size", before_size);

  let input = InFile::Path(PathBuf::from(&path));
  let output = OutFile::Path(Some(PathBuf::from(&out_path)));
  let mut opts = Options::from_preset(4);
  opts.strip = Headers::All;

  match optimize(&input, &output, &opts) {
    Ok(_) => {
      let after_size = png::get_file_size(&out_path);
      res.insert("after_size", after_size);
      res
    }
    Err(_) => {
      println!("png compress error!");
      res
    }
  }
}

#[napi(js_name = "svgCompress")]
pub fn optimize_svg(path: String, out_path: String) -> HashMap<&'static str, f64> {
  let parse_opt = svg::gen_parse_options();
  let write_opt = svg::gen_write_options();
  let clean_opt = svg::gen_clean_options();

  // Load data.
  let raw = cleaner::load_file(&path).unwrap();
  let input_size = raw.len();
  let mut buf = raw.into_bytes();
  let mut prev_size: f64 = 0.0;
  let mut res: HashMap<&str, f64> = HashMap::new();

  res.insert("before_size", prev_size);
  res.insert("after_size", prev_size); // 初始化返回结果

  let on_err = || {
    println!("svg compress error!");
    std::process::exit(0);
  };

  loop {
    let mut doc = match cleaner::parse_data(str::from_utf8(&buf).unwrap(), &parse_opt) {
      Ok(d) => d,
      Err(_) => {
        on_err();
        return res;
      }
    };

    // Clean document.
    match cleaner::clean_doc(&mut doc, &clean_opt, &write_opt) {
      Ok(_) => {}
      Err(_) => {
        on_err();
        break;
      }
    }

    buf.clear();

    // Write buffer.
    cleaner::write_buffer(&doc, &write_opt, &mut buf);
    // If size is unchanged - exit from the loop.
    if prev_size == buf.len() as f64 {
      break;
    }

    prev_size = buf.len() as f64;
  }

  if buf.len() > input_size {
    return res;
  }

  match cleaner::save_file(&buf[..], &out_path) {
    Ok(_) => {
      res.insert("after_size", buf.len() as f64);
      res
    }
    Err(_) => res,
  }
}
