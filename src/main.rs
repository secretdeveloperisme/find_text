use find_string_in_files::{command_parser::Args, service::find_error_file};


fn main() {
  match Args::build() {
      Ok(args)=>{
        if let Err(err) = find_error_file(args){
          println!("An error occurs: {}", err.to_string())
        }else {
          println!("Process finished successfully!");
        }
      },
      Err(err)=>{
        println!("An error occurs: {}", err.to_string())
      }
  }
  
}
