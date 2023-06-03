use std::{path::PathBuf, fs};
use chrono::{DateTime, Utc, Days};
use clap::Parser;
use regex::Regex;


pub const DEFAULT_OUT_FILE_NAME: &str = "output.out";
pub const DEFAULT_NUMBER_OF_LINES: u16 = 1;
pub const DEFAULT_NUMBER_OF_PREVIOUS_DAYS: u16 = 0;


#[derive(Parser,Debug)]
#[command(author = "hoanglinh", version="1.0.0", about, long_about = "Find path and content of file that matches by specifying the keywords")]
pub struct Args{

  #[arg(short = 'p', long = "path", value_name = "Path to input folder or file")]
  path: PathBuf,
  
  #[arg(short = 'k', long ="keywords", value_name = "List of keywords separated by [, ; |]")]
  keywords: String,

  #[arg(short, long, value_name = "output file")]
  output: Option<String>,

  #[arg(short = 'd', long = "previous-days", value_name = "Number of Previous days")]
  number_of_previous_days : Option<u16>,

  #[arg(short = 'l', long = "number-lines", value_name = "Number of Lines")]
  number_of_lines : Option<u16>,

  #[arg(short = 'i', long = "ignore-case", value_name = "Ignore case" )]
  ignore_case: bool,
  
  #[clap(skip)]
  keywords_list:Vec<String>,

  #[clap(skip)]
  regex: Option<Regex>,
  #[clap(skip)]
  from_date: DateTime<Utc>
}

impl Args{
  pub fn get_path(&self) ->&PathBuf{
    return &self.path; 
  }

  pub fn get_keyword(&self) -> &String{
    return &self.keywords;
  }
  pub fn get_output_file(&self) -> String{
    if let Some(output_file) = self.output.as_ref(){
      return output_file.to_owned();
    }
    std::env::current_dir().unwrap_or_default().join(DEFAULT_OUT_FILE_NAME).to_string_lossy().to_string()
  }
  pub fn get_keyword_list(&self)->&Vec<String>{
    return &self.keywords_list;
  }
  pub fn build_path(&mut self){
   
  }
  pub fn build_keywords(&mut self, keyword: String){
    self.keywords_list = keyword.split(&[';',',','|']).map(|value|{
      value.to_string()
    }).collect(); 
    
    let ignore_case_flag = if self.is_ignore_case(){"(?i)"}else{""};
    if let Ok(regex) = Regex::new(format!("{}{}",ignore_case_flag,self.keywords_list.join("|")).as_str()){
      self.regex = Some(regex);
    }else{
      self.regex = None;
    }

  }
  pub fn build_from_date(&mut self){
    let date_time = Utc::now();
    self.from_date = date_time.checked_sub_days(Days::new(self.get_previous_days() as u64)).unwrap_or_default();
  }
  pub fn get_previous_days(&self)->u16{
    return if self.number_of_previous_days.is_some(){
      self.number_of_previous_days.unwrap()
    }else{
      DEFAULT_NUMBER_OF_PREVIOUS_DAYS
    }
  }
  pub fn get_from_date(&self)->&DateTime<Utc>{
    &self.from_date
  }
  pub fn get_number_of_lines(&self)->u16{
    return if self.number_of_lines.is_some(){
      self.number_of_lines.unwrap()
    }else{
      DEFAULT_NUMBER_OF_LINES
    }
  }
  pub fn is_ignore_case(&self)->bool{
    self.ignore_case
  }

  pub fn get_regex(&self)->&Option<Regex>{
    &self.regex
  }
  

  pub fn check(&self)->Result<(), String>{
    if self.keywords.is_empty(){
      return Err(String::from("Keywords were not specified"));
    }
    if !(self.get_path().is_file() || self.get_path().is_dir()){
      return Err(String::from("Path is not valid"));
    }
    if self.get_keyword_list().len() <= 0 {
      return Err(String::from("Keywords are invalid"));
    }
    Ok(())
  }
  
  pub fn build() ->Result<Self,String> {
    let mut args = Self::parse();
    if args.path.is_relative(){
      args.path = fs::canonicalize(args.path.clone()).unwrap();
    }
    let string = args.get_keyword();
    args.build_keywords(string.clone());
    args.build_from_date();
    args.check()?;
    Ok(args)
  }
}