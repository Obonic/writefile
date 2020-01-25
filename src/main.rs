// Ask user input: Name, Date & Story.  
// ASk user if they wish to Publish? Y/N  If yes, write all info down on Name.txt.
// if no, exit.

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut input_name = String::new();
    let mut input_date = String::new();
    let mut input_note = String::new();
    let all = [input_name, input_date, input_note].concat();

    let mut publish = false;

    match askConsoleInput("Please enter your name.".to_string()){
        Ok(n) => {
            input_name = n;
        }
        Err(error) => {
            println!("error: {}", error);
        },
    }

    match askConsoleInput("Please enter your Date of Birth.".to_string()){
            Ok(n) => {
                input_date = n;
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }

    match askConsoleInput("Please enter your story".to_string()){
            Ok(n) => {
                input_note = n;
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }

    match askConsoleInput("Would you like to publish? (Write to file?)".to_string()){
            Ok(n) => {
                println!("Response {}", n);
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }
        //println!("Thank you! You Published your document {}", input_name); 
   // writefile(input_name,all); //Pass to function!
}

// //Maybe?? V^? donno. ¯\_(ツ)_/¯
// match  &input {
//     Y => println!("YES PLEASE!!!"),
//     N => println!("F*CKNO!"),
//     _ => println!("what?"),
// }



fn askConsoleInput(mut prompt:String) -> Result<String,String> {
    let input = String::new();
    println!("{}",prompt);
    match io::stdin().read_line(&mut prompt) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
            Ok(input)
        }
        Err(error) => {
            println!("error: {}", error);
            Err(error.to_string())
        },
    }
}

//write to file.   
//Not sure this works yet.
fn writefile(input_name: String, all: String) -> std::io::Result<()> {
    //let mut all ="{}";                          
    let mut pos = 0;
    let mut buffer = File::create(input_name + ".txt")?;

    while pos <all.len() {
        let bytes_written = buffer.write(&all.as_bytes()[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}







// //This works
// use std::io::prelude::*;
// use std::fs::File;

// fn main() -> std::io::Result<()> {
//     let data = b"Doug's text to be writen to file Doug.txt";

//     let mut pos = 0;
//     let mut buffer = File::create("Doug.txt")?;

//     while pos < data.len() {
//         let bytes_written = buffer.write(&data[pos..])?;
//         pos += bytes_written;
//     }
//     Ok(())
// }
