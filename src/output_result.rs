use std::time::SystemTime;

use chrono::{DateTime, Utc};

use crate::utils::Serializable;

#[derive(Debug)]
pub struct Line{
  number: u32,
  content: String 
}
#[derive(Debug)]
pub struct OutputResult{
  name: String, 
  lines: Vec<Line>,
  modified_date: DateTime<Utc>
}

impl Line {
  pub fn new () -> Line{
    Line{
      number: 0,
      content: String::new()
    }
  }
  pub fn new_with_args(number: u32,content: String)->Line{
    Line { number, content }
  }
  pub fn set_number(&mut self, number: u32){
    self.number = number;
  }
  pub fn set_content(&mut self, content: String){
    self.content = content;
  }

  pub fn get_number(&self)->u32{
    self.number
  }
  pub fn get_content(&self)->&String{
    &self.content
  }
}

impl Serializable for Line {
   fn serialize(&self)->String{
      let mut result = String::new();
      result += format!("{}:{}", self.get_number(), self.get_content()).as_str();
      return result;
   }
}

impl OutputResult {
  pub fn new() -> OutputResult{
    OutputResult {
     name: String::new(),
     lines: vec![],
     modified_date: Utc::now()
    } 
  }
  pub fn set_name(&mut self, name: String){
    self.name = name;
  }
  pub fn add_lines(&mut self, lines: Line){
    self.lines.push(lines);
  }
  pub fn add_line_with_args(&mut self ,number:u32, content:String){
    self.add_lines(Line::new_with_args(number, content));
  }
  pub fn set_modified_date(&mut self, modified_date: DateTime<Utc>){
    self.modified_date = modified_date;
  }
  pub fn set_modified_date_by_system_time(&mut self, sys_time:SystemTime){
    self.modified_date = sys_time.into();
  }
  pub fn get_lines(&self)->&Vec<Line>{
    &self.lines
  }
  pub fn get_name(&self)->&String{
    &self.name
  }
  pub fn get_modified(&self)->&DateTime<Utc>{
    &self.modified_date
  }
  
}
impl Serializable for OutputResult {
    fn serialize(&self)->String {
      let mut result = String::new();
      result += format!("FileName: {}", self.get_name()).as_str();
      result += "\n";
      result += format!("modified: {}", self.get_modified()).as_str();
      result += "\n";
      for line in self.get_lines(){
        result += line.serialize().as_str();
        result += "\n";
      }
      return result;
    }
}