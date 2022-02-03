use sha1::Digest;
use std::{
    fs::File,
    env,
    error::Error,
    io::{BufRead, BufReader}
};

const SHA_LEN: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Incorrect number of arguments".into());
    }

    let hash = args[2].trim();
    if hash.len() != SHA_LEN {
        return Err("Invalid hash length".into());
    }


    let file_name = args[1].trim();
    let reader = BufReader::new(File::open(file_name)?);
    for line in reader.lines() {
        let line = line?;
        let word = line.trim();
        let word_hash = hex::encode(sha1::Sha1::digest(word.as_bytes()));
        if hash == word_hash {
            println!("Found a match: '{}'", word);
            return Ok(());
        }
    }

    println!("No match :-(");
    Ok(())
}
