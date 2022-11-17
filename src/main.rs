use std::fs;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {

    /* Simple write */
    fs::write("favorite_websites.txt", b"opensource.com")?;

    /* Write using file descriptor */
    let mut file = fs::File::create("favorite_websites.txt")?;
    file.write_all(b"opensource.com\n")?;

    /* Write using file descriptor and opening modes */

    let mut file = fs::OpenOptions::new()
                                        //.write(true)
                                        .append(true)
                                        .open("favorite_websites.txt")?;

    file.write_all(b"sourceforge.net\n")?;
    Ok(())
}
