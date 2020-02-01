// Ask user input: Name, Date & Story.  
// ASk user if they wish to Publish? Y/N  If yes, write all info down on Name.txt.
// if no, exit.

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut input_name = String::new();
    let mut input_date = String::new();
    let mut input_story = String::new();
    let mut filename = String::new();

    match ask_console_input("Please enter your name.".to_string()){
        Ok(n) => {
            input_name = n;
        }
        Err(error) => {
            println!("error: {}", error);
        },
    }

    match ask_console_input("Please enter your Date of Birth.".to_string()){
            Ok(n) => {
                input_date = n;
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }

    match ask_console_input("Please enter your story".to_string()){
            Ok(n) => {
                input_story = n;
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }

        match ask_console_input("Please name your file.".to_string()){
            Ok(n) => {
                filename = n;
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }

    match ask_console_input("Would you like to publish? Please enter Y or N. (write to File)".to_string()){
            Ok(response) => {
                println!("Response {}", response);
                let strresponse = &response[..].to_string();

                match strresponse.trim(){
                    "y" => {
                        println!("YES PLZ");
                        let all = [input_name, input_date, input_story].concat();
                        match writefile(filename, all){
                            Ok(_) => println!("Write file sucess!!"),
                            Err(n) => {
                                println!("Write file sad. :(");
                                println!("{}",n)
                            },
                        }
                    },
                    "n" => println!("Heck NO!"),
                    _ => println!("??????"),
                }
            }
            Err(error) => {
                println!("error: {}", error);
            },
        }
    }

fn ask_console_input(prompt:String) -> Result<String,String> {
    let mut input = String::new();
    println!("{}",prompt);
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You entered: {}", input);
            Ok(input)
        }
        Err(error) => {
            println!("error: {}", error);
            Err(error.to_string())
        },
    }
}

//  write to file.  
fn writefile(input_name: String, data: String) -> std::io::Result<()> {                          
    let test = data.as_bytes();
    let mut buffer = File::create(input_name.trim().to_owned() + ".txt")?;
    buffer.write_all(test)?;
    Ok(())
}
