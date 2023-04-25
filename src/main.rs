use std::fs::File;
use std::io::{self, BufRead, BufReader};

use csv;

fn main() -> io::Result<()>{
    println!("Hello, world!");
    let file = File::open("data/14bus_buses.csv")?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
