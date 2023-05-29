use chrono::{DateTime, Local};
pub struct Line{
  number : i32,
  content: String
}
pub struct OutputStructure{
  file_name: String,
  lines: Vec<Line>,
  modified_date: DateTime<Local>
}

impl OutputStructure {
    pub fn new()->OutputStructure{
      OutputStructure{
        file_name: String::new(),
        lines: Vec::<Line>::new(),
        modified_date: DateTime::<Local>::default()
      }
    }
}