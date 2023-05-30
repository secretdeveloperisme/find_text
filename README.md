# find_text
Find path and content of file that matches by specifying the keywords
```bash
λ find_string_in_files.exe --help
Usage: find_string_in_files.exe [OPTIONS] --path <Path to input folder or file> --keywords <List of keywords separated by [, ; |]>

Options:
  -p, --path <Path to input folder or file>
  -k, --keywords <List of keywords separated by [, ; |]>
  -o, --output <output file>
  -d, --previous-days <Number of Previous days>
  -l, --number-lines <Number of Lines>
  -h, --help Print help (see a summary with '-h')
  -V, --version Print version
```

## Example 

```bash 
find_string_in_files.exe --path D:\target_folder\ --keywords "hoanglinh|hl"  --previous-days 1 --number-lines 2
```
> *The command will search on target folder with the key words **hoanglinh** or **hl**, the files must has modified date from **yesterday** to now and the line that match the keywords and next line will be write to the output file.*\
> ***Note**: if the output file argument was not provided, the result will write to the file with name output.out and it's location is current working directory*.

## Output file example 
```bash
FileName: D:\target\abc.txt
Modified: 2023-05-05/30/23 00:08:58
{5}:  use std::{io::{self, BufReader, BufRead}, path::{Path, PathBuf}, error::Error, fs::{File, self}};
{6}:  use crate::{command_parser::Args, output_result::OutputResult, utils::compare_time};
{9}:  pub fn find_error_file(args: Args)->Result<(), Box<dyn Error>>{
{10}:    if !(args.get_path().is_dir() || args.get_path().is_file()) {
{11}:      return Err(io::Error::new(io::ErrorKind::NotFound, "Path is not valid").into());
{12}:    }
{68}:              output_results_hoanglinh.push(out_result);
{69}:          }
{76}: hoanglinh #[test]
{77}:  fn test_call_call(){
```

