pub mod command_parser;
pub mod output_result;
mod utils;
pub mod service{
  use std::{io::{self, BufReader, BufRead, BufWriter}, path::{Path, PathBuf}, error::Error, fs::{File, self}};
  use crate::{command_parser::Args, output_result::OutputResult, utils::{compare_time, write_output_to_file}};
  
  pub fn find_error_file(args: Args)->Result<(), Box<dyn Error>>{
    if !(args.get_path().is_dir() || args.get_path().is_file()) {
      return Err(io::Error::new(io::ErrorKind::NotFound, "Path is not valid").into());
    }
    let path = args.get_path();
    let mut output_results = Vec::<OutputResult>::new();
    handle_file_recursive(&path,&args, &mut output_results,&process_file);
    if output_results.len() <= 0 {
      println!("Not Found the keywords in the files");
      return Ok(())
    }
    let file_output = File::options().create(true).write(true).open(args.get_output_file())?;
    let mut file_buffer = BufWriter::new(file_output);
    for out_result in output_results{
      write_output_to_file(&mut file_buffer, &out_result)?;
    }
    Ok(())
  }
  pub fn handle_file_recursive<F>(path: &PathBuf, config: &Args, output_result: &mut Vec<OutputResult>, f:&F)
  where
    F: Fn(&PathBuf,&Args,&mut Vec<OutputResult>)
  {
    if path.is_file(){
        f(path, config, output_result);
    }else if path.is_dir() {
      if let Ok(dir_read) = fs::read_dir(path) {
          for dir_result in dir_read.into_iter(){
            if let Ok(dir_entry) = dir_result {
              let path_dir = Path::join(path, dir_entry.path());
              handle_file_recursive(&path_dir, config, output_result, f)
            }
          }
      }
    }
  }
  pub fn process_file(path: &PathBuf, config: &Args, output_results:&mut Vec<OutputResult>){
    if let Ok(file) = File::open(path) {
      if let Ok(meta_data) = &file.metadata(){
        let modified = meta_data.modified().unwrap();
        if config.get_previous_days() == 0|| compare_time(&modified, config.get_from_date()) {
          let buffer_reader = BufReader::new(file);
          let mut line_iter = buffer_reader.lines().into_iter();
          let mut line_number:u32 = 0;
          let mut out_result = OutputResult::new();
          out_result.set_name(path.to_string_lossy().to_string());
          loop{
            if let Some(Ok(line)) = line_iter.next() {
              line_number += 1;
              if let Some(regex) = config.get_regex(){
                if regex.is_match(&line){
                  println!("Found: {} in line {}", path.to_string_lossy(),line_number);
                  out_result.add_line_with_args(line_number, line);
                  for _ in 0..(config.get_number_of_lines()-1){
                    if let Some(Ok(next_line)) = line_iter.next(){
                      line_number += 1;
                      out_result.add_line_with_args(line_number, next_line);
                    }
                  }
                }
              }
            }
            else{
              break;
            }
          }
          if out_result.get_lines().len() > 0 {
              output_results.push(out_result);
          }
        }
      }
    }
  }
}