use size::Size;
use std::{
  fs::{self, OpenOptions},
  io::{BufRead, BufReader, Read},
  path::{Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct File {
  pub _path: Option<PathBuf>,
  pub _size: Size,
  pub content: String,
  pub line_count: usize,
  pub num_offset: usize,
}

impl File {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    // TODO: write to /tmp when path is None
    let path = path.as_ref();

    let _size = Size::from_bytes(fs::metadata(path).unwrap().len());
    let file = OpenOptions::new().write(true).read(true).create(false).open(path).unwrap();
    let line_count = BufReader::new(fs::File::open(path).unwrap()).lines().count();
    let num_offset = line_count.to_string().chars().count();

    let mut buf_read = BufReader::new(&file);
    let mut content = String::new();
    buf_read.read_to_string(&mut content).unwrap();

    Self {
      _path: Some(path.to_path_buf()),
      _size,
      content,
      line_count,
      num_offset,
    }
  }
}
