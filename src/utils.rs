use std::{time::SystemTime, fs::File, io::Write};

use chrono::{DateTime, Utc};

pub trait Serializable : Sized{
  fn serialize(&self)->String;
}

pub fn compare_time(sys_time: &SystemTime, chrono_time: &DateTime<Utc>)->bool{
  let target : DateTime<Utc> =sys_time.clone().into(); 
  return if target > chrono_time.to_owned() {true} else{false};
}

pub fn write_output_to_file<T>(file : &mut File, target: &T)->Result<(), std::io::Error>
where T: Serializable{
  let string = target.serialize();
  let buffer = string.as_bytes();
  if let Ok(_) = file.write(buffer){
    return Ok(());
  }
  return Err(std::io::Error::new(std::io::ErrorKind::WriteZero, "Cannot write to file "));
}