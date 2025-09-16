use std::env;
use std::fmt::{Debug, };
use crate::util::exit::*;
use crate::util::stc_conversion::*;

#[derive(Debug, PartialEq)]
pub struct ArgumentList{
    pub convert_to_stc: Option<String>,                         // --convert-to-stc
}

impl ArgumentList{
    pub fn new() -> ArgumentList{
        ArgumentList{ convert_to_stc: None }
    }

}


mod controller;
mod util;

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    // Generate a reasonable argument list
    let mut args = get_arguments_from_list(cli_args);

    if let Some(string) = args.convert_to_stc {
        let stc_array = ascii_string_to_stc(string.clone());

        println!("{}", stc_array.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
        return;
    }
}

fn get_arguments_from_list(args: Vec<String>) -> ArgumentList {
    // Remove the first argument as it's just the name of the bin
    let mut args = args;
    args.remove(0);

    // Make space for the result
    let mut result = ArgumentList::new();

    // Sort the arguments
    // The first out-of-context (not belonging or being connected to a flag (-)) is the input file
    let mut current_flag: Option<String> = None;

    for arg in args {
        if let Some(arg_first_char) = arg.chars().nth(0){
            // Check if this argument is necessary for the last flag
            if let Some(flag) = current_flag.clone(){
                let value = arg.clone();

                match flag.as_str() {
                    "--convert-to-stc" => {
                        result.convert_to_stc = Some(value);
                    }

                    _=>{
                        exit(format!("Unknown flag {}.", flag), ExitCode::BadArgument);
                    }
                }

                current_flag = None;
                continue;
            }

            // Check if the argument is a flag
            if arg_first_char == '-' {
                // This is a flag
                // Therefore, look if the next argument also needs to be checked or the argument can be added right away

                match arg.as_str() {
                    _=>{
                        current_flag = Some(arg);
                    }
                }
                continue;
            }
        }
    }

    result
}