// Ask user input: Name, Date and Story.  
// Publish? Y/N  If yes, write all info down on Name.txt.
// if no exit.

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut inputName = String::new();
    let mut inputDate = String::new();
    let mut inputStory = String::new();
    let mut all = [inputName, inputDate, inputStory].concat();

    let mut publish = false;

    loop // while true
    {
        if publish != true

        {
            println!("Please enter your name.");
            io::stdin()
            .read_line(&mut inputName)
            .expect("failed to read from stdin");
        }{
            println!("Please enter your Date of Birth.");
            io::stdin()
            .read_line(&mut inputDate)
            .expect("failed to read from stdin");
        }{
            println!("Please enter your story");
            io::stdin()
            .read_line(&mut inputStory)
            .expect("failed to read from stdin");
        }{
            println!("Would you like to publish? (write to File)");
            io::stdin()
            .read_line(&mut publish == true) ///notsure how to fix this yet.  How to toggle bollen?
            .expect("failed to read from stdin");
            
        }
        if publish == true { break; } //Break out of loop. 2 to the power of 10 = 1024
        println!("Thank you! You Published your document {}", input_text1); 
        writefile();
    }
    Err(_) =>  println!("This didn't work. :( ")
}


 //write to file.    Not working??????????????
fn writefile() -> std::io::Result<()> {
    let mut all ="{}";                          
    let mut pos = 0;
    let mut buffer = File::create("{}.txt",inputDate)?;

    while pos < all.len() {
        let bytes_written = buffer.write(&all[pos..])?;
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
