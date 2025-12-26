use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize, Serialize)]
pub struct LR {
  pub left: f32,
  pub right: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditorConfig {
  // pub theme: Theme,
  pub num_pad: LR,
}

impl Default for EditorConfig {
  fn default() -> EditorConfig {
    let file = File::open("config/init.json").unwrap();
    let reader = BufReader::new(file);
    let config: EditorConfig = serde_json::from_reader(reader).unwrap();

    EditorConfig { ..config }
  }
}
