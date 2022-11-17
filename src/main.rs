use std::fs;
use std::io::{self, prelude::*};


fn main() -> std::io::Result<()> {

    /***********
     * WRITING *
     ***********/

    /* Simple write */
    fs::write("favorite_websites.txt", b"opensource.com")?;

    /* Write using file descriptor */
    let mut file = fs::File::create("favorite_websites.txt")?;
    file.write_all(b"opensource.com\n")?;

    /* Write using file descriptor and opening modes */

    let mut file = fs::OpenOptions::new()
                                        .append(true)
                                        .open("favorite_websites.txt")?;

    file.write_all(b"sourceforge.net\n")?;

    /***********
     * READING *
     ***********/

    /* Simple read */
    let websites = fs::read_to_string("favorite_websites.txt")?;
    println!("Simple read:\n{}\n\n", websites);
    
    /* Linewise read */
    let file = fs::File::open("favorite_websites.txt")?;
    let lines = io::BufReader::new(file).lines();

    println!("Line wise read:");
    for line in lines {
        if let Ok(_line) = line {
            println!(">>> {}", _line);
        }
    }

    Ok(())
}