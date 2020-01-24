// Ask user input: Name, Date & Story.  
// ASk user if they wish to Publish? Y/N  If yes, write all info down on Name.txt.
// if no, exit.

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut inputName = String::new();
    let mut inputDate = String::new();
    let mut inputStory = String::new();
    let mut all = [inputName, inputDate, inputStory].concat();

    let mut publish = false;

    match askConsoleInput("Please enter your name.".to_string()){
        Ok(n) => {
            inputName = n;
        }
        Err(error) => {
            println!("error: {}", error);
        },
    }

    match askConsoleInput("Please enter your Date of Birth.".to_string()){
            Ok(n) => {
                inputDate = n;
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }

    match askConsoleInput("Please enter your story".to_string()){
            Ok(n) => {
                inputStory = n;
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }

    match askConsoleInput("Would you like to publish? (write to File)".to_string()){

            Ok(n) => {
                println!("Response {}", n);
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }

        //println!("Thank you! You Published your document {}", inputName); 
        //writefile(inputName,all); //Pass to function!
    
}


fn askConsoleInput(mut Prompt:String) -> Result<String,String> {
    let mut input = String::new();
    println!("{}",Prompt);
    match io::stdin().read_line(&mut Prompt) {
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





 //write to file.    Not working??????????????
// fn writefile(inputName: String, all: String) -> std::io::Result<()> {
    //let mut all ="{}";                          
//     let mut pos = 0;
//     let mut buffer = File::create(inputName + ".txt")?;

//     while pos < all.len() {
//         let bytes_written = buffer.write(&all[pos..])?;
//         pos += bytes_written;
//     }
//     Ok(())
// }



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
